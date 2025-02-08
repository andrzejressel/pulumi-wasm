#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RoleManagementPolicyNotificationRulesEligibleActivations {
    /// Admin notification settings
    #[builder(into, default)]
    #[serde(rename = "adminNotifications")]
    pub r#admin_notifications: Box<Option<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleActivationsAdminNotifications>>,
    /// Approver notification settings
    #[builder(into, default)]
    #[serde(rename = "approverNotifications")]
    pub r#approver_notifications: Box<Option<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleActivationsApproverNotifications>>,
    /// Assignee notification settings
    #[builder(into, default)]
    #[serde(rename = "assigneeNotifications")]
    pub r#assignee_notifications: Box<Option<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleActivationsAssigneeNotifications>>,
}
