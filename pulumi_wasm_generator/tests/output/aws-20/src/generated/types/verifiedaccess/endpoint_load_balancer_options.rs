#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointLoadBalancerOptions {
    #[builder(into, default)]
    #[serde(rename = "loadBalancerArn")]
    pub r#load_balancer_arn: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Option<Vec<String>>>,
}
