#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutoscaleSettingNotificationEmail {
    /// Specifies a list of custom email addresses to which the email notifications will be sent.
    #[builder(into, default)]
    #[serde(rename = "customEmails")]
    pub r#custom_emails: Box<Option<Vec<String>>>,
    /// Should email notifications be sent to the subscription administrator? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "sendToSubscriptionAdministrator")]
    pub r#send_to_subscription_administrator: Box<Option<bool>>,
    /// Should email notifications be sent to the subscription co-administrator? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "sendToSubscriptionCoAdministrator")]
    pub r#send_to_subscription_co_administrator: Box<Option<bool>>,
}
