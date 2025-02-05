#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionUrlMapPathMatcherPathRuleRouteActionUrlRewrite {
    /// Before forwarding the request to the selected service, the request's host header is replaced with contents of hostRewrite.
    /// The value must be from 1 to 255 characters.
    #[builder(into, default)]
    #[serde(rename = "hostRewrite")]
    pub r#host_rewrite: Box<Option<String>>,
    /// Before forwarding the request to the selected backend service, the matching portion of the request's path is replaced by pathPrefixRewrite.
    /// The value must be from 1 to 1024 characters.
    #[builder(into, default)]
    #[serde(rename = "pathPrefixRewrite")]
    pub r#path_prefix_rewrite: Box<Option<String>>,
}
