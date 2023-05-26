import React, { useEffect, useState } from "react";
import "./App.css";
import Button from "@mui/material/Button";
import TextField from "@mui/material/TextField";
import MovieService from "./services/movieService";

function App() {
  const [searchTerm, setSearchTerm] = useState();
  const [favoriteMovies, setFavoriteMovies] = useState([]);
  const [searchResults, setSearchResults] = useState([]);

  useEffect(() => {
    const movieService = new MovieService();
    movieService.getFavorites().then((res) => setFavoriteMovies(res));
  }, []);

  async function searchOMDB() {
    const movieService = new MovieService();
    const results = await movieService.searchOMDB(searchTerm);
    setSearchResults(results);
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
        <p>
          {searchResults?.map((m) => {
            return (
              <div id={m.imdbId}>
                <p>{m.Title}</p>
                <p>{m.Year}</p>
                <p>{m.Poster}</p>
              </div>
            );
          }) || "Loading..."}
        </p>
        <p>
          {favoriteMovies?.map((m) => {
            return (
              <div id={m.imdbId}>
                <p>{m.Title}</p>
                <p>{m.Year}</p>
                <p>{m.Poster}</p>
              </div>
            );
          }) || "Loading..."}
        </p>
      </header>
    </div>
  );
}

export default App;
