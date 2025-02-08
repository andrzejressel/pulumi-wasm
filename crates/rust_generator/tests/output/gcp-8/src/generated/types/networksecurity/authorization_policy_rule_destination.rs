#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthorizationPolicyRuleDestination {
    /// List of host names to match. Matched against the ":authority" header in http requests. At least one host should match. Each host can be an exact match, or a prefix match (example "mydomain.*") or a suffix match (example "*.myorg.com") or a presence (any) match "*".
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Box<Vec<String>>,
    /// Match against key:value pair in http header. Provides a flexible match based on HTTP headers, for potentially advanced use cases. At least one header should match.
    /// Avoid using header matches to make authorization decisions unless there is a strong guarantee that requests arrive through a trusted client or proxy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "httpHeaderMatch")]
    pub r#http_header_match: Box<Option<super::super::types::networksecurity::AuthorizationPolicyRuleDestinationHttpHeaderMatch>>,
    /// A list of HTTP methods to match. At least one method should match. Should not be set for gRPC services.
    #[builder(into)]
    #[serde(rename = "methods")]
    pub r#methods: Box<Vec<String>>,
    /// List of destination ports to match. At least one port should match.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Vec<i32>>,
}
