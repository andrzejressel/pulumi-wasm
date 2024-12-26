#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum TreeSize {
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
}
