#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FunctionUrlCors {
    /// Whether to allow cookies or other credentials in requests to the function URL. The default is `false`.
    #[builder(into, default)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Box<Option<bool>>,
    /// The HTTP headers that origins can include in requests to the function URL. For example: `["date", "keep-alive", "x-custom-header"]`.
    #[builder(into, default)]
    #[serde(rename = "allowHeaders")]
    pub r#allow_headers: Box<Option<Vec<String>>>,
    /// The HTTP methods that are allowed when calling the function URL. For example: `["GET", "POST", "DELETE"]`, or the wildcard character (`["*"]`).
    #[builder(into, default)]
    #[serde(rename = "allowMethods")]
    pub r#allow_methods: Box<Option<Vec<String>>>,
    /// The origins that can access the function URL. You can list any number of specific origins (or the wildcard character (`"*"`)), separated by a comma. For example: `["https://www.example.com", "http://localhost:60905"]`.
    #[builder(into, default)]
    #[serde(rename = "allowOrigins")]
    pub r#allow_origins: Box<Option<Vec<String>>>,
    /// The HTTP headers in your function response that you want to expose to origins that call the function URL.
    #[builder(into, default)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Box<Option<Vec<String>>>,
    /// The maximum amount of time, in seconds, that web browsers can cache results of a preflight request. By default, this is set to `0`, which means that the browser doesn't cache results. The maximum value is `86400`.
    #[builder(into, default)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
}
