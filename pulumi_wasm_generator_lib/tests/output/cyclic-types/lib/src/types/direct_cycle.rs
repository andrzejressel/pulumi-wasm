#[derive(serde::Serialize)]
pub struct DirectCycle {
    #[serde(rename = "foo")]
    pub r#foo: Box<crate::types::DirectCycle>,
}
