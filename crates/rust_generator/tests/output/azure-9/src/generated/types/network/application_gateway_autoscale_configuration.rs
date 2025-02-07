#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewayAutoscaleConfiguration {
    /// Maximum capacity for autoscaling. Accepted values are in the range `2` to `125`.
    #[builder(into, default)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: Box<Option<i32>>,
    /// Minimum capacity for autoscaling. Accepted values are in the range `0` to `100`.
    #[builder(into)]
    #[serde(rename = "minCapacity")]
    pub r#min_capacity: Box<i32>,
}
