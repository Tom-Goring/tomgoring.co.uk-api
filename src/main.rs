#[macro_use]
extern crate log;
extern crate env_logger;

use actix_web::{web, App, HttpRequest, HttpServer, Responder, middleware::Logger};
use listenfd::ListenFd;
use std::env;
use sqlx::PgPool;
use anyhow::Result;

mod todo;

async fn index(_req: HttpRequest) -> impl Responder {
    "Hello!"
}

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::new(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .configure(todo::init)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("0.0.0.0:3000")?
    };

    info!("Starting server");
    server.run().await?;

    Ok(())
}
