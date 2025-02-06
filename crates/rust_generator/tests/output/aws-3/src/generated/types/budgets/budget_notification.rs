#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BudgetNotification {
    /// (Required) Comparison operator to use to evaluate the condition. Can be `LESS_THAN`, `EQUAL_TO` or `GREATER_THAN`.
    #[builder(into)]
    #[serde(rename = "comparisonOperator")]
    pub r#comparison_operator: Box<String>,
    /// (Required) What kind of budget value to notify on. Can be `ACTUAL` or `FORECASTED`
    #[builder(into)]
    #[serde(rename = "notificationType")]
    pub r#notification_type: Box<String>,
    /// (Optional) E-Mail addresses to notify. Either this or `subscriber_sns_topic_arns` is required.
    #[builder(into, default)]
    #[serde(rename = "subscriberEmailAddresses")]
    pub r#subscriber_email_addresses: Box<Option<Vec<String>>>,
    /// (Optional) SNS topics to notify. Either this or `subscriber_email_addresses` is required.
    #[builder(into, default)]
    #[serde(rename = "subscriberSnsTopicArns")]
    pub r#subscriber_sns_topic_arns: Box<Option<Vec<String>>>,
    /// (Required) Threshold when the notification should be sent.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<f64>,
    /// (Required) What kind of threshold is defined. Can be `PERCENTAGE` OR `ABSOLUTE_VALUE`.
    #[builder(into)]
    #[serde(rename = "thresholdType")]
    pub r#threshold_type: Box<String>,
}
