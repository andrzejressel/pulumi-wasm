#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchTemplateLicenseSpecification {
    /// ARN of the license configuration.
    #[builder(into)]
    #[serde(rename = "licenseConfigurationArn")]
    pub r#license_configuration_arn: Box<String>,
}