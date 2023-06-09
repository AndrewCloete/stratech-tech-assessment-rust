use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer};
use deadpool_postgres::Pool;
use movie::Movie;

mod movie;
mod postgres;

// For a production application the DB connection would be fetched as part of
// the middleware This would remove the duplication in each handler.

#[get("/movies")]
async fn list_movies(
    pool: web::Data<Pool>,
    search: web::Query<movie::OmdbSearchParams>,
) -> HttpResponse {
    match &search.title {
        // If a search term is provided, search the OMDB API
        Some(title) => match movie::Movie::search(&title).await {
            Ok(list) => HttpResponse::Ok().json(list),
            Err(err) => {
                log::debug!("unable to fetch movies: {:?}", err);
                return HttpResponse::InternalServerError().json("unable to fetch movies");
            }
        },
        // else, fetch all movies from the database (i.e. the "favorites list")
        None => {
            let client = match pool.get().await {
                Ok(client) => client,
                Err(err) => {
                    log::debug!("unable to get postgres client: {:?}", err);
                    return HttpResponse::InternalServerError()
                        .json("unable to get postgres client");
                }
            };
            match movie::Movie::all(&**client).await {
                Ok(list) => HttpResponse::Ok().json(list),
                Err(err) => {
                    log::debug!("unable to fetch movies: {:?}", err);
                    return HttpResponse::InternalServerError().json("unable to fetch movies");
                }
            }
        }
    }
}

#[post("/movies")]
async fn upsert_movies(pool: web::Data<Pool>, movie_payload: web::Json<Movie>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match movie::Movie::upsert(&**client, &movie_payload).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {
            log::debug!("unable post movie {:?}, {:?}", &movie_payload, err);
            return HttpResponse::InternalServerError().json("unable to fetch movies");
        }
    }
}

#[delete("/movies/{imdb_id}")]
async fn delete_movie(pool: web::Data<Pool>, path: web::Path<String>) -> HttpResponse {
    let imdb_id = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match movie::Movie::delete(&**client, &imdb_id).await {
        Ok(_) => HttpResponse::Ok().json(()),
        Err(err) => {
            log::debug!("unable to delete movie: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to delete movie");
        }
    }
}

fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pg_pool = postgres::create_pool();
    postgres::migrate_up(&pg_pool).await;

    let address = address();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_pool.clone()))
            .service(list_movies)
            .service(upsert_movies)
            .service(delete_movie)
    })
    .bind(&address)?
    .run()
    .await
}
