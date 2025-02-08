#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BackendBucketCdnPolicyBypassCacheOnRequestHeader {
    /// The header field name to match on when bypassing cache. Values are case-insensitive.
    #[builder(into, default)]
    #[serde(rename = "headerName")]
    pub r#header_name: Box<Option<String>>,
}
