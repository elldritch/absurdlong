use crate::wordle::responses::LetterResponse::*;
use crate::wordle::responses::*;
use crate::wordle::words::*;
use std::collections::HashMap;

mod wordle;

fn main() {
  println!("Hello, absurdle!\n");
  println!("Initial guess: {}\n", "audio");

  // for response in POSSIBLE_RESPONSES {
  //   println!("{:?}", response);
  // }

  let buckets = response_buckets(['a', 'u', 'd', 'i', 'o'], TARGET_WORDS.to_vec());
  let mut sorted_buckets: Vec<(WordResponse, Vec<Word>)> = buckets.into_iter().collect();
  sorted_buckets.sort_by_key(|(_, words)| words.len());

  for (response, words) in sorted_buckets {
    println!(
      "{} ({} candidates)",
      response
        .iter()
        .map(|r| match r {
          Correct => "🟩",
          IncorrectPosition => "🟨",
          IncorrectLetter => "⬜",
        })
        .collect::<String>(),
      words.len()
    );
    let mut word_list = words
      .iter()
      .map(|word| word.iter().collect::<String>())
      .collect::<Vec<String>>();
    word_list.sort();
    println!("{}\n", word_list.join(" "));
  }

  // TODO: Next up, play a game until completion (picking first result every
  // time?). Then, define a sort order on patterns (convert into number using
  // reverse of possible_responses() generation). Then, explore tree with
  // pruning and checkpointing.
}

// Given a guess and remaining uneliminated target words, what are the buckets
// of possible remaining target words for every response pattern?
fn response_buckets(guess: Word, remaining_targets: Vec<Word>) -> HashMap<WordResponse, Vec<Word>> {
  let mut buckets: HashMap<WordResponse, Vec<Word>> = HashMap::new();

  // For every remaining word...
  for target in remaining_targets {
    // Calculate the word's response pattern.
    let response_pattern = response(guess, target);

    // Group words by response.
    if buckets.contains_key(&response_pattern) {
      let bucket = buckets.get_mut(&response_pattern).unwrap();
      bucket.push(target);
    } else {
      buckets.insert(response_pattern, vec![target]);
    }
  }

  buckets
}

fn response(guess: Word, target: Word) -> WordResponse {
  let mut unused_counts: HashMap<char, u8> = HashMap::new();

  // First, count up the character counts in the target word.
  for target_char in target {
    let current_count = unused_counts.get(&target_char).unwrap_or(&0);
    let next_count = current_count + 1;
    unused_counts.insert(target_char, next_count);
  }

  // By default, all response spaces are incorrect.
  let mut response: WordResponse = [IncorrectLetter; WORD_LENGTH];

  // Any matching characters are correct. Decrement a character usage.
  for i in 0..WORD_LENGTH {
    if guess[i] == target[i] {
      response[i] = Correct;
      let unused_count = unused_counts.get(&guess[i]).unwrap();
      let next_count = unused_count - 1;
      unused_counts.insert(guess[i], next_count);
    }
  }

  // Remaining incorrect characters can potentially be correct letters in wrong
  // positions. Decrement character usages from left to right.
  for i in 0..WORD_LENGTH {
    if response[i] == IncorrectLetter {
      if let Some(count) = unused_counts.get(&guess[i]) {
        if count >= &1 {
          response[i] = IncorrectPosition;
          let next_count = count - 1;
          unused_counts.insert(guess[i], next_count);
        }
      }
    }
  }

  response
}
