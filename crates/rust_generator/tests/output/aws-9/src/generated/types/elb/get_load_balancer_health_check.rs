#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLoadBalancerHealthCheck {
    #[builder(into)]
    #[serde(rename = "healthyThreshold")]
    pub r#healthy_threshold: Box<i32>,
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<i32>,
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<i32>,
    #[builder(into)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: Box<i32>,
}
