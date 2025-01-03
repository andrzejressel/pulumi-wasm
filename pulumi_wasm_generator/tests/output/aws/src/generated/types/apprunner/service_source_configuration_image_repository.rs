#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceSourceConfigurationImageRepository {
    /// Configuration for running the identified image. See Image Configuration below for more details.
    #[builder(into, default)]
    #[serde(rename = "imageConfiguration")]
    pub r#image_configuration: Box<Option<super::super::types::apprunner::ServiceSourceConfigurationImageRepositoryImageConfiguration>>,
    /// Identifier of an image. For an image in Amazon Elastic Container Registry (Amazon ECR), this is an image name. For the
    /// image name format, see Pulling an image in the Amazon ECR User Guide.
    #[builder(into)]
    #[serde(rename = "imageIdentifier")]
    pub r#image_identifier: Box<String>,
    /// Type of the image repository. This reflects the repository provider and whether the repository is private or public. Valid values: `ECR` , `ECR_PUBLIC`.
    #[builder(into)]
    #[serde(rename = "imageRepositoryType")]
    pub r#image_repository_type: Box<String>,
}
