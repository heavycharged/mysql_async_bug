use std::time::Duration;
use actix_web::{App, Responder, web};
use mysql_async::{Pool};
use mysql_async_bug::AppState;
use actix_web::{HttpServer, get};
use log::{error, debug, warn};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    // spawn some mysql instance in docker, e.g.:
    // docker run -it -d --name mysql_test -e MYSQL_ROOT_PASSWORD=root -e MYSQL_DATABASE=test -p 127.0.0.1:3306:3306 mysql
    // dont forget to provide MYSQL_ADDR env variable with host:port pairs.

    debug!("starting application...");

    let mysql_addr = std::env::var("MYSQL_ADDR").unwrap_or_else(|_| "mysql:3306".to_string());

    let dsn = format!(
        "mysql://{}:{}@{}/{}",
        "root", "root", mysql_addr, "test"
    );

    let pool = Pool::from_url(&dsn)?;

    let app = AppState::new(pool);

    let data = web::Data::new(app);
    let data_clone = data.clone();

    debug!("starting http server...");
    let result = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(load_from_db)
    }).bind(("0.0.0.0", 8080))?
        .run()
        .await;

    if let Err(e) = result {
        error!("http server stopped with error: {}", e);
    }

    debug!("sleep for 3 secs...");
    tokio::time::sleep(Duration::from_secs(3)).await;
    debug!("sleep done");

    if let Err(e) = data_clone.into_inner().shutdown().await {
        error!("cannot shutdown mysql: {}", e);
    }
    debug!("WOOHOO! pool was shutdown finally (or you forgot to make at least one request)");

    Ok(())
}

#[get("/")]
async fn load_from_db(data: web::Data<AppState>) -> impl Responder {
    let number = data.select_number().await;

    match number {
        Ok(v) => v.to_string(),
        Err(e) => {
            warn!("error occurred: {}", e);
            e.to_string()
        }
    }
}