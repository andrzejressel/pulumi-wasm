#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRoleManagementPolicyActivationRuleApprovalStage {
    /// The IDs of the users or groups who can approve the activation
    #[builder(into)]
    #[serde(rename = "primaryApprovers")]
    pub r#primary_approvers: Box<Vec<super::super::types::pim::GetRoleManagementPolicyActivationRuleApprovalStagePrimaryApprover>>,
}