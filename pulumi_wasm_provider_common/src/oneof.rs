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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOf3<A: Serialize + Debug, B: Serialize + Debug, C: Serialize + Debug> {
    Left(A),
    Middle(B),
    Right(C),
}

impl<A: Serialize + Debug, B: Serialize + Debug, C: Serialize + Debug> OneOf3<A, B, C> {
    pub fn left(a: A) -> Self {
        OneOf3::Left(a)
    }
    pub fn middle(b: B) -> Self {
        OneOf3::Middle(b)
    }
    pub fn right(c: C) -> Self {
        OneOf3::Right(c)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOf4<A: Serialize + Debug, B: Serialize + Debug, C: Serialize + Debug, D: Serialize + Debug> {
    Left(A),
    Middle1(B),
    Middle2(C),
    Right(D),
}

impl<A: Serialize + Debug, B: Serialize + Debug, C: Serialize + Debug, D: Serialize + Debug> OneOf4<A, B, C, D> {
    pub fn left(a: A) -> Self {
        OneOf4::Left(a)
    }
    pub fn middle1(b: B) -> Self {
        OneOf4::Middle1(b)
    }
    pub fn middle2(c: C) -> Self {
        OneOf4::Middle2(c)
    }
    pub fn right(d: D) -> Self {
        OneOf4::Right(d)
    }
}
