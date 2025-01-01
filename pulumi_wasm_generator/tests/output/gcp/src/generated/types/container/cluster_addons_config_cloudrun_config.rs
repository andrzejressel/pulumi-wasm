#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterAddonsConfigCloudrunConfig {
    /// The status of the CloudRun addon. It is disabled by default. Set `disabled=false` to enable.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<bool>,
    /// The load balancer type of CloudRun ingress service. It is external load balancer by default.
    /// Set `load_balancer_type=LOAD_BALANCER_TYPE_INTERNAL` to configure it as internal load balancer.
    #[builder(into, default)]
    #[serde(rename = "loadBalancerType")]
    pub r#load_balancer_type: Box<Option<String>>,
}
