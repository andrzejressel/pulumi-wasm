#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolContainerConfiguration {
    /// A list of container image names to use, as would be specified by `docker pull`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "containerImageNames")]
    pub r#container_image_names: Box<Option<Vec<String>>>,
    /// One or more `container_registries` blocks as defined below. Additional container registries from which container images can be pulled by the pool's VMs. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "containerRegistries")]
    pub r#container_registries: Box<Option<Vec<super::super::types::batch::PoolContainerConfigurationContainerRegistry>>>,
    /// The type of container configuration. Possible value is `DockerCompatible`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
