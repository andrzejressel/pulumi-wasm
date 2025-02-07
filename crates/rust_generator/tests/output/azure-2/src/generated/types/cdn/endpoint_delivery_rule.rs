#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointDeliveryRule {
    /// A `cache_expiration_action` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "cacheExpirationAction")]
    pub r#cache_expiration_action: Box<Option<super::super::types::cdn::EndpointDeliveryRuleCacheExpirationAction>>,
    /// A `cache_key_query_string_action` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "cacheKeyQueryStringAction")]
    pub r#cache_key_query_string_action: Box<Option<super::super::types::cdn::EndpointDeliveryRuleCacheKeyQueryStringAction>>,
    /// A `cookies_condition` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "cookiesConditions")]
    pub r#cookies_conditions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRuleCookiesCondition>>>,
    /// A `device_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "deviceCondition")]
    pub r#device_condition: Box<Option<super::super::types::cdn::EndpointDeliveryRuleDeviceCondition>>,
    /// A `http_version_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "httpVersionConditions")]
    pub r#http_version_conditions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRuleHttpVersionCondition>>>,
    /// A `modify_request_header_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "modifyRequestHeaderActions")]
    pub r#modify_request_header_actions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRuleModifyRequestHeaderAction>>>,
    /// A `modify_response_header_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "modifyResponseHeaderActions")]
    pub r#modify_response_header_actions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRuleModifyResponseHeaderAction>>>,
    /// The Name which should be used for this Delivery Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The order used for this rule. The order values should be sequential and begin at `1`.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Box<i32>,
    /// A `post_arg_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "postArgConditions")]
    pub r#post_arg_conditions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRulePostArgCondition>>>,
    /// A `query_string_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "queryStringConditions")]
    pub r#query_string_conditions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRuleQueryStringCondition>>>,
    /// A `remote_address_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "remoteAddressConditions")]
    pub r#remote_address_conditions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRuleRemoteAddressCondition>>>,
    /// A `request_body_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "requestBodyConditions")]
    pub r#request_body_conditions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRuleRequestBodyCondition>>>,
    /// A `request_header_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "requestHeaderConditions")]
    pub r#request_header_conditions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRuleRequestHeaderCondition>>>,
    /// A `request_method_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "requestMethodCondition")]
    pub r#request_method_condition: Box<Option<super::super::types::cdn::EndpointDeliveryRuleRequestMethodCondition>>,
    /// A `request_scheme_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "requestSchemeCondition")]
    pub r#request_scheme_condition: Box<Option<super::super::types::cdn::EndpointDeliveryRuleRequestSchemeCondition>>,
    /// A `request_uri_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "requestUriConditions")]
    pub r#request_uri_conditions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRuleRequestUriCondition>>>,
    /// A `url_file_extension_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "urlFileExtensionConditions")]
    pub r#url_file_extension_conditions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRuleUrlFileExtensionCondition>>>,
    /// A `url_file_name_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "urlFileNameConditions")]
    pub r#url_file_name_conditions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRuleUrlFileNameCondition>>>,
    /// A `url_path_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "urlPathConditions")]
    pub r#url_path_conditions: Box<Option<Vec<super::super::types::cdn::EndpointDeliveryRuleUrlPathCondition>>>,
    /// A `url_redirect_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "urlRedirectAction")]
    pub r#url_redirect_action: Box<Option<super::super::types::cdn::EndpointDeliveryRuleUrlRedirectAction>>,
    /// A `url_rewrite_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "urlRewriteAction")]
    pub r#url_rewrite_action: Box<Option<super::super::types::cdn::EndpointDeliveryRuleUrlRewriteAction>>,
}
