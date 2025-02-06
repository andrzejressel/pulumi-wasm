#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UrlMapPathMatcherPathRuleUrlRedirect {
    /// The host that will be used in the redirect response instead of the one that was
    /// supplied in the request. The value must be between 1 and 255 characters.
    #[builder(into, default)]
    #[serde(rename = "hostRedirect")]
    pub r#host_redirect: Box<Option<String>>,
    /// If set to true, the URL scheme in the redirected request is set to https. If set
    /// to false, the URL scheme of the redirected request will remain the same as that
    /// of the request. This must only be set for UrlMaps used in TargetHttpProxys.
    /// Setting this true for TargetHttpsProxy is not permitted. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "httpsRedirect")]
    pub r#https_redirect: Box<Option<bool>>,
    /// The path that will be used in the redirect response instead of the one that was
    /// supplied in the request. Only one of pathRedirect or prefixRedirect must be
    /// specified. The value must be between 1 and 1024 characters.
    #[builder(into, default)]
    #[serde(rename = "pathRedirect")]
    pub r#path_redirect: Box<Option<String>>,
    /// The prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,
    /// retaining the remaining portion of the URL before redirecting the request.
    #[builder(into, default)]
    #[serde(rename = "prefixRedirect")]
    pub r#prefix_redirect: Box<Option<String>>,
    /// The HTTP Status code to use for this RedirectAction. Supported values are:
    /// * MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.
    /// * FOUND, which corresponds to 302.
    /// * SEE_OTHER which corresponds to 303.
    /// * TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method will be retained.
    /// * PERMANENT_REDIRECT, which corresponds to 308. In this case, the request method will be retained.
    #[builder(into, default)]
    #[serde(rename = "redirectResponseCode")]
    pub r#redirect_response_code: Box<Option<String>>,
    /// If set to true, any accompanying query portion of the original URL is removed
    /// prior to redirecting the request. If set to false, the query portion of the
    /// original URL is retained. Defaults to false.
    #[builder(into)]
    #[serde(rename = "stripQuery")]
    pub r#strip_query: Box<bool>,
}
