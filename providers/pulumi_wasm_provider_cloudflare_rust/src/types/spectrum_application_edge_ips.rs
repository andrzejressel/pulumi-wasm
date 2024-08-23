#[derive(serde::Serialize)]
pub struct SpectrumApplicationEdgeIps {
    #[serde(rename = "connectivity")]
    pub r#connectivity: Box<Option<String>>,
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
