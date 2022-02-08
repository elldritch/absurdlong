use crate::wordle::responses::*;
use crate::wordle::words::*;
use std::mem::{self, MaybeUninit};

mod wordle;

fn main() {
  println!("Hello, absurdle!");
  println!("Initial guess: {:?}", GUESS_WORDS[0]);

  let remaining_targets = [true; TARGET_WORDS_COUNT];

  for response in POSSIBLE_RESPONSES {
    println!("{:?}", response);
  }
}

struct RemainingWords {
  indexes: [bool; TARGET_WORDS_COUNT],
  count: usize,
}

// Given a guess and remaining uneliminated target words, what are the buckets
// of possible remaining target words for every response pattern?
fn buckets(
  guess: Word,
  remaining_targets: RemainingWords,
) -> [RemainingWords; POSSIBLE_RESPONSES_COUNT] {
  let mut uninit_ret: [MaybeUninit<RemainingWords>; POSSIBLE_RESPONSES_COUNT] =
    unsafe { MaybeUninit::uninit().assume_init() };

  // For every possible response pattern...
  for i in 0..POSSIBLE_RESPONSES_COUNT {
    let response = POSSIBLE_RESPONSES[i];
    let indexes = remaining_targets.indexes;
    let count = remaining_targets.count;

    // Check which target words might remain.
    for j in 0..TARGET_WORDS_COUNT {
      if !remaining_targets.indexes[j] {
        continue;
      }

      // A target word remains if it's compatible with the response pattern.
      let potential_target = TARGET_WORDS[j];
      let mut compatible = true;

      // TODO: No, it's probably easier to generate response patterns and then group, rather than trying to compute compatibility.
      for k in 0..5 {
        match response[k] {
          LetterResponse::Correct => {
            if potential_target[k] != guess[k] {
              compatible = false
            }
          }
          LetterResponse::IncorrectPosition => unimplemented!(),
          LetterResponse::IncorrectLetter => unimplemented!(),
        }
      }
    }
  }

  unsafe { mem::transmute::<_, [RemainingWords; POSSIBLE_RESPONSES_COUNT]>(uninit_ret) }
}
