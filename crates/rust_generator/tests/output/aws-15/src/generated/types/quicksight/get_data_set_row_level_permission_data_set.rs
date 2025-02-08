#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDataSetRowLevelPermissionDataSet {
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "formatVersion")]
    pub r#format_version: Box<String>,
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<String>,
    #[builder(into)]
    #[serde(rename = "permissionPolicy")]
    pub r#permission_policy: Box<String>,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
