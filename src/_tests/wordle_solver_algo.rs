use crate::WordleManager;

#[tokio::test]
async fn test_wordle_solver() -> Result<(), Box<dyn std::error::Error>> {
    let correct_results = vec!["02200", "01002", "02212", "33333"];
    let guesses = vec!["mulch", "nasty", "bbbAA", "bee"];
    let answers = vec!["Pulse", "CrazY", "ABBcA", "be"];

    let wordle_manager = WordleManager::new();

    for i in 0..3 {
        let result_as_str = String::from_iter(
            wordle_manager
                .evaluate_guess(answers[i], guesses[i])
                .await
                .iter()
                .map(|result| result.to_string()),
        );
        assert_eq!(&result_as_str, correct_results[i]);
    }

    Ok(())
}

// #[tokio::test]
// async fn test_set_daily_word() -> Result<(), Box<dyn std::error::Error>> {
//     dotenv::dotenv().ok();
//     let db_url = env::var("DATABASE_URL").unwrap();
//     let cache_url = env::var("REDIS_URL").unwrap();

//     let db_conn = connect_to_db(&db_url).await?;
//     let mut cache_conn = connect_to_cache(&cache_url).await?;

//     let wordle_manager = WordleManager::new();

//     let daily_word = wordle_manager.daily_word(&db_conn, &mut cache_conn).await?;

//     // println!("Daily word: {daily_word}");

//     Ok(())
// }
