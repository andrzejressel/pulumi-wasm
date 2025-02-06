#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRoleManagementPolicyNotificationRuleEligibleAssignment {
    /// A `notification_settings` block as defined above.
    #[builder(into)]
    #[serde(rename = "adminNotifications")]
    pub r#admin_notifications: Box<Vec<super::super::types::pim::GetRoleManagementPolicyNotificationRuleEligibleAssignmentAdminNotification>>,
    /// A `notification_settings` block as defined above.
    #[builder(into)]
    #[serde(rename = "approverNotifications")]
    pub r#approver_notifications: Box<Vec<super::super::types::pim::GetRoleManagementPolicyNotificationRuleEligibleAssignmentApproverNotification>>,
    /// A `notification_settings` block as defined above.
    #[builder(into)]
    #[serde(rename = "assigneeNotifications")]
    pub r#assignee_notifications: Box<Vec<super::super::types::pim::GetRoleManagementPolicyNotificationRuleEligibleAssignmentAssigneeNotification>>,
}
