#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkInsightsAnalysisExplanationClassicLoadBalancerListener {
    #[builder(into)]
    #[serde(rename = "instancePort")]
    pub r#instance_port: Box<i32>,
    #[builder(into)]
    #[serde(rename = "loadBalancerPort")]
    pub r#load_balancer_port: Box<i32>,
}
