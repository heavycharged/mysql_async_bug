# How to reproduce

1. Spawn docker mysql container:
   ```docker run -it -d --name mysql_test -e MYSQL_ROOT_PASSWORD=root -e MYSQL_DATABASE=test -p 127.0.0.1:3306:3306 mysql```
2. `cargo run --release`
3. Make some requests: `http://localhost:8080/`
4. Press `Ctrl+C` to stop application, see it blocks forever