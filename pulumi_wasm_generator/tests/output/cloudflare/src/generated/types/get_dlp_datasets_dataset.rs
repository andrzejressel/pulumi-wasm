#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetDlpDatasetsDataset {
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Box<bool>,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
