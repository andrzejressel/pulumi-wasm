#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthzPolicyHttpRuleToOperation {
    /// A list of headers to match against in http header.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headerSet")]
    pub r#header_set: Box<Option<super::super::types::networksecurity::AuthzPolicyHttpRuleToOperationHeaderSet>>,
    /// A list of HTTP Hosts to match against. The match can be one of exact, prefix, suffix, or contains (substring match). Matches are always case sensitive unless the ignoreCase is set.
    /// Limited to 5 matches.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "hosts")]
    pub r#hosts: Box<Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRuleToOperationHost>>>,
    /// A list of HTTP methods to match against. Each entry must be a valid HTTP method name (GET, PUT, POST, HEAD, PATCH, DELETE, OPTIONS). It only allows exact match and is always case sensitive.
    #[builder(into, default)]
    #[serde(rename = "methods")]
    pub r#methods: Box<Option<Vec<String>>>,
    /// A list of paths to match against. The match can be one of exact, prefix, suffix, or contains (substring match). Matches are always case sensitive unless the ignoreCase is set.
    /// Limited to 5 matches.
    /// Note that this path match includes the query parameters. For gRPC services, this should be a fully-qualified name of the form /package.service/method.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRuleToOperationPath>>>,
}
