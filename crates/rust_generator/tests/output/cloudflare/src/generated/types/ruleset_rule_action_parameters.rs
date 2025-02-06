#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RulesetRuleActionParameters {
    /// Specifies uncommon ports to allow cacheable assets to be served from.
    #[builder(into, default)]
    #[serde(rename = "additionalCacheablePorts")]
    pub r#additional_cacheable_ports: Box<Option<Vec<i32>>>,
    /// Compression algorithms to use in order of preference.
    #[builder(into, default)]
    #[serde(rename = "algorithms")]
    pub r#algorithms: Box<Option<Vec<super::types::RulesetRuleActionParametersAlgorithm>>>,
    /// Turn on or off Cloudflare Automatic HTTPS rewrites.
    #[builder(into, default)]
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Box<Option<bool>>,
    /// Indicate which file extensions to minify automatically.
    #[builder(into, default)]
    #[serde(rename = "autominifies")]
    pub r#autominifies: Box<Option<Vec<super::types::RulesetRuleActionParametersAutominify>>>,
    /// Inspect the visitor's browser for headers commonly associated with spammers and certain bots.
    #[builder(into, default)]
    #[serde(rename = "bic")]
    pub r#bic: Box<Option<bool>>,
    /// List of browser TTL parameters to apply to the request.
    #[builder(into, default)]
    #[serde(rename = "browserTtl")]
    pub r#browser_ttl: Box<Option<super::types::RulesetRuleActionParametersBrowserTtl>>,
    /// Whether to cache if expression matches.
    #[builder(into, default)]
    #[serde(rename = "cache")]
    pub r#cache: Box<Option<bool>>,
    /// List of cache key parameters to apply to the request.
    #[builder(into, default)]
    #[serde(rename = "cacheKey")]
    pub r#cache_key: Box<Option<super::types::RulesetRuleActionParametersCacheKey>>,
    /// List of cache reserve parameters to apply to the request.
    #[builder(into, default)]
    #[serde(rename = "cacheReserve")]
    pub r#cache_reserve: Box<Option<super::types::RulesetRuleActionParametersCacheReserve>>,
    /// Content of the custom error response.
    #[builder(into, default)]
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// Content-Type of the custom error response.
    #[builder(into, default)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// List of cookie values to include as part of custom fields logging.
    #[builder(into, default)]
    #[serde(rename = "cookieFields")]
    pub r#cookie_fields: Box<Option<Vec<String>>>,
    /// Turn off all active Cloudflare Apps.
    #[builder(into, default)]
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Box<Option<bool>>,
    /// Turn off railgun feature of the Cloudflare Speed app.
    #[builder(into, default)]
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Box<Option<bool>>,
    /// Turn off RUM feature.
    #[builder(into, default)]
    #[serde(rename = "disableRum")]
    pub r#disable_rum: Box<Option<bool>>,
    /// Turn off zaraz feature.
    #[builder(into, default)]
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Box<Option<bool>>,
    /// List of edge TTL parameters to apply to the request.
    #[builder(into, default)]
    #[serde(rename = "edgeTtl")]
    pub r#edge_ttl: Box<Option<super::types::RulesetRuleActionParametersEdgeTtl>>,
    /// Turn on or off the Cloudflare Email Obfuscation feature of the Cloudflare Scrape Shield app.
    #[builder(into, default)]
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Box<Option<bool>>,
    /// Toggle fonts.
    #[builder(into, default)]
    #[serde(rename = "fonts")]
    pub r#fonts: Box<Option<bool>>,
    /// Use a list to lookup information for the action.
    #[builder(into, default)]
    #[serde(rename = "fromList")]
    pub r#from_list: Box<Option<super::types::RulesetRuleActionParametersFromList>>,
    /// Use a value to lookup information for the action.
    #[builder(into, default)]
    #[serde(rename = "fromValue")]
    pub r#from_value: Box<Option<super::types::RulesetRuleActionParametersFromValue>>,
    /// List of HTTP header modifications to perform in the ruleset rule. Note: Headers are order dependent and must be provided sorted alphabetically ascending based on the `name` value.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::types::RulesetRuleActionParametersHeader>>>,
    /// Host Header that request origin receives.
    #[builder(into, default)]
    #[serde(rename = "hostHeader")]
    pub r#host_header: Box<Option<String>>,
    /// Turn on or off the hotlink protection feature.
    #[builder(into, default)]
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Box<Option<bool>>,
    /// Identifier of the action parameter to modify.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "increment")]
    pub r#increment: Box<Option<i32>>,
    /// List of properties to configure WAF payload logging.
    #[builder(into, default)]
    #[serde(rename = "matchedData")]
    pub r#matched_data: Box<Option<super::types::RulesetRuleActionParametersMatchedData>>,
    /// Turn on or off Cloudflare Mirage of the Cloudflare Speed app.
    #[builder(into, default)]
    #[serde(rename = "mirage")]
    pub r#mirage: Box<Option<bool>>,
    /// Turn on or off the Cloudflare Opportunistic Encryption feature of the Edge Certificates tab in the Cloudflare SSL/TLS app.
    #[builder(into, default)]
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Box<Option<bool>>,
    /// List of properties to change request origin.
    #[builder(into, default)]
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<super::types::RulesetRuleActionParametersOrigin>>,
    /// Enable or disable the use of a more compliant Cache Control parsing mechanism, enabled by default for most zones.
    #[builder(into, default)]
    #[serde(rename = "originCacheControl")]
    pub r#origin_cache_control: Box<Option<bool>>,
    /// Pass-through error page for origin.
    #[builder(into, default)]
    #[serde(rename = "originErrorPagePassthru")]
    pub r#origin_error_page_passthru: Box<Option<bool>>,
    /// List of override configurations to apply to the ruleset.
    #[builder(into, default)]
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<super::types::RulesetRuleActionParametersOverrides>>,
    /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
    #[builder(into, default)]
    #[serde(rename = "phases")]
    pub r#phases: Box<Option<Vec<String>>>,
    /// Apply options from the Polish feature of the Cloudflare Speed app.
    #[builder(into, default)]
    #[serde(rename = "polish")]
    pub r#polish: Box<Option<String>>,
    /// Products to target with the actions. Available values: `bic`, `hot`, `ratelimit`, `securityLevel`, `uablock`, `waf`, `zonelockdown`.
    #[builder(into, default)]
    #[serde(rename = "products")]
    pub r#products: Box<Option<Vec<String>>>,
    /// Specifies a maximum timeout for reading content from an origin server.
    #[builder(into, default)]
    #[serde(rename = "readTimeout")]
    pub r#read_timeout: Box<Option<i32>>,
    /// List of request headers to include as part of custom fields logging, in lowercase.
    #[builder(into, default)]
    #[serde(rename = "requestFields")]
    pub r#request_fields: Box<Option<Vec<String>>>,
    /// Respect strong ETags.
    #[builder(into, default)]
    #[serde(rename = "respectStrongEtags")]
    pub r#respect_strong_etags: Box<Option<bool>>,
    /// List of response headers to include as part of custom fields logging, in lowercase.
    #[builder(into, default)]
    #[serde(rename = "responseFields")]
    pub r#response_fields: Box<Option<Vec<String>>>,
    /// List of parameters that configure the response given to end users.
    #[builder(into, default)]
    #[serde(rename = "responses")]
    pub r#responses: Box<Option<Vec<super::types::RulesetRuleActionParametersResponse>>>,
    /// Turn on or off Cloudflare Rocket Loader in the Cloudflare Speed app.
    #[builder(into, default)]
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Box<Option<bool>>,
    /// Map of managed WAF rule ID to comma-delimited string of ruleset rule IDs. Example: `rules = { "efb7b8c949ac4650a09736fc376e9aee" = "5de7edfa648c4d6891dc3e7f84534ffa,e3a567afc347477d9702d9047e97d760" }`.
    #[builder(into, default)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<std::collections::HashMap<String, String>>>,
    /// Which ruleset ID to target.
    #[builder(into, default)]
    #[serde(rename = "ruleset")]
    pub r#ruleset: Box<Option<String>>,
    /// List of managed WAF rule IDs to target. Only valid when the `"action"` is set to skip.
    #[builder(into, default)]
    #[serde(rename = "rulesets")]
    pub r#rulesets: Box<Option<Vec<String>>>,
    /// Control options for the Security Level feature from the Security app.
    #[builder(into, default)]
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    /// List of serve stale parameters to apply to the request.
    #[builder(into, default)]
    #[serde(rename = "serveStale")]
    pub r#serve_stale: Box<Option<super::types::RulesetRuleActionParametersServeStale>>,
    /// Turn on or off the Server Side Excludes feature of the Cloudflare Scrape Shield app.
    #[builder(into, default)]
    #[serde(rename = "serverSideExcludes")]
    pub r#server_side_excludes: Box<Option<bool>>,
    /// List of properties to manange Server Name Indication.
    #[builder(into, default)]
    #[serde(rename = "sni")]
    pub r#sni: Box<Option<super::types::RulesetRuleActionParametersSni>>,
    /// Control options for the SSL feature of the Edge Certificates tab in the Cloudflare SSL/TLS app.
    #[builder(into, default)]
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<String>>,
    /// HTTP status code of the custom error response.
    #[builder(into, default)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Turn on or off the SXG feature.
    #[builder(into, default)]
    #[serde(rename = "sxg")]
    pub r#sxg: Box<Option<bool>>,
    /// List of URI properties to configure for the ruleset rule when performing URL rewrite transformations.
    #[builder(into, default)]
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<super::types::RulesetRuleActionParametersUri>>,
    /// Version of the ruleset to deploy.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
