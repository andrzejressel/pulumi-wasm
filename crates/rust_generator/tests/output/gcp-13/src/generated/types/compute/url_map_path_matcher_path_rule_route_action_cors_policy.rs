#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UrlMapPathMatcherPathRuleRouteActionCorsPolicy {
    /// In response to a preflight request, setting this to true indicates that the actual request can include user credentials.
    /// This translates to the Access-Control-Allow-Credentials header.
    #[builder(into, default)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Box<Option<bool>>,
    /// Specifies the content for the Access-Control-Allow-Headers header.
    #[builder(into, default)]
    #[serde(rename = "allowHeaders")]
    pub r#allow_headers: Box<Option<Vec<String>>>,
    /// Specifies the content for the Access-Control-Allow-Methods header.
    #[builder(into, default)]
    #[serde(rename = "allowMethods")]
    pub r#allow_methods: Box<Option<Vec<String>>>,
    /// Specifies the regular expression patterns that match allowed origins. For regular expression grammar
    /// please see en.cppreference.com/w/cpp/regex/ecmascript
    /// An origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes.
    #[builder(into, default)]
    #[serde(rename = "allowOriginRegexes")]
    pub r#allow_origin_regexes: Box<Option<Vec<String>>>,
    /// Specifies the list of origins that will be allowed to do CORS requests.
    /// An origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes.
    #[builder(into, default)]
    #[serde(rename = "allowOrigins")]
    pub r#allow_origins: Box<Option<Vec<String>>>,
    /// If true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<bool>,
    /// Specifies the content for the Access-Control-Expose-Headers header.
    #[builder(into, default)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Box<Option<Vec<String>>>,
    /// Specifies how long results of a preflight request can be cached in seconds.
    /// This translates to the Access-Control-Max-Age header.
    #[builder(into, default)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
}
