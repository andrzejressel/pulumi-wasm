#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParameters {
    /// Allows for the ability to support caching on non-standard ports.
    #[serde(rename = "additionalCacheablePorts")]
    pub r#additional_cacheable_ports: Box<Option<Vec<i32>>>,
    /// Turn on or off Cloudflare Automatic HTTPS rewrites.
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Box<Option<bool>>,
    /// Indicate which file extensions to minify automatically.
    #[serde(rename = "autominifies")]
    pub r#autominifies: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersAutominify>>>,
    /// Inspect the visitor's browser for headers commonly associated with spammers and certain bots.
    #[serde(rename = "bic")]
    pub r#bic: Box<Option<bool>>,
    /// List of browser TTL parameters to apply to the request.
    #[serde(rename = "browserTtl")]
    pub r#browser_ttl: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersBrowserTtl>>,
    /// Whether to cache if expression matches.
    #[serde(rename = "cache")]
    pub r#cache: Box<Option<bool>>,
    /// List of cache key parameters to apply to the request.
    #[serde(rename = "cacheKey")]
    pub r#cache_key: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersCacheKey>>,
    /// Content of the custom error response
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// Content-Type of the custom error response
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// List of cookie values to include as part of custom fields logging.
    #[serde(rename = "cookieFields")]
    pub r#cookie_fields: Box<Option<Vec<String>>>,
    /// Turn off all active Cloudflare Apps.
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Box<Option<bool>>,
    /// Turn off railgun feature of the Cloudflare Speed app.
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Box<Option<bool>>,
    /// Turn off zaraz feature.
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Box<Option<bool>>,
    /// List of edge TTL parameters to apply to the request.
    #[serde(rename = "edgeTtl")]
    pub r#edge_ttl: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersEdgeTtl>>,
    /// Turn on or off the Cloudflare Email Obfuscation feature of the Cloudflare Scrape Shield app.
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Box<Option<bool>>,
    /// Use a list to lookup information for the action.
    #[serde(rename = "fromList")]
    pub r#from_list: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersFromList>>,
    /// Use a value to lookup information for the action.
    #[serde(rename = "fromValue")]
    pub r#from_value: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersFromValue>>,
    /// List of HTTP header modifications to perform in the ruleset rule.
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersHeader>>>,
    /// Host Header that request origin receives.
    #[serde(rename = "hostHeader")]
    pub r#host_header: Box<Option<String>>,
    /// Turn on or off the hotlink protection feature.
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Box<Option<bool>>,
    /// The ID of the Ruleset to target.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "increment")]
    pub r#increment: Box<Option<i32>>,
    /// List of properties to configure WAF payload logging.
    #[serde(rename = "matchedData")]
    pub r#matched_data: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersMatchedData>>,
    /// Turn on or off Cloudflare Mirage of the Cloudflare Speed app.
    #[serde(rename = "mirage")]
    pub r#mirage: Box<Option<bool>>,
    /// Turn on or off the Cloudflare Opportunistic Encryption feature of the Edge Certificates tab in the Cloudflare SSL/TLS app.
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Box<Option<bool>>,
    /// List of properties to change request origin.
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersOrigin>>,
    /// Sets a more compliant mode for parsing Cache Control headers
    #[serde(rename = "originCacheControl")]
    pub r#origin_cache_control: Box<Option<bool>>,
    /// Pass-through error page for origin.
    #[serde(rename = "originErrorPagePassthru")]
    pub r#origin_error_page_passthru: Box<Option<bool>>,
    /// List of override configurations to apply to the ruleset.
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersOverrides>>,
    /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`
    #[serde(rename = "phases")]
    pub r#phases: Box<Option<Vec<String>>>,
    /// Apply options from the Polish feature of the Cloudflare Speed app.
    #[serde(rename = "polish")]
    pub r#polish: Box<Option<String>>,
    /// Products to target with the actions. Available values: `bic`, `hot`, `ratelimit`, `securityLevel`, `uablock`, `waf`, `zonelockdown`
    #[serde(rename = "products")]
    pub r#products: Box<Option<Vec<String>>>,
    /// Sets the timeout value for reading content from an origin server.
    #[serde(rename = "readTimeout")]
    pub r#read_timeout: Box<Option<i32>>,
    /// List of request headers to include as part of custom fields logging, in lowercase.
    #[serde(rename = "requestFields")]
    pub r#request_fields: Box<Option<Vec<String>>>,
    /// Respect strong ETags.
    #[serde(rename = "respectStrongEtags")]
    pub r#respect_strong_etags: Box<Option<bool>>,
    /// List of response headers to include as part of custom fields logging, in lowercase.
    #[serde(rename = "responseFields")]
    pub r#response_fields: Box<Option<Vec<String>>>,
    /// List of parameters that configure the response given to end users
    #[serde(rename = "responses")]
    pub r#responses: Box<Option<Vec<crate::types::GetRulesetsRulesetRuleActionParametersResponse>>>,
    /// Turn on or off Cloudflare Rocket Loader in the Cloudflare Speed app.
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Box<Option<bool>>,
    /// Map of managed WAF rule ID to comma-delimited string of ruleset rule IDs. Example: `rules = { "efb7b8c949ac4650a09736fc376e9aee" = "5de7edfa648c4d6891dc3e7f84534ffa,e3a567afc347477d9702d9047e97d760" }`
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<std::collections::HashMap<String, String>>>,
    /// Which ruleset ID to target.
    #[serde(rename = "ruleset")]
    pub r#ruleset: Box<Option<String>>,
    /// List of managed WAF rule IDs to target. Only valid when the `"action"` is set to skip
    #[serde(rename = "rulesets")]
    pub r#rulesets: Box<Option<Vec<String>>>,
    /// Control options for the Security Level feature from the Security app.
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    /// List of serve stale parameters to apply to the request.
    #[serde(rename = "serveStale")]
    pub r#serve_stale: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersServeStale>>,
    /// Turn on or off the Server Side Excludes feature of the Cloudflare Scrape Shield app.
    #[serde(rename = "serverSideExcludes")]
    pub r#server_side_excludes: Box<Option<bool>>,
    /// List of properties to manange Server Name Indication.
    #[serde(rename = "sni")]
    pub r#sni: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersSni>>,
    /// Control options for the SSL feature of the Edge Certificates tab in the Cloudflare SSL/TLS app.
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<String>>,
    /// HTTP status code of the custom error response
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Turn on or off the SXG feature.
    #[serde(rename = "sxg")]
    pub r#sxg: Box<Option<bool>>,
    /// List of URI properties to configure for the ruleset rule when performing URL rewrite transformations.
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<crate::types::GetRulesetsRulesetRuleActionParametersUri>>,
    /// Version of the ruleset to filter on.
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
