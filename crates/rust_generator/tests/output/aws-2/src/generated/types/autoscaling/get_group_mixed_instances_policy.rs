#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGroupMixedInstancesPolicy {
    /// List of instances distribution objects.
    #[builder(into)]
    #[serde(rename = "instancesDistributions")]
    pub r#instances_distributions: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyInstancesDistribution>>,
    /// List of launch templates along with the overrides.
    #[builder(into)]
    #[serde(rename = "launchTemplates")]
    pub r#launch_templates: Box<Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplate>>,
}
