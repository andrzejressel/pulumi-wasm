#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetDlpDatasetsDataset {
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "secret")]
    pub r#secret: Box<bool>,
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
