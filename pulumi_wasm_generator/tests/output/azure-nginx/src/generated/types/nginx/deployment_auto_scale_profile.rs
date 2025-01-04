#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeploymentAutoScaleProfile {
    #[builder(into)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: Box<i32>,
    /// Specify the minimum number of NGINX capacity units for this NGINX Deployment.
    #[builder(into)]
    #[serde(rename = "minCapacity")]
    pub r#min_capacity: Box<i32>,
    /// Specify the name of the autoscaling profile.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
