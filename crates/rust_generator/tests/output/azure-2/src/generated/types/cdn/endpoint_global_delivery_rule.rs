#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EndpointGlobalDeliveryRule {
    /// A `cache_expiration_action` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "cacheExpirationAction")]
    pub r#cache_expiration_action: Box<Option<super::super::types::cdn::EndpointGlobalDeliveryRuleCacheExpirationAction>>,
    /// A `cache_key_query_string_action` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "cacheKeyQueryStringAction")]
    pub r#cache_key_query_string_action: Box<Option<super::super::types::cdn::EndpointGlobalDeliveryRuleCacheKeyQueryStringAction>>,
    /// A `modify_request_header_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "modifyRequestHeaderActions")]
    pub r#modify_request_header_actions: Box<Option<Vec<super::super::types::cdn::EndpointGlobalDeliveryRuleModifyRequestHeaderAction>>>,
    /// A `modify_response_header_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "modifyResponseHeaderActions")]
    pub r#modify_response_header_actions: Box<Option<Vec<super::super::types::cdn::EndpointGlobalDeliveryRuleModifyResponseHeaderAction>>>,
    /// A `url_redirect_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "urlRedirectAction")]
    pub r#url_redirect_action: Box<Option<super::super::types::cdn::EndpointGlobalDeliveryRuleUrlRedirectAction>>,
    /// A `url_rewrite_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "urlRewriteAction")]
    pub r#url_rewrite_action: Box<Option<super::super::types::cdn::EndpointGlobalDeliveryRuleUrlRewriteAction>>,
}
