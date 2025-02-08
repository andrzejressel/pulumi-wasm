#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FrontdoorRoutingRuleForwardingConfiguration {
    /// Specifies the name of the Backend Pool to forward the incoming traffic to.
    #[builder(into)]
    #[serde(rename = "backendPoolName")]
    pub r#backend_pool_name: Box<String>,
    /// Specify the minimum caching duration (in ISO8601 notation e.g. `P1DT2H` for 1 day and 2 hours). Needs to be greater than 0 and smaller than 365 days. `cache_duration` works only in combination with `cache_enabled` set to `true`.
    #[builder(into, default)]
    #[serde(rename = "cacheDuration")]
    pub r#cache_duration: Box<Option<String>>,
    /// Specifies whether to Enable caching or not. Valid options are `true` or `false`. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "cacheEnabled")]
    pub r#cache_enabled: Box<Option<bool>>,
    /// Defines cache behaviour in relation to query string parameters. Valid options are `StripAll`, `StripAllExcept`, `StripOnly` or `StripNone`. Defaults to `StripAll`.
    #[builder(into, default)]
    #[serde(rename = "cacheQueryParameterStripDirective")]
    pub r#cache_query_parameter_strip_directive: Box<Option<String>>,
    /// Specify query parameters (array). Works only in combination with `cache_query_parameter_strip_directive` set to `StripAllExcept` or `StripOnly`.
    #[builder(into, default)]
    #[serde(rename = "cacheQueryParameters")]
    pub r#cache_query_parameters: Box<Option<Vec<String>>>,
    /// Whether to use dynamic compression when caching. Valid options are `true` or `false`. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "cacheUseDynamicCompression")]
    pub r#cache_use_dynamic_compression: Box<Option<bool>>,
    /// Path to use when constructing the request to forward to the backend. This functions as a URL Rewrite. Default behaviour preserves the URL path.
    #[builder(into, default)]
    #[serde(rename = "customForwardingPath")]
    pub r#custom_forwarding_path: Box<Option<String>>,
    /// Protocol to use when redirecting. Valid options are `HttpOnly`, `HttpsOnly`, or `MatchRequest`. Defaults to `HttpsOnly`.
    #[builder(into, default)]
    #[serde(rename = "forwardingProtocol")]
    pub r#forwarding_protocol: Box<Option<String>>,
}
