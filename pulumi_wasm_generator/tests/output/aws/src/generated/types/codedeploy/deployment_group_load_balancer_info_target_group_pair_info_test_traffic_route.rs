#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeploymentGroupLoadBalancerInfoTargetGroupPairInfoTestTrafficRoute {
    /// List of Amazon Resource Names (ARNs) of the load balancer listeners.
    #[builder(into)]
    #[serde(rename = "listenerArns")]
    pub r#listener_arns: Box<Vec<String>>,
}
