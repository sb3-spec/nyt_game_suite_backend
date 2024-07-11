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
