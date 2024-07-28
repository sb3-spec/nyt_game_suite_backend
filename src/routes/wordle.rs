use std::sync::Arc;

use super::json_response;
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::Mutex;
use warp::{reply::Json, Filter};

use crate::{filters::with_cache, managers::wordle::WordleManager};

use super::with_db;

/// Function that serves as a REST API endpoint hub for Wordle
pub fn wordle_rest_filters(
    db: Arc<PgPool>,
    cache: Arc<Mutex<redis::Connection>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let wordle_path = warp::path("api").and(warp::path("wordle"));
    let common = with_db(db.clone());
    let cache_filter = with_cache(cache.clone());

    let evaluate_guess = wordle_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(validate_guess);

    let daily_wordle_count = wordle_path
        .and(warp::get())
        .and(warp::path("count"))
        .and(common.clone())
        .and(warp::path::end())
        .and_then(get_daily_wordle_count);

    let daily_wordle = wordle_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and(cache_filter.clone())
        .and_then(get_daily_wordle);

    evaluate_guess.or(daily_wordle_count).or(daily_wordle)
}

/// Warp route for validating a guess against the correct word.
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

/// Warp route for getting the daily wordle word
pub async fn get_daily_wordle(
    db: Arc<PgPool>,
    cache: Arc<Mutex<redis::Connection>>,
) -> Result<Json, warp::Rejection> {
    let wordle_manager = WordleManager;
    let word = wordle_manager
        .get_daily_word(&mut *cache.lock().await)
        .await;

    let wordle_count = wordle_manager.get_daily_wordle_count(&db).await;

    #[derive(Serialize)]
    #[allow(non_snake_case)]
    struct DailyWordleResponse {
        dailyWord: String,
        wordleCount: i64,
    }

    let data = DailyWordleResponse {
        dailyWord: word,
        wordleCount: wordle_count,
    };
    json_response(data)
}

/// Warp route for getting the number of daily wordle words that have been played
pub async fn get_daily_wordle_count(db: Arc<PgPool>) -> Result<Json, warp::Rejection> {
    let wordle_manager = WordleManager;
    let count = wordle_manager.get_daily_wordle_count(&db).await;

    json_response(count)
}
