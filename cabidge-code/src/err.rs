use std::{borrow::Borrow, hash::Hash, ops::Range};

use serde::{Deserialize, Serialize};

/// A range bounded by distances from the end of the source file.
/// This is used for error reporting.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Bounds {
    pub start: u32,
    pub end: u32,
}
impl From<Range<u32>> for Bounds {
    fn from(item: Range<u32>) -> Self {
        Bounds {
            start: item.start,
            end: item.end,
        }
    }
}
impl Hash for Bounds {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}
impl PartialEq for Bounds {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl Eq for Bounds {}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Bounded<T> {
    pub bounds: Bounds,
    pub item: T,
}
impl<T: PartialEq> PartialEq for Bounded<T> {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}
impl<T: PartialEq> Eq for Bounded<T> {}