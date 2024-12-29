#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SubscriberNotificationConfiguration {
    /// The configurations for HTTPS subscriber notification.
    #[builder(into, default)]
    #[serde(rename = "httpsNotificationConfiguration")]
    pub r#https_notification_configuration: Box<Option<super::super::types::securitylake::SubscriberNotificationConfigurationHttpsNotificationConfiguration>>,
    /// The configurations for SQS subscriber notification.
    /// There are no parameters within `sqs_notification_configuration`.
    #[builder(into, default)]
    #[serde(rename = "sqsNotificationConfiguration")]
    pub r#sqs_notification_configuration: Box<Option<super::super::types::securitylake::SubscriberNotificationConfigurationSqsNotificationConfiguration>>,
}
