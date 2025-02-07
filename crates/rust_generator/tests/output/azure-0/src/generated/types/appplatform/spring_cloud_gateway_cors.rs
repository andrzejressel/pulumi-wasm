#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudGatewayCors {
    /// Allowed headers in cross-site requests. The special value `*` allows actual requests to send any header.
    #[builder(into, default)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Box<Option<Vec<String>>>,
    /// Allowed HTTP methods on cross-site requests. The special value `*` allows all methods. If not set, `GET` and `HEAD` are allowed by default. Possible values are `DELETE`, `GET`, `HEAD`, `MERGE`, `POST`, `OPTIONS` and `PUT`.
    #[builder(into, default)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Box<Option<Vec<String>>>,
    /// Allowed origin patterns to make cross-site requests.
    #[builder(into, default)]
    #[serde(rename = "allowedOriginPatterns")]
    pub r#allowed_origin_patterns: Box<Option<Vec<String>>>,
    /// Allowed origins to make cross-site requests. The special value `*` allows all domains.
    #[builder(into, default)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Box<Option<Vec<String>>>,
    /// is user credentials are supported on cross-site requests?
    #[builder(into, default)]
    #[serde(rename = "credentialsAllowed")]
    pub r#credentials_allowed: Box<Option<bool>>,
    /// HTTP response headers to expose for cross-site requests.
    #[builder(into, default)]
    #[serde(rename = "exposedHeaders")]
    pub r#exposed_headers: Box<Option<Vec<String>>>,
    /// How long, in seconds, the response from a pre-flight request can be cached by clients.
    #[builder(into, default)]
    #[serde(rename = "maxAgeSeconds")]
    pub r#max_age_seconds: Box<Option<i32>>,
}
