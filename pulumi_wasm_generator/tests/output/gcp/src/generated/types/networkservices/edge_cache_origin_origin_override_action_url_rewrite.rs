#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EdgeCacheOriginOriginOverrideActionUrlRewrite {
    /// Prior to forwarding the request to the selected
    /// origin, the request's host header is replaced with
    /// contents of the hostRewrite.
    /// This value must be between 1 and 255 characters.
    #[builder(into, default)]
    #[serde(rename = "hostRewrite")]
    pub r#host_rewrite: Box<Option<String>>,
}
