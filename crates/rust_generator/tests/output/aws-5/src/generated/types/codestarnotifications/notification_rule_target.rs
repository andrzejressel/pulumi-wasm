#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NotificationRuleTarget {
    /// The ARN of notification rule target. For example, a SNS Topic ARN.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// The status of the notification rule. Possible values are `ENABLED` and `DISABLED`, default is `ENABLED`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// The type of the notification target. Default value is `SNS`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
