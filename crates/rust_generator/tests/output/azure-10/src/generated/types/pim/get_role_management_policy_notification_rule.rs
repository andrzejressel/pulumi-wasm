#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRoleManagementPolicyNotificationRule {
    /// A `notification_target` block as defined below with the details of notfications on active role assignments.
    #[builder(into)]
    #[serde(rename = "activeAssignments")]
    pub r#active_assignments: Box<Vec<super::super::types::pim::GetRoleManagementPolicyNotificationRuleActiveAssignment>>,
    /// A `notification_target` block as defined below with the details of notifications on activation of eligible role.
    #[builder(into)]
    #[serde(rename = "eligibleActivations")]
    pub r#eligible_activations: Box<Vec<super::super::types::pim::GetRoleManagementPolicyNotificationRuleEligibleActivation>>,
    /// A `notification_target` block as defined below with the details of notifications on eligible role assignments.
    #[builder(into)]
    #[serde(rename = "eligibleAssignments")]
    pub r#eligible_assignments: Box<Vec<super::super::types::pim::GetRoleManagementPolicyNotificationRuleEligibleAssignment>>,
}
