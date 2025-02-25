use std::collections::HashMap;

use crate::error::Error;
use sqlx::PgPool;

/// Function List
/// validate_guess: Checks if the guess is a valid word
/// daily_word: Sets the new daily word at midnight
/// get_daily_word: Retrieves the current daily word
///
pub struct WordleManager;

#[allow(unused)]
impl WordleManager {
    pub fn new() -> Self {
        Self
    }
    /// Checks if the guess is a valid word
    pub async fn validate_guess(&self, db: &PgPool, guess: &str) -> bool {
        match sqlx::query!(r#"SELECT * FROM valid_words WHERE word = $1"#, guess)
            .fetch_one(db)
            .await
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Sets the new daily word at midnight
    pub async fn daily_word(
        &self,
        db: &PgPool,
        cache_connection: &mut redis::Connection,
    ) -> Result<String, Error> {
        let daily_word = sqlx::query!(
            r#"SELECT * FROM word_bank WHERE last_used_on IS NULL ORDER BY random() limit 1"#
        )
        .fetch_one(db)
        .await
        .unwrap()
        .word;

        // println!("{daily_word}");
        match sqlx::query!(
            r#"UPDATE word_bank SET last_used_on = current_date::date WHERE word = $1"#,
            &daily_word
        )
        .execute(db)
        .await
        {
            Ok(_) => println!("Daily word updated!"),
            Err(_) => println!("Failed to update daily word"),
        };

        redis::cmd("SET")
            .arg("daily_word")
            .arg(&daily_word)
            .query::<String>(cache_connection)?;

        Ok(daily_word)
    }

    pub async fn get_daily_word(&self, cache_conn: &mut redis::Connection) -> String {
        let cache_word = redis::cmd("GET")
            .arg("daily_word")
            .query::<String>(cache_conn)
            .unwrap();

        cache_word
    }

    pub async fn play_previous_wordle(db: &PgPool) -> String {
        let new_word = sqlx::query!(
            r#"SELECT * FROM word_bank WHERE last_used_on IS NOT NULL ORDER BY random() limit 1"#
        )
        .fetch_one(db)
        .await
        .unwrap()
        .word;

        // println!("{new_word}");

        new_word
    }

    /// Assumes guess is a valid word. Compares guess with the correct word and returns a score vector
    pub async fn evaluate_guess(&self, correct_word: &str, guess: &str) -> Vec<i32> {
        if correct_word.len() != guess.len() && guess.len() != 5 {
            return vec![3, 3, 3, 3, 3];
        }
        let mut answer_map: HashMap<char, usize> = HashMap::new();
        let mut guess_map: HashMap<char, usize> = HashMap::new();
        let mut response = Vec::new();
        let mut incorrect_pos_map: HashMap<char, Vec<usize>> = HashMap::new();

        correct_word
            .trim()
            .to_lowercase()
            .chars()
            .for_each(|letter| {
                *answer_map.entry(letter).or_insert(0) += 1;
            });

        for i in 0..5 {
            let guess_char = guess.trim().to_lowercase().chars().nth(i).unwrap();
            let correct_char = correct_word.trim().to_lowercase().chars().nth(i).unwrap();

            let letter_count_diff: i32 = *guess_map.get(&guess_char).unwrap_or_else(|| &0) as i32
                - *answer_map.get(&guess_char).unwrap_or_else(|| &0) as i32;

            // EXACT MATCH
            if guess_char == correct_char {
                response.push(2);

                // ABCBA
                // CFCST
                if letter_count_diff == 0 {
                    response[incorrect_pos_map
                        .get_mut(&guess_char)
                        .unwrap()
                        .pop()
                        .unwrap()] = 0;
                } else {
                    *guess_map.entry(guess_char).or_insert(0) += 1;
                }
            } else if answer_map.contains_key(&guess_char) {
                if letter_count_diff < 0 {
                    *guess_map.entry(guess_char).or_insert(0) += 1;
                    incorrect_pos_map
                        .entry(guess_char)
                        .or_insert(vec![])
                        .push(i);
                    response.push(1);
                } else {
                    response.push(0);
                }
            } else {
                response.push(0);
            }
        }

        return response;
    }

    /// Counts the number of days the wordle has been updated
    pub async fn get_daily_wordle_count(&self, db: &PgPool) -> i64 {
        match sqlx::query!(r#"SELECT COUNT(*) FROM word_bank WHERE last_used_on IS NOT NULL"#,)
            .fetch_one(db)
            .await
        {
            Ok(count) => count.count.unwrap_or_else(|| 0) + 1,
            Err(err) => {
                println!("{err}");
                return 1;
            }
        }
    }
}

#[cfg(test)]
#[path = "../_tests/wordle_solver_algo.rs"]
mod tests;
