#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetBucketCor {
    /// The value, in seconds, to return in the Access-Control-Max-Age header used in preflight responses.
    #[builder(into)]
    #[serde(rename = "maxAgeSeconds")]
    pub r#max_age_seconds: Box<i32>,
    /// The list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: "*" is permitted in the list of methods, and means "any method".
    #[builder(into)]
    #[serde(rename = "methods")]
    pub r#methods: Box<Vec<String>>,
    /// The list of Origins eligible to receive CORS response headers. Note: "*" is permitted in the list of origins, and means "any Origin".
    #[builder(into)]
    #[serde(rename = "origins")]
    pub r#origins: Box<Vec<String>>,
    /// The list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains.
    #[builder(into)]
    #[serde(rename = "responseHeaders")]
    pub r#response_headers: Box<Vec<String>>,
}
