#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRoleManagementPolicyNotificationRuleEligibleActivationAdminNotification {
    /// A list of additional email addresses that will receive these notifications.
    #[builder(into)]
    #[serde(rename = "additionalRecipients")]
    pub r#additional_recipients: Box<Vec<String>>,
    /// (Boolean) Should the default recipients receive these notifications.
    #[builder(into)]
    #[serde(rename = "defaultRecipients")]
    pub r#default_recipients: Box<bool>,
    /// (String) What level of notifications should be sent. Either `All` or `Critical`.
    #[builder(into)]
    #[serde(rename = "notificationLevel")]
    pub r#notification_level: Box<String>,
}
