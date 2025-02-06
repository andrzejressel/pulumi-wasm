#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobHttpTarget {
    /// HTTP request body.
    /// A request body is allowed only if the HTTP method is POST, PUT, or PATCH.
    /// It is an error to set body on a job with an incompatible HttpMethod.
    /// A base64-encoded string.
    #[builder(into, default)]
    #[serde(rename = "body")]
    pub r#body: Box<Option<String>>,
    /// This map contains the header field names and values.
    /// Repeated headers are not supported, but a header value can contain commas.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<std::collections::HashMap<String, String>>>,
    /// Which HTTP method to use for the request.
    #[builder(into, default)]
    #[serde(rename = "httpMethod")]
    pub r#http_method: Box<Option<String>>,
    /// Contains information needed for generating an OAuth token.
    /// This type of authorization should be used when sending requests to a GCP endpoint.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oauthToken")]
    pub r#oauth_token: Box<Option<super::super::types::cloudscheduler::JobHttpTargetOauthToken>>,
    /// Contains information needed for generating an OpenID Connect token.
    /// This type of authorization should be used when sending requests to third party endpoints or Cloud Run.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oidcToken")]
    pub r#oidc_token: Box<Option<super::super::types::cloudscheduler::JobHttpTargetOidcToken>>,
    /// The full URI path that the request will be sent to.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
