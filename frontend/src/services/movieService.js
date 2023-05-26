class MovieService {
  async searchOMDB(term) {
    return fetch(`/api/movies?title=${term}`).then((res) => res.json());
  }

  async getFavorites() {
    return fetch(`/api/movies`).then((res) => res.json());
  }

  async addFavorite(movie) {
    return fetch(`/api/movies`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(movie),
    }).then((res) => res.json());
  }
}

export default MovieService;
