#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataSetRowLevelPermissionDataSet {
    /// ARN of the dataset that contains permissions for RLS.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// User or group rules associated with the dataset that contains permissions for RLS.
    #[builder(into, default)]
    #[serde(rename = "formatVersion")]
    pub r#format_version: Box<Option<String>>,
    /// Namespace associated with the dataset that contains permissions for RLS.
    #[builder(into, default)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<Option<String>>,
    /// Type of permissions to use when interpreting the permissions for RLS. Valid values are `GRANT_ACCESS` and `DENY_ACCESS`.
    #[builder(into)]
    #[serde(rename = "permissionPolicy")]
    pub r#permission_policy: Box<String>,
    /// Status of the row-level security permission dataset. If enabled, the status is `ENABLED`. If disabled, the status is `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
