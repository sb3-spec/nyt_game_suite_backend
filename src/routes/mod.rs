use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use std::{convert::Infallible, sync::Arc};
use warp::{reply::Json, Filter};
use wordle::wordle_rest_filters;

mod wordle;
use crate::error::Error;

pub async fn start_server(web_port: u16, db: Arc<PgPool>) -> Result<(), Error> {
    let cors = warp::cors()
        .allow_origins(["http://localhost:5173"])
        .allow_headers(vec!["X-Auth-Token", "Content-Type", "content-type"])
        .allow_methods(vec!["GET", "POST", "HEAD", "DELETE", "PATCH"]);

    let content = warp::fs::dir("web_folder/".to_string());

    let root_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("web_folder/index.html"));

    let static_site = root_index.or(content);

    let api = wordle_rest_filters(db.clone());

    let routes = static_site.or(api).with(cors);

    println!("Starting server on port {web_port}");

    warp::serve(routes).run(([0, 0, 0, 0], web_port)).await;

    Ok(())
}

pub fn with_db(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (Arc<PgPool>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!(data);
    Ok(warp::reply::json(&response))
}
