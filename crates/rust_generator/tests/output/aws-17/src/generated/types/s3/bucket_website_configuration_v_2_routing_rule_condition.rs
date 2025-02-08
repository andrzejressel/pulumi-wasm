#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketWebsiteConfigurationV2RoutingRuleCondition {
    /// HTTP error code when the redirect is applied. If specified with `key_prefix_equals`, then both must be true for the redirect to be applied.
    #[builder(into, default)]
    #[serde(rename = "httpErrorCodeReturnedEquals")]
    pub r#http_error_code_returned_equals: Box<Option<String>>,
    /// Object key name prefix when the redirect is applied. If specified with `http_error_code_returned_equals`, then both must be true for the redirect to be applied.
    #[builder(into, default)]
    #[serde(rename = "keyPrefixEquals")]
    pub r#key_prefix_equals: Box<Option<String>>,
}
