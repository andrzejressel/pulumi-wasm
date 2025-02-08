#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ModelContainerImageConfig {
    /// Specifies whether the model container is in Amazon ECR or a private Docker registry accessible from your Amazon Virtual Private Cloud (VPC). Allowed values are: `Platform` and `Vpc`.
    #[builder(into)]
    #[serde(rename = "repositoryAccessMode")]
    pub r#repository_access_mode: Box<String>,
    /// Specifies an authentication configuration for the private docker registry where your model image is hosted. Specify a value for this property only if you specified Vpc as the value for the RepositoryAccessMode field, and the private Docker registry where the model image is hosted requires authentication. see Repository Auth Config.
    #[builder(into, default)]
    #[serde(rename = "repositoryAuthConfig")]
    pub r#repository_auth_config: Box<Option<super::super::types::sagemaker::ModelContainerImageConfigRepositoryAuthConfig>>,
}
