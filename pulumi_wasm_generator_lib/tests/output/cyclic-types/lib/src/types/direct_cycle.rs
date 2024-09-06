#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DirectCycle {
    #[serde(rename = "foo")]
    pub r#foo: Box<crate::types::DirectCycle>,
}
