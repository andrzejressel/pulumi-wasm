#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DetectorFeatureAdditionalConfiguration {
    /// The name of the additional configuration for a feature. Valid values: `EKS_ADDON_MANAGEMENT`, `ECS_FARGATE_AGENT_MANAGEMENT`, `EC2_AGENT_MANAGEMENT`. Refer to the [AWS Documentation](https://docs.aws.amazon.com/guardduty/latest/APIReference/API_DetectorAdditionalConfiguration.html) for the current list of supported values.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The status of the additional configuration. Valid values: `ENABLED`, `DISABLED`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}