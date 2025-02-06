#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContainerServicePrivateRegistryAccess {
    /// Describes a request to configure an Amazon Lightsail container service to access private container image repositories, such as Amazon Elastic Container Registry (Amazon ECR) private repositories. See ECR Image Puller Role below for more details.
    #[builder(into, default)]
    #[serde(rename = "ecrImagePullerRole")]
    pub r#ecr_image_puller_role: Box<Option<super::super::types::lightsail::ContainerServicePrivateRegistryAccessEcrImagePullerRole>>,
}
