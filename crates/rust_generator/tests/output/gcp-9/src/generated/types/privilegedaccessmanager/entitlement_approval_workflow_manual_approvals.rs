#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EntitlementApprovalWorkflowManualApprovals {
    /// Optional. Do the approvers need to provide a justification for their actions?
    #[builder(into, default)]
    #[serde(rename = "requireApproverJustification")]
    pub r#require_approver_justification: Box<Option<bool>>,
    /// List of approval steps in this workflow. These steps would be followed in the specified order sequentially.  1 step is supported for now.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "steps")]
    pub r#steps: Box<Vec<super::super::types::privilegedaccessmanager::EntitlementApprovalWorkflowManualApprovalsStep>>,
}
