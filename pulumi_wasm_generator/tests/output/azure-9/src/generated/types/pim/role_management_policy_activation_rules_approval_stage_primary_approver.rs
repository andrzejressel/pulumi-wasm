#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RoleManagementPolicyActivationRulesApprovalStagePrimaryApprover {
    /// The ID of the object which will act as an approver.
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: Box<String>,
    /// The type of object acting as an approver. Possible options are `User` and `Group`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
