import React, { useEffect, useState } from "react";
import "./App.css";
import Button from "@mui/material/Button";

function App() {
  const [message, setMessage] = useState();
  useEffect(() => {
    fetch("/api/movies")
      .then((res) => res.json())
      .then((res) => setMessage(`Hello with ${res.length} movies`))
      .catch(console.error);
  }, [setMessage]);
  return (
    <div className="App">
      <header className="App-header">
        <p>{message || "Loading..."}</p>
        <Button variant="contained">Hello World</Button>
      </header>
    </div>
  );
}

export default App;
