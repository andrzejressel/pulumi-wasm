#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationGatewayAutoscaleConfiguration {
    /// Maximum capacity for autoscaling.
    #[builder(into)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: Box<i32>,
    /// Minimum capacity for autoscaling.
    #[builder(into)]
    #[serde(rename = "minCapacity")]
    pub r#min_capacity: Box<i32>,
}
