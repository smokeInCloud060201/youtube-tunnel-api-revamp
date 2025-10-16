use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware;
use actix_web::web;
use listenfd::ListenFd;
use std::env;
use tracing::info;

use crate::controllers::video_player::{get_video_playlist, request_video};
use crate::controllers::video_search::get_video;

pub async fn start() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    unsafe {
        env::set_var(
            "RUST_LOG",
            "web=info,web=debug,actix_web=debug,actix_server=info",
        );
    }

    tracing_subscriber::fmt::init();

    let host = env::var("HOST").expect("HOST not set in .env file");
    let port = env::var("PORT").expect("PORT not set in .env file");
    let server_url = format!("{host}:{port}");

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(init)
    });

    let mut listen_fd = ListenFd::from_env();
    server = if let Some(listener) = listen_fd.take_tcp_listener(0)? {
        server.listen(listener)?
    } else {
        server.bind(&server_url)?
    };

    info!("Starting server at {server_url}");
    server.run().await
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(request_video)
        .service(get_video)
        .service(get_video_playlist);
}
