#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PageRuleActions {
    /// Boolean of whether this action is enabled. Default: false.
    #[builder(into, default)]
    #[serde(rename = "alwaysUseHttps")]
    pub r#always_use_https: Box<Option<bool>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Box<Option<String>>,
    /// The Time To Live for the browser cache. `0` means 'Respect Existing Headers'
    #[builder(into, default)]
    #[serde(rename = "browserCacheTtl")]
    pub r#browser_cache_ttl: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "browserCheck")]
    pub r#browser_check: Box<Option<String>>,
    /// String value of cookie name to conditionally bypass cache the page.
    #[builder(into, default)]
    #[serde(rename = "bypassCacheOnCookie")]
    pub r#bypass_cache_on_cookie: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Box<Option<String>>,
    /// Controls how Cloudflare creates Cache Keys used to identify files in cache. See below for full description.
    #[builder(into, default)]
    #[serde(rename = "cacheKeyFields")]
    pub r#cache_key_fields: Box<Option<super::types::PageRuleActionsCacheKeyFields>>,
    /// Whether to set the cache level to `"bypass"`, `"basic"`, `"simplified"`, `"aggressive"`, or `"cache_everything"`.
    #[builder(into, default)]
    #[serde(rename = "cacheLevel")]
    pub r#cache_level: Box<Option<String>>,
    /// String value of cookie name to conditionally cache the page.
    #[builder(into, default)]
    #[serde(rename = "cacheOnCookie")]
    pub r#cache_on_cookie: Box<Option<String>>,
    /// Set cache TTL based on the response status from the origin web server. Can be specified multiple times. See below for full description.
    #[builder(into, default)]
    #[serde(rename = "cacheTtlByStatuses")]
    pub r#cache_ttl_by_statuses: Box<Option<Vec<super::types::PageRuleActionsCacheTtlByStatus>>>,
    /// Boolean of whether this action is enabled. Default: false.
    #[builder(into, default)]
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Box<Option<bool>>,
    /// Boolean of whether this action is enabled. Default: false.
    #[builder(into, default)]
    #[serde(rename = "disablePerformance")]
    pub r#disable_performance: Box<Option<bool>>,
    /// Boolean of whether this action is enabled. Default: false.
    #[builder(into, default)]
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Box<Option<bool>>,
    /// Boolean of whether this action is enabled. Default: false.
    #[builder(into, default)]
    #[serde(rename = "disableSecurity")]
    pub r#disable_security: Box<Option<bool>>,
    /// Boolean of whether this action is enabled. Default: false.
    #[builder(into, default)]
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Box<Option<bool>>,
    /// The Time To Live for the edge cache.
    #[builder(into, default)]
    #[serde(rename = "edgeCacheTtl")]
    pub r#edge_cache_ttl: Box<Option<i32>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Box<Option<String>>,
    /// Whether origin Cache-Control action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "explicitCacheControl")]
    pub r#explicit_cache_control: Box<Option<String>>,
    /// The URL to forward to, and with what status. See below.
    #[builder(into, default)]
    #[serde(rename = "forwardingUrl")]
    pub r#forwarding_url: Box<Option<super::types::PageRuleActionsForwardingUrl>>,
    /// Value of the Host header to send.
    #[builder(into, default)]
    #[serde(rename = "hostHeaderOverride")]
    pub r#host_header_override: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "ipGeolocation")]
    pub r#ip_geolocation: Box<Option<String>>,
    /// The configuration for HTML, CSS and JS minification. See below for full list of options.
    #[builder(into, default)]
    #[serde(rename = "minifies")]
    pub r#minifies: Box<Option<Vec<super::types::PageRuleActionsMinify>>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "mirage")]
    pub r#mirage: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "originErrorPagePassThru")]
    pub r#origin_error_page_pass_thru: Box<Option<String>>,
    /// Whether this action is `"off"`, `"lossless"` or `"lossy"`.
    #[builder(into, default)]
    #[serde(rename = "polish")]
    pub r#polish: Box<Option<String>>,
    /// Overridden origin server name.
    #[builder(into, default)]
    #[serde(rename = "resolveOverride")]
    pub r#resolve_override: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "respectStrongEtag")]
    pub r#respect_strong_etag: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "responseBuffering")]
    pub r#response_buffering: Box<Option<String>>,
    /// Whether to set the rocket loader to `"on"`, `"off"`.
    #[builder(into, default)]
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Box<Option<String>>,
    /// Whether to set the security level to `"off"`, `"essentially_off"`, `"low"`, `"medium"`, `"high"`, or `"under_attack"`.
    #[builder(into, default)]
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "serverSideExclude")]
    pub r#server_side_exclude: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "sortQueryStringForCache")]
    pub r#sort_query_string_for_cache: Box<Option<String>>,
    /// Whether to set the SSL mode to `"off"`, `"flexible"`, `"full"`, `"strict"`, or `"origin_pull"`.
    #[builder(into, default)]
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "trueClientIpHeader")]
    pub r#true_client_ip_header: Box<Option<String>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into, default)]
    #[serde(rename = "waf")]
    pub r#waf: Box<Option<String>>,
}
