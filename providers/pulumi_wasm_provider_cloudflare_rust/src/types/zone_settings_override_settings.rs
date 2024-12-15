#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZoneSettingsOverrideSettings {
    #[builder(into, default)]
    #[serde(rename = "alwaysOnline")]
    pub r#always_online: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "alwaysUseHttps")]
    pub r#always_use_https: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "binaryAst")]
    pub r#binary_ast: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "brotli")]
    pub r#brotli: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "browserCacheTtl")]
    pub r#browser_cache_ttl: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "browserCheck")]
    pub r#browser_check: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "cacheLevel")]
    pub r#cache_level: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "challengeTtl")]
    pub r#challenge_ttl: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "ciphers")]
    pub r#ciphers: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "cnameFlattening")]
    pub r#cname_flattening: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "developmentMode")]
    pub r#development_mode: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "earlyHints")]
    pub r#early_hints: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "filterLogsToCloudflare")]
    pub r#filter_logs_to_cloudflare: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "fonts")]
    pub r#fonts: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "h2Prioritization")]
    pub r#h_2_prioritization: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "http2")]
    pub r#http_2: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "http3")]
    pub r#http_3: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "imageResizing")]
    pub r#image_resizing: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ipGeolocation")]
    pub r#ip_geolocation: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "logToCloudflare")]
    pub r#log_to_cloudflare: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "maxUpload")]
    pub r#max_upload: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "minify")]
    pub r#minify: Box<Option<crate::types::ZoneSettingsOverrideSettingsMinify>>,
    #[builder(into, default)]
    #[serde(rename = "mirage")]
    pub r#mirage: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "mobileRedirect")]
    pub r#mobile_redirect: Box<Option<crate::types::ZoneSettingsOverrideSettingsMobileRedirect>>,
    #[builder(into, default)]
    #[serde(rename = "nel")]
    pub r#nel: Box<Option<crate::types::ZoneSettingsOverrideSettingsNel>>,
    #[builder(into, default)]
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "opportunisticOnion")]
    pub r#opportunistic_onion: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "orangeToOrange")]
    pub r#orange_to_orange: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "originErrorPagePassThru")]
    pub r#origin_error_page_pass_thru: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "originMaxHttpVersion")]
    pub r#origin_max_http_version: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "polish")]
    pub r#polish: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "prefetchPreload")]
    pub r#prefetch_preload: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "privacyPass")]
    pub r#privacy_pass: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "proxyReadTimeout")]
    pub r#proxy_read_timeout: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "pseudoIpv4")]
    pub r#pseudo_ipv_4: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "replaceInsecureJs")]
    pub r#replace_insecure_js: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "responseBuffering")]
    pub r#response_buffering: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "securityHeader")]
    pub r#security_header: Box<Option<crate::types::ZoneSettingsOverrideSettingsSecurityHeader>>,
    #[builder(into, default)]
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "serverSideExclude")]
    pub r#server_side_exclude: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "sortQueryStringForCache")]
    pub r#sort_query_string_for_cache: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "speedBrain")]
    pub r#speed_brain: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "tls12Only")]
    pub r#tls_12_only: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "tls13")]
    pub r#tls_13: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "tlsClientAuth")]
    pub r#tls_client_auth: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "trueClientIpHeader")]
    pub r#true_client_ip_header: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "universalSsl")]
    pub r#universal_ssl: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "visitorIp")]
    pub r#visitor_ip: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "waf")]
    pub r#waf: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "webp")]
    pub r#webp: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "websockets")]
    pub r#websockets: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "zeroRtt")]
    pub r#zero_rtt: Box<Option<String>>,
}
