#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpaceSpaceSettingsJupyterLabAppSettingsDefaultResourceSpec {
    /// The instance type.
    #[builder(into, default)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<Option<String>>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configuration attached to the Resource.
    #[builder(into, default)]
    #[serde(rename = "lifecycleConfigArn")]
    pub r#lifecycle_config_arn: Box<Option<String>>,
    /// The Amazon Resource Name (ARN) of the SageMaker image created on the instance.
    #[builder(into, default)]
    #[serde(rename = "sagemakerImageArn")]
    pub r#sagemaker_image_arn: Box<Option<String>>,
    /// The SageMaker Image Version Alias.
    #[builder(into, default)]
    #[serde(rename = "sagemakerImageVersionAlias")]
    pub r#sagemaker_image_version_alias: Box<Option<String>>,
    /// The ARN of the image version created on the instance.
    #[builder(into, default)]
    #[serde(rename = "sagemakerImageVersionArn")]
    pub r#sagemaker_image_version_arn: Box<Option<String>>,
}
