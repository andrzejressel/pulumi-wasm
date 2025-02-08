#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PipelineThumbnailConfigPermission {
    /// The permission that you want to give to the AWS user that you specified in `thumbnail_config_permissions.grantee`. Valid values are `Read`, `ReadAcp`, `WriteAcp` or `FullControl`.
    #[builder(into, default)]
    #[serde(rename = "accesses")]
    pub r#accesses: Box<Option<Vec<String>>>,
    /// The AWS user or group that you want to have access to thumbnail files.
    #[builder(into, default)]
    #[serde(rename = "grantee")]
    pub r#grantee: Box<Option<String>>,
    /// Specify the type of value that appears in the `thumbnail_config_permissions.grantee` object. Valid values are `Canonical`, `Email` or `Group`.
    #[builder(into, default)]
    #[serde(rename = "granteeType")]
    pub r#grantee_type: Box<Option<String>>,
}
