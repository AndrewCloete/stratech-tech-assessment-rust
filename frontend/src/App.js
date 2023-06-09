import React, { useEffect, useState } from "react";
import "./App.css";

import { ThemeProvider, createTheme } from "@mui/material/styles";
import { CssBaseline } from "@mui/material";

import Button from "@mui/material/Button";
import TextField from "@mui/material/TextField";
import MovieService from "./services/movieService";
import { Star, Close } from "@mui/icons-material";

function MovieSearchResultRow(props) {
  const { movie, onStar } = props;
  return (
    <div className="movieRow movieHover">
      <div className="item">
        <Star
          className={!movie.isFavorite ? "star" : "starFavorite"}
          onClick={onStar(movie)}
        />
      </div>
      <div className="item">{movie.Year}</div>
      <div className="item">{movie.Title}</div>
    </div>
  );
}

function MovieSearchResults(props) {
  const { movies, onStar } = props;
  return (
    <div className="movieRows">
      {movies.map((m) => {
        return (
          <MovieSearchResultRow key={m.imdbID} movie={m} onStar={onStar} />
        );
      })}
    </div>
  );
}

function FavoriteMovies(props) {
  const { movies, onUnStar } = props;
  return (
    <div className="favoriteMovies">
      {movies.map((m) => {
        return (
          <FavoriteMovieCard key={m.imdbID} movie={m} onUnStar={onUnStar} />
        );
      })}
    </div>
  );
}

function FavoriteMovieCard(props) {
  const { movie, onUnStar } = props;
  return (
    <div className="movieCard movieHover">
      <div>{movie.Title}</div>
      <div>{movie.Year}</div>
      <div>
        <img className="moviePoster" src={movie.Poster} />
      </div>
      <Close className="unstar" onClick={onUnStar(movie.imdbID)} />
    </div>
  );
}

const darkTheme = createTheme({
  palette: {
    mode: "dark",
  },
});

function App() {
  const [searchTerm, setSearchTerm] = useState("");
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
    if (!searchTerm) {
      return;
    }
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
      // Remove favorite status to search results
      setSearchResults(
        searchResults.map((m) => {
          if (m.imdbID === imdbID) {
            m.isFavorite = false;
          }
          return m;
        })
      );
    };
  }

  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <div className="App">
        <header className="App-header">
          <div className="searchBar">
            <div className="item searchBox">
              <TextField
                id="searchTerm"
                label="Search movie by title"
                variant="outlined"
                onChange={(e) => setSearchTerm(e.target.value)}
                onKeyDown={(e) => {
                  if (e.keyCode === 13) {
                    searchOMDB();
                  }
                }}
                value={searchTerm}
                margin="dense"
              />
            </div>
            <div className="searchButtons">
              <div className="item">
                <Button
                  variant="contained"
                  onClick={searchOMDB}
                  className="something"
                >
                  Search
                </Button>
              </div>
              <div className="item">
                <Button
                  variant="outlined"
                  onClick={() => {
                    setSearchTerm("");
                    setSearchResults([]);
                  }}
                >
                  Clear
                </Button>
              </div>
            </div>
          </div>
          {<MovieSearchResults movies={searchResults} onStar={onStar} /> ||
            "Loading..."}
          {favoriteMovies.length !== 0 ? <h3>My Favorites</h3> : null}
          <FavoriteMovies movies={favoriteMovies} onUnStar={onUnStar} />
        </header>
      </div>
    </ThemeProvider>
  );
}

export default App;
