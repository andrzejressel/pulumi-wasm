#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RoleManagementPolicyNotificationRulesActiveAssignmentsAssigneeNotifications {
    /// The additional recipients to notify
    #[builder(into, default)]
    #[serde(rename = "additionalRecipients")]
    pub r#additional_recipients: Box<Option<Vec<String>>>,
    /// Whether the default recipients are notified
    #[builder(into)]
    #[serde(rename = "defaultRecipients")]
    pub r#default_recipients: Box<bool>,
    /// What level of notifications are sent
    #[builder(into)]
    #[serde(rename = "notificationLevel")]
    pub r#notification_level: Box<String>,
}