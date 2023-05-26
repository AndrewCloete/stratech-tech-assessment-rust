import React, { useEffect, useState } from "react";
import "./App.css";
import Button from "@mui/material/Button";
import TextField from "@mui/material/TextField";
import MovieService from "./services/movieService";

function MovieSearchResultRow(props) {
  const { movie, onStar } = props;
  return (
    <div className="movieRow">
      <p>{movie.Title}</p>
      <p>{movie.Year}</p>
      <Button variant="contained" onClick={onStar(movie)}>
        Star!
      </Button>
    </div>
  );
}

function MovieSearchResults(props) {
  const { movies, onStar } = props;
  return (
    <div className="movieRows">
      {movies.map((m) => {
        return <MovieSearchResultRow id={m.imdbId} movie={m} onStar={onStar} />;
      })}
    </div>
  );
}

function FavoriteMovieCard(props) {
  const { movie, onUnStar } = props;
  return (
    <div className="movieCard">
      <p>{movie.Title}</p>
      <p>{movie.Year}</p>
      <div>
        <img className="moviePoster" src={movie.Poster} />
      </div>
      <Button variant="contained" onClick={onUnStar(movie.imdbID)}>
        Unstar!
      </Button>
    </div>
  );
}

function App() {
  const [searchTerm, setSearchTerm] = useState();
  const [favoriteMovies, setFavoriteMovies] = useState([]);
  const [searchResults, setSearchResults] = useState([]);

  useEffect(() => {
    refreshFavorites();
  }, []);

  async function refreshFavorites() {
    const movieService = new MovieService();
    const favorites = await movieService.getFavorites();
    setFavoriteMovies(favorites);
  }

  async function searchOMDB() {
    const movieService = new MovieService();
    const results = await movieService.searchOMDB(searchTerm);

    // Add favorite status to search results
    results.forEach((movie) => {
      if (favoriteMovies.some((m) => m.imdbID === movie.imdbID)) {
        movie.isFavorite = true;
      } else {
        movie.isFavorite = false;
      }
    });
    setSearchResults(results);
  }

  function onStar(movie) {
    return async function () {
      if (favoriteMovies.some((m) => m.imdbID === movie.imdbID)) {
        return;
      }
      // Add favorite status to search results
      setSearchResults(
        searchResults.map((m) => {
          if (m.imdbID === movie.imdbID) {
            m.isFavorite = true;
          }
          return m;
        })
      );
      const movieService = new MovieService();
      await movieService.addFavorite(movie);
      // Explicitly deciding against optimistic local state update here for
      // simplicity.
      await refreshFavorites();
    };
  }

  function onUnStar(imdbID) {
    return async function () {
      const movieService = new MovieService();
      await movieService.removeFavorite(imdbID);
      // Again, no optimistic local state update here.
      await refreshFavorites();
    };
  }

  return (
    <div className="App">
      <header className="App-header">
        <TextField
          id="searchTerm"
          label="Search by Title"
          variant="outlined"
          onChange={(e) => setSearchTerm(e.target.value)}
          value={searchTerm}
        />
        <Button variant="contained" onClick={searchOMDB}>
          Search
        </Button>
        {<MovieSearchResults movies={searchResults} onStar={onStar} /> ||
          "Loading..."}
        <p>
          {favoriteMovies?.map((m) => {
            return (
              <FavoriteMovieCard id={m.imdbId} movie={m} onUnStar={onUnStar} />
            );
          }) || "Loading..."}
        </p>
      </header>
    </div>
  );
}

export default App;
