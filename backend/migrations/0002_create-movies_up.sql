CREATE TABLE movies (
  imdb_id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  year INTEGER NOT NULL,
  movie_type TEXT NOT NULL,
  poster_url TEXT NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);