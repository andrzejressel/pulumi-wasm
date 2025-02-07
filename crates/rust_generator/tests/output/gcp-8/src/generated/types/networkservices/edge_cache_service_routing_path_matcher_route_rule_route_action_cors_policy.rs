#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCorsPolicy {
    /// In response to a preflight request, setting this to true indicates that the actual request can include user credentials.
    /// This translates to the Access-Control-Allow-Credentials response header.
    #[builder(into, default)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Box<Option<bool>>,
    /// Specifies the content for the Access-Control-Allow-Headers response header.
    #[builder(into, default)]
    #[serde(rename = "allowHeaders")]
    pub r#allow_headers: Box<Option<Vec<String>>>,
    /// Specifies the content for the Access-Control-Allow-Methods response header.
    #[builder(into, default)]
    #[serde(rename = "allowMethods")]
    pub r#allow_methods: Box<Option<Vec<String>>>,
    /// Specifies the list of origins that will be allowed to do CORS requests.
    /// This translates to the Access-Control-Allow-Origin response header.
    #[builder(into, default)]
    #[serde(rename = "allowOrigins")]
    pub r#allow_origins: Box<Option<Vec<String>>>,
    /// If true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect.
    #[builder(into, default)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    /// Specifies the content for the Access-Control-Allow-Headers response header.
    #[builder(into, default)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Box<Option<Vec<String>>>,
    /// Specifies how long results of a preflight request can be cached by a client in seconds. Note that many browser clients enforce a maximum TTL of 600s (10 minutes).
    /// - Setting the value to -1 forces a pre-flight check for all requests (not recommended)
    /// - A maximum TTL of 86400s can be set, but note that (as above) some clients may force pre-flight checks at a more regular interval.
    /// - This translates to the Access-Control-Max-Age header.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<String>,
}
