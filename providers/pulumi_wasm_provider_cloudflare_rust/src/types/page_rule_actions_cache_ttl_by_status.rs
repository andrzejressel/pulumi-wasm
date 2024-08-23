#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheTtlByStatus {
    #[serde(rename = "codes")]
    pub r#codes: Box<String>,
    #[serde(rename = "ttl")]
    pub r#ttl: Box<i32>,
}
