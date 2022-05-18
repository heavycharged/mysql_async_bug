use actix_web::{App, Responder, web};
use mysql_async::{Pool};
use mysql_async_bug::AppState;
use actix_web::{HttpServer, get};
use anyhow::anyhow;
use log::{error, debug};
use thiserror::Error;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    // spawn some mysql instance in docker, e.g.:
    // docker run -it -d --name mysql_test -e MYSQL_ROOT_PASSWORD=root -e MYSQL_DATABASE=test -p 127.0.0.1:3306:3306 mysql

    let dsn = format!(
        "mysql://{}:{}@{}/{}",
        "root", "root", "127.0.0.1:3306", "test"
    );

    let app = AppState::new(Pool::from_url(
        &dsn
    )?);

    let data = web::Data::new(app);
    let data_clone = data.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(load_from_db)
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await;


    debug!("starting shutdown mysql");
    if let Err(e) = data_clone.into_inner().shutdown().await {
        error!("cannot shutdown mysql: {}", e);
    }
    debug!("finished shutdown mysql pool");

    Ok(())
}

#[get("/")]
async fn load_from_db(data: web::Data<AppState>) -> impl Responder {
    let number = data.select_number().await;

    match number {
        Ok(v) => v.to_string(),
        Err(e) => e.to_string(),
    }
}