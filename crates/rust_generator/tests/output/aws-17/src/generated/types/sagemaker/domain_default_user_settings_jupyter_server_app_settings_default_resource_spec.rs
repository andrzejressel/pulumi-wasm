#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainDefaultUserSettingsJupyterServerAppSettingsDefaultResourceSpec {
    /// The instance type that the image version runs on.. For valid values see [SageMaker Instance Types](https://docs.aws.amazon.com/sagemaker/latest/dg/notebooks-available-instance-types.html).
    #[builder(into, default)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<Option<String>>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configuration attached to the Resource.
    #[builder(into, default)]
    #[serde(rename = "lifecycleConfigArn")]
    pub r#lifecycle_config_arn: Box<Option<String>>,
    /// The ARN of the SageMaker image that the image version belongs to.
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
