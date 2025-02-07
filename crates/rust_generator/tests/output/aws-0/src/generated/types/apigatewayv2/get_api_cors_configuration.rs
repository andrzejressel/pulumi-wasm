#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApiCorsConfiguration {
    /// Whether credentials are included in the CORS request.
    #[builder(into)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Box<bool>,
    /// Set of allowed HTTP headers.
    #[builder(into)]
    #[serde(rename = "allowHeaders")]
    pub r#allow_headers: Box<Vec<String>>,
    /// Set of allowed HTTP methods.
    #[builder(into)]
    #[serde(rename = "allowMethods")]
    pub r#allow_methods: Box<Vec<String>>,
    /// Set of allowed origins.
    #[builder(into)]
    #[serde(rename = "allowOrigins")]
    pub r#allow_origins: Box<Vec<String>>,
    /// Set of exposed HTTP headers.
    #[builder(into)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Box<Vec<String>>,
    /// Number of seconds that the browser should cache preflight request results.
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<i32>,
}
