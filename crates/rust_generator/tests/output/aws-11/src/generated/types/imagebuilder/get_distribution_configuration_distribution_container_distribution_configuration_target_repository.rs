#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDistributionConfigurationDistributionContainerDistributionConfigurationTargetRepository {
    /// Name of the container repository where the output container image is stored.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: Box<String>,
    /// Service in which the image is registered.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
