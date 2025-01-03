#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HttpRouteRuleActionRedirect {
    /// The host that will be used in the redirect response instead of the one that was supplied in the request.
    #[builder(into, default)]
    #[serde(rename = "hostRedirect")]
    pub r#host_redirect: Box<Option<String>>,
    /// If set to true, the URL scheme in the redirected request is set to https.
    #[builder(into, default)]
    #[serde(rename = "httpsRedirect")]
    pub r#https_redirect: Box<Option<bool>>,
    /// The path that will be used in the redirect response instead of the one that was supplied in the request. pathRedirect can not be supplied together with prefixRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect.
    #[builder(into, default)]
    #[serde(rename = "pathRedirect")]
    pub r#path_redirect: Box<Option<String>>,
    /// The port that will be used in the redirected request instead of the one that was supplied in the request.
    #[builder(into, default)]
    #[serde(rename = "portRedirect")]
    pub r#port_redirect: Box<Option<i32>>,
    /// Indicates that during redirection, the matched prefix (or path) should be swapped with this value.
    #[builder(into, default)]
    #[serde(rename = "prefixRewrite")]
    pub r#prefix_rewrite: Box<Option<String>>,
    /// The HTTP Status code to use for the redirect.
    #[builder(into, default)]
    #[serde(rename = "responseCode")]
    pub r#response_code: Box<Option<String>>,
    /// If set to true, any accompanying query portion of the original URL is removed prior to redirecting the request.
    #[builder(into, default)]
    #[serde(rename = "stripQuery")]
    pub r#strip_query: Box<Option<bool>>,
}
