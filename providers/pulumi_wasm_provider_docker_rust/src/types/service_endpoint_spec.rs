#[derive(serde::Serialize)]
pub struct ServiceEndpointSpec {
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<crate::types::ServiceEndpointSpecPort>>>,
}
