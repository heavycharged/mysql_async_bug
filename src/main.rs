use actix_web::{web::Data, App, get, HttpServer};
use mysql_async::{Pool, prelude::*};

#[actix_web::main]
async fn main() {
    let mysql_addr = std::env::var("MYSQL_ADDR").unwrap_or_else(|_| "mysql:3306".into());

    let dsn = format!("mysql://{}:{}@{}/{}", "root", "root", mysql_addr, "test");

    let pool = Pool::from_url(&dsn).unwrap();

    let pool2 = pool.clone();
    HttpServer::new(move || App::new().app_data(Data::new(pool.clone())).service(index))
        .bind(("0.0.0.0", 8080))
        .unwrap()
        .run()
        .await
        .unwrap();

    actix_web::rt::time::sleep(std::time::Duration::from_secs(3)).await;

    pool2.disconnect().await.unwrap()
}

#[get("/")]
async fn index(pool: Data<Pool>) -> String {
    let conn = pool.get_conn().await.unwrap();

    "SELECT 42"
        .first::<u32, _>(conn)
        .await
        .unwrap()
        .map(|i| i.to_string())
        .unwrap_or_else(|| "cannot query static number".into())
}