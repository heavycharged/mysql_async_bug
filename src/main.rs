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

    debug!("starting application...");

    let dsn = format!(
        "mysql://{}:{}@{}/{}",
        "root", "root", "mysql:3306", "test"
    );

    let pool = loop {
        match Pool::from_url(&dsn) {
            Ok(pool) => break pool,
            Err(e) => {
                warn!("cannot connect to db... {:?}", e);
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    };
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

    debug!("starting shutdown of mysql pool...");
    if let Err(e) = data_clone.shutdown().await {
        error!("cannot shutdown pool: {}", e);
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
        },
    }
}