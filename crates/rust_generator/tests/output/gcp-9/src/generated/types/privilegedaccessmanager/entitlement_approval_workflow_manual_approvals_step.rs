#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EntitlementApprovalWorkflowManualApprovalsStep {
    /// How many users from the above list need to approve.
    /// If there are not enough distinct users in the list above then the workflow
    /// will indefinitely block. Should always be greater than 0. Currently 1 is the only
    /// supported value.
    #[builder(into, default)]
    #[serde(rename = "approvalsNeeded")]
    pub r#approvals_needed: Box<Option<i32>>,
    /// Optional. Additional email addresses to be notified when a grant is pending approval.
    #[builder(into, default)]
    #[serde(rename = "approverEmailRecipients")]
    pub r#approver_email_recipients: Box<Option<Vec<String>>>,
    /// The potential set of approvers in this step. This list should contain at only one entry.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "approvers")]
    pub r#approvers: Box<super::super::types::privilegedaccessmanager::EntitlementApprovalWorkflowManualApprovalsStepApprovers>,
}
