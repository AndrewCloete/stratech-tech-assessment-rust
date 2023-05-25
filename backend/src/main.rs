use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use deadpool_postgres::Pool;
use movie::Movie;

mod movie;
mod postgres;
mod user;

#[get("/users")]
async fn list_users(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match user::User::all(&**client).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {
            log::debug!("unable to fetch users: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch users");
        }
    }
}

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
            .service(list_users)
            .service(list_movies)
            .service(upsert_movies)
    })
    .bind(&address)?
    .run()
    .await
}
