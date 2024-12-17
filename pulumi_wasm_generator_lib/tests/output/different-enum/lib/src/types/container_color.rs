//! plant container colors

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum ContainerColor {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "yellow")]
    Yellow,
}
