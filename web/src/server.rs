use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware;
use actix_web::web;
use listenfd::ListenFd;
use std::env;
use std::sync::Arc;
use tracing::info;
use service::video_search::VideoSearchService;

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
            .wrap(GlobalErrorHandler)
            .configure(init_app_data)
            .configure(init_config)
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

fn init_config(cfg: &mut web::ServiceConfig) {
    cfg.service(request_video)
        .service(get_video)
        .service(get_video_playlist);
}

fn init_app_data(app_data: &mut web::ServiceConfig) {
    let video_search = Arc::new(VideoSearchService::new());

    app_data.app_data(web::Data::new(video_search.clone()));
}

fn init_middle_error_handle() {

}

