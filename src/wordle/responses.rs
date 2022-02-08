use self::LetterResponse::*;
use crate::WORD_LENGTH;
use std::fmt;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum LetterResponse {
  Correct,
  IncorrectPosition,
  IncorrectLetter,
}

impl fmt::Debug for LetterResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Correct => f.write_str("Y"),
      IncorrectPosition => f.write_str("M"),
      IncorrectLetter => f.write_str("N"),
    }
  }
}

pub type WordResponse = [LetterResponse; WORD_LENGTH];

pub const POSSIBLE_RESPONSES_COUNT: usize = usize::pow(3, WORD_LENGTH as u32);

pub const POSSIBLE_RESPONSES: [WordResponse; POSSIBLE_RESPONSES_COUNT] = possible_responses();

const fn possible_responses() -> [WordResponse; POSSIBLE_RESPONSES_COUNT] {
  let mut ret: [WordResponse; POSSIBLE_RESPONSES_COUNT] =
    [[IncorrectLetter; WORD_LENGTH]; POSSIBLE_RESPONSES_COUNT];
  let mut i = 0;
  while i < POSSIBLE_RESPONSES_COUNT {
    ret[i] = [
      usize_to_letter_response((i / usize::pow(3, 0)) % 3),
      usize_to_letter_response((i / usize::pow(3, 1)) % 3),
      usize_to_letter_response((i / usize::pow(3, 2)) % 3),
      usize_to_letter_response((i / usize::pow(3, 3)) % 3),
      usize_to_letter_response((i / usize::pow(3, 4)) % 3),
    ];
    i += 1;
  }
  ret
}

const fn usize_to_letter_response(n: usize) -> LetterResponse {
  match n {
    0 => IncorrectLetter,
    1 => IncorrectPosition,
    2 => Correct,
    _ => panic!("invalid cast to LetterResponse:"),
  }
}
