#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineConfigurationAssignmentConfigurationParameter {
    /// The name of the configuration parameter to check.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value to check the configuration parameter with.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
