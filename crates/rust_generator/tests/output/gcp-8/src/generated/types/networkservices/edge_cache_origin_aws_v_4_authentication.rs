#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EdgeCacheOriginAwsV4Authentication {
    /// The access key ID your origin uses to identify the key.
    #[builder(into)]
    #[serde(rename = "accessKeyId")]
    pub r#access_key_id: Box<String>,
    /// The name of the AWS region that your origin is in.
    #[builder(into)]
    #[serde(rename = "originRegion")]
    pub r#origin_region: Box<String>,
    /// The Secret Manager secret version of the secret access key used by your origin.
    /// 
    /// This is the resource name of the secret version in the format 'projects/*/secrets/*/versions/*' where the '*' values are replaced by the project, secret, and version you require.
    #[builder(into)]
    #[serde(rename = "secretAccessKeyVersion")]
    pub r#secret_access_key_version: Box<String>,
}
