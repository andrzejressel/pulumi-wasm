#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionJobResources {
    /// The configuration for the cluster resources used to run the processing job. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "clusterConfig")]
    pub r#cluster_config: Box<super::super::types::sagemaker::DataQualityJobDefinitionJobResourcesClusterConfig>,
}
