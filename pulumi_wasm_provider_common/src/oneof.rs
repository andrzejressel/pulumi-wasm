use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum OneOf2<A: Serialize + Debug, B: Serialize + Debug> {
    Left(A),
    Right(B),
}

impl<A: Serialize + Debug, B: Serialize + Debug> OneOf2<A, B> {
    pub fn left(a: A) -> Self {
        OneOf2::Left(a)
    }
    pub fn right(b: B) -> Self {
        OneOf2::Right(b)
    }
}
