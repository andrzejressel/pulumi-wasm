#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CanarySchedule {
    /// Duration in seconds, for the canary to continue making regular runs according to the schedule in the Expression value.
    #[builder(into, default)]
    #[serde(rename = "durationInSeconds")]
    pub r#duration_in_seconds: Box<Option<i32>>,
    /// Rate expression or cron expression that defines how often the canary is to run. For rate expression, the syntax is `rate(number unit)`. _unit_ can be `minute`, `minutes`, or `hour`. For cron expression, the syntax is `cron(expression)`. For more information about the syntax for cron expressions, see [Scheduling canary runs using cron](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_cron.html).
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
}
