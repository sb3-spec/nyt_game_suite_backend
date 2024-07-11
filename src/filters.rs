use std::{convert::Infallible, sync::Arc};

use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use warp::{reply::Json, Filter};

use crate::Error;

pub fn with_db(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (Arc<PgPool>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
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

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({"data": data});
    Ok(warp::reply::json(&response))
}
