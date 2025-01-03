#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEntitlementApprovalWorkflow {
    /// A manual approval workflow where users who are designated as approvers need to call the ApproveGrant/DenyGrant APIs for an Grant.
    /// The workflow can consist of multiple serial steps where each step defines who can act as Approver in that step and how many of those users should approve before the workflow moves to the next step.
    /// This can be used to create approval workflows such as
    /// * Require an approval from any user in a group G.
    /// * Require an approval from any k number of users from a Group G.
    /// * Require an approval from any user in a group G and then from a user U. etc.
    /// A single user might be part of 'approvers' ACL for multiple steps in this workflow but they can only approve once and that approval will only be considered to satisfy the approval step at which it was granted.
    #[builder(into)]
    #[serde(rename = "manualApprovals")]
    pub r#manual_approvals: Box<Vec<super::super::types::privilegedaccessmanager::GetEntitlementApprovalWorkflowManualApproval>>,
}
