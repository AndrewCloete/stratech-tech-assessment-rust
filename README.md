# Declaration
I declare that this is my own work and that all sources that I have used are
indicated and acknowledged by means of references. I have not consulted any
prior attempts at this exercise. Github Copilot was enabled during the
development of this project. 

# Todo
- Test on M1. State if it does not work.

# Dependencies
- `docker`
- `docker-compose` >= 2.x.x


# Running
Create the .env file and add your OMDb API key
```bash
cp env.example .env
vim .env
```

In the project root run
```bash
docker-compose up -d
```
NB: On first time deploy the DB needs to initialise before the backend can
connect to it. The backend container will fail. Monitor the DB logs until it is
ready to accept connections then restart the service
```bash
docker-compose logs db -f # Follow the logs to inspect 
docker-compose restart backend # Restart the backend
```

# References
https://github.com/docker/awesome-compose/tree/master/react-rust-postgres
