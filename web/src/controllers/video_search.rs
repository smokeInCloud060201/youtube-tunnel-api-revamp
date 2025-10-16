use actix_web::{get, web, HttpResponse};
use log::{debug};
use shared::model::video_search;

#[get("/v1/search")]
async fn get_video(query: web::Query<video_search::QueryParams>) -> actix_web::Result<HttpResponse> {

    let query_params = query.into_inner();

    debug!("Query: {:?}", query);

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("mock body")
    )
}