#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolContainerConfiguration {
    /// A list of container image names to use, as would be specified by `docker pull`.
    #[builder(into)]
    #[serde(rename = "containerImageNames")]
    pub r#container_image_names: Box<Vec<String>>,
    /// Additional container registries from which container images can be pulled by the pool's VMs.
    #[builder(into)]
    #[serde(rename = "containerRegistries")]
    pub r#container_registries: Box<Vec<super::super::types::batch::GetPoolContainerConfigurationContainerRegistry>>,
    /// The type of container configuration.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
