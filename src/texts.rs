mod nth_letter;

pub use nth_letter::NthLetterOfTextQ;

use crate::{ARc, SValue};

impl SValue {
    pub fn concat_no_limit(&self, other: &SValue) -> ARc<str> {
        self.concat(other, usize::MAX).unwrap_or_default()
    }

    /// Concatenates this value (as string) with another value (as string)
    pub fn concat(&self, other: &SValue, max_string_length: usize) -> Option<ARc<str>> {
        let my_text = self.as_text();
        let ot_text = other.as_text();
        if my_text.len() + ot_text.len() > max_string_length {
            return None;
        }

        let mut s = my_text.to_string();
        s += ot_text.as_ref();
        Some(s.into())
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
