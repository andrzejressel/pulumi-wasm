#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBackendBucketCdnPolicyBypassCacheOnRequestHeader {
    /// The header field name to match on when bypassing cache. Values are case-insensitive.
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: Box<String>,
}
