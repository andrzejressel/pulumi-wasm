#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionConfigurationDistributionContainerDistributionConfigurationTargetRepository {
    /// The name of the container repository where the output container image is stored. This name is prefixed by the repository location.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: Box<String>,
    /// The service in which this image is registered. Valid values: `ECR`.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
