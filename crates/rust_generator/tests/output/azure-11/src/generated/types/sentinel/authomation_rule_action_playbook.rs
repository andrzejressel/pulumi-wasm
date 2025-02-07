#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthomationRuleActionPlaybook {
    /// The ID of the Logic App that defines the playbook's logic.
    #[builder(into)]
    #[serde(rename = "logicAppId")]
    pub r#logic_app_id: Box<String>,
    /// The execution order of this action.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Box<i32>,
    /// The ID of the Tenant that owns the playbook.
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
}
