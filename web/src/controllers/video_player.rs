use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::HttpResponse;
use tracing::info;

#[post("/v1/video-player")]
async fn request_video() -> actix_web::Result<HttpResponse> {
    info!("Got a /v1/video-player request");

    let response = "this is mock body return";

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response))
}

#[get("/v1/video-player/{job_id}/playlist")]
async fn get_video_playlist(job_id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    info!("Got a /v1/video-playlist request {job_id}");

    let response = "Mock playlist";

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
    )
}



