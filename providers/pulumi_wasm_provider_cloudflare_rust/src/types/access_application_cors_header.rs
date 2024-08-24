#[derive(serde::Serialize)]
pub struct AccessApplicationCorsHeader {
    /// Value to determine whether all HTTP headers are exposed.
    #[serde(rename = "allowAllHeaders")]
    pub r#allow_all_headers: Box<Option<bool>>,
    /// Value to determine whether all methods are exposed.
    #[serde(rename = "allowAllMethods")]
    pub r#allow_all_methods: Box<Option<bool>>,
    /// Value to determine whether all origins are permitted to make CORS requests.
    #[serde(rename = "allowAllOrigins")]
    pub r#allow_all_origins: Box<Option<bool>>,
    /// Value to determine if credentials (cookies, authorization headers, or TLS client certificates) are included with requests.
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Box<Option<bool>>,
    /// List of HTTP headers to expose via CORS.
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Box<Option<Vec<String>>>,
    /// List of methods to expose via CORS.
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Box<Option<Vec<String>>>,
    /// List of origins permitted to make CORS requests.
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Box<Option<Vec<String>>>,
    /// The maximum time a preflight request will be cached.
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
}
