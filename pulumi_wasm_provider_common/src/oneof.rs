use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOf2<A: Serialize, B: Serialize> {
    Left(A),
    Right(B),
}

impl<A: Serialize, B: Serialize> OneOf2<A, B> {
    pub fn left(a: A) -> Self {
        OneOf2::Left(a)
    }
    pub fn right(b: B) -> Self {
        OneOf2::Right(b)
    }
}
