use std::{convert::Infallible, sync::Arc};

use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use tokio::sync::Mutex;
use warp::{reply::Json, Filter};
#[allow(dead_code)]

// use crate::Error;
pub fn with_db(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (Arc<PgPool>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn with_cache(
    cache: Arc<Mutex<redis::Connection>>,
) -> impl Filter<Extract = (Arc<Mutex<redis::Connection>>,), Error = Infallible> + Clone {
    warp::any().map(move || cache.clone())
}

// pub fn wordle_api_filters(
//     db: Arc<PgPool>,
// ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
//     let wordle_path = warp::path("api").and(warp::path("wordle"));
//     let common = with_db(db.clone());

//     wordle_path
//         .and(warp::get())
//         .and(warp::path::end())
//         .and(common.clone())
// }

#[allow(dead_code)]
fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({"data": data});
    Ok(warp::reply::json(&response))
}
