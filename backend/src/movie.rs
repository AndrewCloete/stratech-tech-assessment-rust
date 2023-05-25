use tokio_postgres::{Error, GenericClient, Row};

#[derive(Debug, serde::Serialize)]
pub enum MovieType {
    Movie,
    Series,
}

#[derive(Debug, serde::Serialize)]
pub struct Movie {
    pub imdb_id: String, // Fine for demo to use IMDB ID as primary key
    pub title: String,
    pub year: i32,
    pub movie_type: MovieType,
    pub poster_url: String,
    pub updated_at: String,
}

impl From<Row> for Movie {
    fn from(row: Row) -> Self {
        Self {
            title: row.get(0),
            year: row.get(1),
            imdb_id: row.get(2),
            movie_type: match row.get(3) {
                "movie" => MovieType::Movie,
                "series" => MovieType::Series,
                _ => panic!("invalid movie type"),
            },
            poster_url: row.get(4),
            updated_at: row.get(5),
        }
    }
}

impl Movie {
    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<Movie>, Error> {
        let stmt = client
            .prepare("SELECT title, year, imdb_id, movie_type, poster_url, updated_at FROM movies")
            .await?;
        let rows = client.query(&stmt, &[]).await?;

        Ok(rows.into_iter().map(Movie::from).collect())
    }

    pub async fn upsert<C: GenericClient>(client: &C, movie: &Movie) -> Result<(), Error> {
        let stmt = client
            .prepare(
                "INSERT INTO movies (title, year, imdb_id, movie_type, poster_url, updated_at) \
                 VALUES ($1, $2, $3, $4, $5, $6) \
                 ON CONFLICT (imdb_id) DO UPDATE SET \
                 title = $1, year = $2, movie_type = $4, poster_url = $5, updated_at = $6",
            )
            .await?;
        client
            .execute(
                &stmt,
                &[
                    &movie.title,
                    &movie.year,
                    &movie.imdb_id,
                    &match movie.movie_type {
                        MovieType::Movie => "movie",
                        MovieType::Series => "series",
                    },
                    &movie.poster_url,
                ],
            )
            .await?;
        Ok(())
    }
}
