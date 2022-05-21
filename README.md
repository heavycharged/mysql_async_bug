# How to reproduce

## The problem

This is simple app that handles HTTP request on `/` route, 
and queries `SELECT 42` from database. When you hit 
`Ctrl+C`, it starts graceful shutdown, but it cannot 
disconnect from MySQL: `Pool::disconnect` hangs forever.

## Easiest way

1. `docker-compose up --build`
2. Make some requests http://localhost:8080/
3. Hit `Ctrl+C`
4. See it blocks forever

_Note: `app` service in `docker-compose.yml` has 
`stop_grace_period: 30m`, it is intentionally, otherwise
docker will force-kill container after 10 secs_ 

## More robust way

**Disclaimer**: when you hit `Ctrl+C` when running this 
app via `docker-compose`, you will not see any logs 
after it. This is useful information, so if you want
to see it, run this app on your host machine. Also, 
it rules out a possibility networking issues between 
two docker containers.

1. Spawn docker mysql container:
   ```docker run -it -d --name mysql_test -e MYSQL_ROOT_PASSWORD=root -e MYSQL_DATABASE=test -p 127.0.0.1:3306:3306 mysql```
2. `cargo run --release`
3. Make some requests http://localhost:8080/
4. Press `Ctrl+C` to stop application, see it blocks forever

