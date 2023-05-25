use reqwest;
use tokio_postgres::{self, GenericClient, Row};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum MovieType {
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "series")]
    Series,
    #[serde(rename = "episode")]
    Episode,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Movie {
    #[serde(rename = "imdbID")]
    pub imdb_id: String, // Fine for demo to use IMDB ID as primary key
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "Type")]
    pub movie_type: MovieType,
    #[serde(rename = "Poster")]
    pub poster_url: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OmdbSearchParams {
    pub title: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct OmdbSearchResult {
    #[serde(rename = "Search")]
    search: Option<Vec<Movie>>,
    #[serde(rename = "Response")]
    response: String,
    #[serde(rename = "Error")]
    error: Option<String>,
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
            updated_at: Some(row.get(5)),
        }
    }
}

impl Movie {
    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<Movie>, tokio_postgres::Error> {
        let stmt = client
            .prepare("SELECT title, year, imdb_id, movie_type, poster_url, updated_at FROM movies")
            .await?;
        let rows = client.query(&stmt, &[]).await?;

        Ok(rows.into_iter().map(Movie::from).collect())
    }

    pub async fn upsert<C: GenericClient>(
        client: &C,
        movie: &Movie,
    ) -> Result<(), tokio_postgres::Error> {
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
                        MovieType::Episode => "episode",
                    },
                    &movie.poster_url,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn search(term: &str) -> Result<Vec<Movie>, reqwest::Error> {
        let url = format!(
            "http://www.omdbapi.com/?apikey={}&s={}",
            std::env::var("OMDB_API_KEY").unwrap(),
            term
        );
        println!("{:?}", url);
        let response = reqwest::get(&url).await?.text().await?;
        println!("{:?}", response);
        let result: OmdbSearchResult = serde_json::from_str(&response).unwrap();
        println!("{:?}", result);
        match result.response.as_str() {
            "True" => Ok(result.search.unwrap()),
            "False" => Ok(Vec::new()),
            _ => panic!("invalid response"),
        }
    }
}
