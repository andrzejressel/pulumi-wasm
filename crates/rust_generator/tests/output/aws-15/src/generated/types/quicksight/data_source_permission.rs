#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourcePermission {
    /// Set of IAM actions to grant or revoke permissions on. Max of 16 items.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<String>>,
    /// The Amazon Resource Name (ARN) of the principal.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: Box<String>,
}
