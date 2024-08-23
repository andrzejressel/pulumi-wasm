#[derive(serde::Serialize)]
pub struct RateLimitMatchRequest {
    #[serde(rename = "methods")]
    pub r#methods: Box<Option<Vec<String>>>,
    #[serde(rename = "schemes")]
    pub r#schemes: Box<Option<Vec<String>>>,
    #[serde(rename = "urlPattern")]
    pub r#url_pattern: Box<Option<String>>,
}
