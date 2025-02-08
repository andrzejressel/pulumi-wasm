#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorRuleActionsRouteConfigurationOverrideAction {
    /// `HonorOrigin` the Front Door will always honor origin response header directive. If the origin directive is missing, Front Door will cache contents anywhere from `1` to `3` days. `OverrideAlways` the TTL value returned from your Front Door Origin is overwritten with the value specified in the action. This behavior will only be applied if the response is cacheable. `OverrideIfOriginMissing` if no TTL value gets returned from your Front Door Origin, the rule sets the TTL to the value specified in the action. This behavior will only be applied if the response is cacheable. `Disabled` the Front Door will not cache the response contents, irrespective of Front Door Origin response directives. Possible values include `HonorOrigin`, `OverrideAlways`, `OverrideIfOriginMissing` or `Disabled`.
    #[builder(into, default)]
    #[serde(rename = "cacheBehavior")]
    pub r#cache_behavior: Box<Option<String>>,
    /// When Cache behavior is set to `Override` or `SetIfMissing`, this field specifies the cache duration to use. The maximum duration is 366 days specified in the `d.HH:MM:SS` format(e.g. `365.23:59:59`). If the desired maximum cache duration is less than 1 day then the maximum cache duration should be specified in the `HH:MM:SS` format(e.g. `23:59:59`).
    #[builder(into, default)]
    #[serde(rename = "cacheDuration")]
    pub r#cache_duration: Box<Option<String>>,
    /// The Front Door Origin Group resource ID that the request should be routed to. This overrides the configuration specified in the Front Door Endpoint route.
    #[builder(into, default)]
    #[serde(rename = "cdnFrontdoorOriginGroupId")]
    pub r#cdn_frontdoor_origin_group_id: Box<Option<String>>,
    /// Should the Front Door dynamically compress the content? Possible values include `true` or `false`.
    /// 
    /// ->**NOTE:** Content won't be compressed on AzureFrontDoor when requested content is smaller than `1 byte` or larger than `1 MB`.
    #[builder(into, default)]
    #[serde(rename = "compressionEnabled")]
    pub r#compression_enabled: Box<Option<bool>>,
    /// The forwarding protocol the request will be redirected as. This overrides the configuration specified in the route to be associated with. Possible values include `MatchRequest`, `HttpOnly` or `HttpsOnly`.
    /// 
    /// ->**NOTE:** If the `cdn_frontdoor_origin_group_id` is not defined you cannot set the `forwarding_protocol`.
    #[builder(into, default)]
    #[serde(rename = "forwardingProtocol")]
    pub r#forwarding_protocol: Box<Option<String>>,
    /// `IncludeSpecifiedQueryStrings` query strings specified in the `query_string_parameters` field get included when the cache key gets generated. `UseQueryString` cache every unique URL, each unique URL will have its own cache key. `IgnoreSpecifiedQueryStrings` query strings specified in the `query_string_parameters` field get excluded when the cache key gets generated. `IgnoreQueryString` query strings aren't considered when the cache key gets generated. Possible values include `IgnoreQueryString`, `UseQueryString`, `IgnoreSpecifiedQueryStrings` or `IncludeSpecifiedQueryStrings`.
    #[builder(into, default)]
    #[serde(rename = "queryStringCachingBehavior")]
    pub r#query_string_caching_behavior: Box<Option<String>>,
    /// A list of query string parameter names.
    /// 
    /// ->**NOTE:** `query_string_parameters` is a required field when the `query_string_caching_behavior` is set to `IncludeSpecifiedQueryStrings` or `IgnoreSpecifiedQueryStrings`.
    #[builder(into, default)]
    #[serde(rename = "queryStringParameters")]
    pub r#query_string_parameters: Box<Option<Vec<String>>>,
}
