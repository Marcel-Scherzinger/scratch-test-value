mod nth_letter;

pub use nth_letter::NthLetterOfTextQ;

use crate::{ARc, SValue};

impl SValue {
    /// Concatenates this value (as string) with another value (as string)
    pub fn concat(&self, other: &SValue) -> ARc<str> {
        let mut s = self.as_text().to_string();
        s += other.as_text().as_ref();
        s.into()
    }

    /// Does this text contain another text?
    // TODO: more efficient implementation
    pub fn contains_text(&self, maybe_contained: &SValue) -> bool {
        self.as_text()
            .to_lowercase()
            .contains(&maybe_contained.as_text().to_lowercase())
    }

    /// The number of characters in the textual representation of a value
    pub fn textual_length(&self) -> usize {
        self.as_text().chars().count()
    }
}
