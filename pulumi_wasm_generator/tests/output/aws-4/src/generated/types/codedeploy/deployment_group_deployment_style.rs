#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeploymentGroupDeploymentStyle {
    /// Indicates whether to route deployment traffic behind a load balancer. Valid Values are `WITH_TRAFFIC_CONTROL` or `WITHOUT_TRAFFIC_CONTROL`. Default is `WITHOUT_TRAFFIC_CONTROL`.
    #[builder(into, default)]
    #[serde(rename = "deploymentOption")]
    pub r#deployment_option: Box<Option<String>>,
    /// Indicates whether to run an in-place deployment or a blue/green deployment. Valid Values are `IN_PLACE` or `BLUE_GREEN`. Default is `IN_PLACE`.
    /// 
    /// _Only one `deployment_style` is allowed_.
    #[builder(into, default)]
    #[serde(rename = "deploymentType")]
    pub r#deployment_type: Box<Option<String>>,
}
