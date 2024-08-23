#[derive(serde::Serialize)]
pub struct HealthcheckHeader {
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
