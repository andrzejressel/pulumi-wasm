#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecretRotationRotationRule {
    #[builder(into)]
    #[serde(rename = "automaticallyAfterDays")]
    pub r#automatically_after_days: Box<i32>,
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    #[builder(into)]
    #[serde(rename = "scheduleExpression")]
    pub r#schedule_expression: Box<String>,
}
