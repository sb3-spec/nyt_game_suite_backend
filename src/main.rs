use chrono::{NaiveTime, Timelike};
use managers::wordle::WordleManager;
use routes::start_server;
use sqlx::PgPool;
use std::{env, sync::Arc, time::Duration};

use tokio::sync::Mutex;

mod error;
mod filters;
mod managers;
mod routes;

use error::Error;

#[allow(dead_code)]

const DEFAULT_API_PORT: u16 = 8080;
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let redis_url = env::var("REDIS_URL").unwrap();

    // println!("DATABASE_URL: {db_url}");

    let db = Arc::new(connect_to_db(&db_url).await.unwrap());
    let db_2: Arc<sqlx::Pool<sqlx::Postgres>> = Arc::clone(&db);
    let cache = Arc::new(Mutex::new(connect_to_cache(&redis_url).await.unwrap()));
    let cache_2 = Arc::clone(&cache);

    let web_port = match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => DEFAULT_API_PORT,
    };

    let _thread_handle = tokio::spawn(async move {
        match start_server(web_port, db_2, cache_2).await {
            Ok(_) => println!("Server started successfully!"),
            Err(e) => println!("Server failed to start: {e}"),
        }
    });

    let mut pst_current: NaiveTime;
    let wordle_manager = WordleManager::new();

    loop {
        pst_current = chrono::Utc::now().with_timezone(&chrono_tz::PST8PDT).time();
        if pst_current.num_seconds_from_midnight() == 0 {
            match wordle_manager
                .daily_word(&db.clone(), &mut *cache.lock().await)
                .await
            {
                Ok(_) => println!("Daily wordle updated!"),
                Err(e) => println!("Daily wordle update failed: {}", e),
            }
            std::thread::sleep(Duration::from_secs(120));
        }
    }
}

pub async fn connect_to_db(db_url: &str) -> Result<PgPool, Error> {
    // println!("Connecting to database...");
    let pool = PgPool::connect(db_url).await?;
    // println!("Connected to database!");

    // sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

pub async fn connect_to_cache(cache_url: &str) -> Result<redis::Connection, Error> {
    // Function to connect to Redis cache
    // println!("Attempting to connect to cache...");
    let client = redis::Client::open(cache_url)?;
    let con = client.get_connection()?;
    // println!("Connection successfull");

    Ok(con)
}
