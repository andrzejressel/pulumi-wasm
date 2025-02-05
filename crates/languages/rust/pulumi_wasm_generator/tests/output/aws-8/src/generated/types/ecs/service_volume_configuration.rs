#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceVolumeConfiguration {
    /// Configuration for the Amazon EBS volume that Amazon ECS creates and manages on your behalf. See below.
    #[builder(into)]
    #[serde(rename = "managedEbsVolume")]
    pub r#managed_ebs_volume: Box<super::super::types::ecs::ServiceVolumeConfigurationManagedEbsVolume>,
    /// Name of the volume.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
