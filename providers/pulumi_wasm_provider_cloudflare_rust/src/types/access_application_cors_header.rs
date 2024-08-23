#[derive(serde::Serialize)]
pub struct AccessApplicationCorsHeader {
    #[serde(rename = "allowAllHeaders")]
    pub r#allow_all_headers: Box<Option<bool>>,
    #[serde(rename = "allowAllMethods")]
    pub r#allow_all_methods: Box<Option<bool>>,
    #[serde(rename = "allowAllOrigins")]
    pub r#allow_all_origins: Box<Option<bool>>,
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Box<Option<bool>>,
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Box<Option<Vec<String>>>,
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Box<Option<Vec<String>>>,
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Box<Option<Vec<String>>>,
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
}
