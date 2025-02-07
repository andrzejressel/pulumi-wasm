#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorRoutingRuleRedirectConfiguration {
    /// The destination fragment in the portion of URL after '#'. Set this to add a fragment to the redirect URL.
    #[builder(into, default)]
    #[serde(rename = "customFragment")]
    pub r#custom_fragment: Box<Option<String>>,
    /// Set this to change the URL for the redirection.
    #[builder(into, default)]
    #[serde(rename = "customHost")]
    pub r#custom_host: Box<Option<String>>,
    /// The path to retain as per the incoming request, or update in the URL for the redirection.
    #[builder(into, default)]
    #[serde(rename = "customPath")]
    pub r#custom_path: Box<Option<String>>,
    /// Replace any existing query string from the incoming request URL.
    #[builder(into, default)]
    #[serde(rename = "customQueryString")]
    pub r#custom_query_string: Box<Option<String>>,
    /// Protocol to use when redirecting. Valid options are `HttpOnly`, `HttpsOnly`, or `MatchRequest`.
    #[builder(into)]
    #[serde(rename = "redirectProtocol")]
    pub r#redirect_protocol: Box<String>,
    /// Status code for the redirect. Valida options are `Moved`, `Found`, `TemporaryRedirect`, `PermanentRedirect`.
    #[builder(into)]
    #[serde(rename = "redirectType")]
    pub r#redirect_type: Box<String>,
}
