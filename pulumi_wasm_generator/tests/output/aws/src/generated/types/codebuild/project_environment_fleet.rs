#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProjectEnvironmentFleet {
    /// Compute fleet ARN for the build project.
    #[builder(into, default)]
    #[serde(rename = "fleetArn")]
    pub r#fleet_arn: Box<Option<String>>,
}
