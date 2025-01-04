#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceCorsConfiguration {
    /// (Boolean) If credentials are allowed via CORS.
    #[builder(into, default)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Box<Option<bool>>,
    /// A set of headers to be allowed via CORS.
    #[builder(into, default)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Box<Option<Vec<String>>>,
    /// The methods to be allowed via CORS. Possible values are `DELETE`, `GET`, `HEAD`, `MERGE`, `POST`, `OPTIONS`, `PATCH` and `PUT`.
    #[builder(into, default)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Box<Option<Vec<String>>>,
    /// A set of origins to be allowed via CORS.
    #[builder(into, default)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Box<Option<Vec<String>>>,
    /// The max age to be allowed via CORS.
    #[builder(into, default)]
    #[serde(rename = "maxAgeInSeconds")]
    pub r#max_age_in_seconds: Box<Option<i32>>,
}
