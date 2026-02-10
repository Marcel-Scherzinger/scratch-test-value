use itertools::Itertools;

use crate::{
    QuirkSink, SValue,
    quirks::{NumberTooBigForIntQ, SValueToNumberQ},
};

#[derive(Debug, Clone)]
pub struct SList {
    data: Vec<SValue>,
    max_length: i64,
}

#[allow(clippy::enum_variant_names)]
pub enum NthItemOfListQ {
    PositionIsNotInt(SValue),
    PositionZero,
    PositionNegative(i64),
    PositionTooBig(i64),
}

#[derive(Debug, thiserror::Error)]
#[error("List already reached maximum length, element insertion failed")]
pub struct SListAlreadyFull(());

pub enum Indexing {
    /// Zero or below or more than one more than the current length
    Invalid,
    /// Valid zero-based index in the current state of the list data
    Normal(usize),
    /// Exactly one after the last element, important for insertions
    OneMore,
}

#[derive(Debug, thiserror::Error)]
#[error("The maximum length can't be smaller than the number of items in the list")]
pub struct ListLongerThanNewMaximum;

impl SList {
    pub fn new(max_length: i64) -> Self {
        Self {
            data: vec![],
            max_length,
        }
    }

    pub fn delete_all(&mut self) {
        self.data.clear();
    }
    pub fn length(&self) -> i64 {
        self.data.len() as i64
    }
    pub fn textual_representation(&self) -> String {
        self.data.iter().join(" ")
    }
    pub fn contains_item(&self, item: &SValue) -> bool {
        self.data.iter().any(|x| x.q_eq(item, &mut ()))
    }

    fn index_for_value<Q>(&self, one_based_n: &SValue, sink: &mut Q, is_insert: bool) -> Indexing
    where
        Q: QuirkSink<NthItemOfListQ> + QuirkSink<SValueToNumberQ> + QuirkSink<NumberTooBigForIntQ>,
    {
        let index = one_based_n.floor(sink).int_or_border(sink);

        if !one_based_n.is_int() {
            sink.put(NthItemOfListQ::PositionIsNotInt(one_based_n.clone()));
            Indexing::Invalid
        } else if index < 0 {
            sink.put(NthItemOfListQ::PositionNegative(index));
            Indexing::Invalid
        } else if index == 0 {
            sink.put(NthItemOfListQ::PositionZero);
            Indexing::Invalid
        } else if index as usize <= self.data.len() {
            let zero_based = (index - 1) as usize;
            Indexing::Normal(zero_based)
        } else if index as usize + 1 == self.data.len() {
            if !is_insert {
                sink.put(NthItemOfListQ::PositionTooBig(index));
            }
            Indexing::OneMore
        } else {
            sink.put(NthItemOfListQ::PositionTooBig(index));
            Indexing::Invalid
        }
    }
    /// Deletes the item at the position `one_based_n` if
    /// `one_based_n` can be converted into an integer in
    /// range `[1; self.length()]`.
    ///
    /// Returns the removed item if it was removed:
    /// - `Some(item)`: `one_based_n` referred to a valid position, item removed
    /// - `None`: `one_based_n` was invalid (0, too big, ...), nothing changed
    pub fn delete_nth<Q>(&mut self, one_based_n: &SValue, sink: &mut Q) -> Option<SValue>
    where
        Q: QuirkSink<NthItemOfListQ> + QuirkSink<SValueToNumberQ> + QuirkSink<NumberTooBigForIntQ>,
    {
        match self.index_for_value(one_based_n, sink, false) {
            Indexing::Normal(zero_based) => Some(self.data.remove(zero_based)),
            Indexing::Invalid | Indexing::OneMore => None,
        }
    }

    /// Appends an item to the end of the list
    pub fn append_item(&mut self, item: SValue) -> Result<(), SListAlreadyFull> {
        if self.length() < self.max_length {
            self.data.push(item);
            Ok(())
        } else {
            Err(SListAlreadyFull(()))
        }
    }

    /// Inserts the `item` at the position `one_based_n` if
    /// `one_based_n` can be converted into an integer in
    /// range `[1; self.length() + 1]`.
    ///
    /// One more than length will be identical to appending.
    ///
    /// Returns if the list was changed (`bool`).
    /// - `true`: `one_based_n` referred to a valid position, item inserted
    /// - `false`: `one_based_n` was invalid (0, too big, ...), nothing changed
    pub fn insert_item_at<Q>(
        &mut self,
        one_based_n: &SValue,
        item: SValue,
        sink: &mut Q,
    ) -> Result<bool, SListAlreadyFull>
    where
        Q: QuirkSink<NthItemOfListQ> + QuirkSink<SValueToNumberQ> + QuirkSink<NumberTooBigForIntQ>,
    {
        if self.length() >= self.max_length {
            return Err(SListAlreadyFull(()));
        }
        match self.index_for_value(one_based_n, sink, true) {
            Indexing::Normal(i) => {
                self.data.insert(i, item);
                Ok(true)
            }
            Indexing::OneMore => {
                self.data.push(item);
                Ok(true)
            }
            Indexing::Invalid => Ok(false),
        }
    }

    /// Replaces the n-th item with `new_val` if `one_based_n`
    /// can be converted into an integer in range `[1; self.length()]`
    pub fn replace_nth_item<Q>(
        &mut self,
        one_based_n: &SValue,
        new_val: SValue,
        sink: &mut Q,
    ) -> bool
    where
        Q: QuirkSink<NthItemOfListQ> + QuirkSink<SValueToNumberQ> + QuirkSink<NumberTooBigForIntQ>,
    {
        match self.index_for_value(one_based_n, sink, false) {
            Indexing::Invalid | Indexing::OneMore => false,
            Indexing::Normal(i) => {
                self.data[i] = new_val;
                true
            }
        }
    }

    pub fn nth_item<Q>(&self, one_based_n: &SValue, sink: &mut Q) -> SValue
    where
        Q: QuirkSink<NthItemOfListQ> + QuirkSink<SValueToNumberQ> + QuirkSink<NumberTooBigForIntQ>,
    {
        // TODO change to borrowing when "" can be created in const context
        match self.index_for_value(one_based_n, sink, false) {
            Indexing::Normal(i) => self.data[i].clone(),
            Indexing::OneMore | Indexing::Invalid => SValue::Text("".into()),
        }
    }

    pub fn first_index_of_item_in_list(&self, item: &SValue) -> i64 {
        self.data
            .iter()
            .find_position(|x| x.q_eq(item, &mut ()))
            .map(|(pos, _)| pos as i64)
            .unwrap_or(0)
    }

    pub fn set_max_length(&mut self, mut max: i64) -> Result<(), ListLongerThanNewMaximum> {
        max = max.max(0);
        if max as usize > self.data.len() {
            Err(ListLongerThanNewMaximum)
        } else {
            self.max_length = max;
            Ok(())
        }
    }

    pub fn max_length(&self) -> i64 {
        self.max_length
    }
    pub fn data(&self) -> &[SValue] {
        &self.data
    }

    pub fn make_readonly(self) -> ReadonlySList {
        ReadonlySList {
            data: self.data.into(),
            max_length: self.max_length,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ReadonlySList {
    data: crate::ARc<[SValue]>,
    max_length: i64,
}
impl ReadonlySList {
    pub fn owned_data(&self) -> crate::ARc<[SValue]> {
        self.data.clone()
    }
    pub fn data(&self) -> &crate::ARc<[SValue]> {
        &self.data
    }
    pub fn max_length(&self) -> i64 {
        self.max_length
    }
}
