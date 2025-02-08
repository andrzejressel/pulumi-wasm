#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNotificationChannelSensitiveLabel {
    /// An authorization token for a notification channel. Channel types that support this field include: slack
    #[builder(into)]
    #[serde(rename = "authToken")]
    pub r#auth_token: Box<String>,
    /// An password for a notification channel. Channel types that support this field include: webhook_basicauth
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// An servicekey token for a notification channel. Channel types that support this field include: pagerduty
    #[builder(into)]
    #[serde(rename = "serviceKey")]
    pub r#service_key: Box<String>,
}
