#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerFargatePlatformConfiguration {
    /// The AWS Fargate platform version where the jobs are running. A platform version is specified only for jobs that are running on Fargate resources.
    #[builder(into)]
    #[serde(rename = "platformVersion")]
    pub r#platform_version: Box<String>,
}
