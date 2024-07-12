use std::sync::Arc;

use super::json_response;
use sqlx::PgPool;
use warp::{reply::Json, Filter};

use crate::managers::wordle::WordleManager;

use super::with_db;

pub fn wordle_rest_filters(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let wordle_path = warp::path("api").and(warp::path("wordle"));
    let common = with_db(db.clone());

    let evaluate_guess = wordle_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(validate_guess);

    evaluate_guess
}

async fn validate_guess(
    db: Arc<PgPool>,
    guess: String,
    answer: String,
) -> Result<Json, warp::Rejection> {
    let wordle_manager = WordleManager;
    if !wordle_manager.validate_guess(&db, &guess).await {
        return json_response(vec![3, 3, 3, 3, 3]);
    }

    let response = wordle_manager.evaluate_guess(&guess, &answer).await;

    json_response(response)
}
