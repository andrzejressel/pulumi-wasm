#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ModelContainerModelDataSource {
    /// The S3 location of model data to deploy.
    #[builder(into)]
    #[serde(rename = "s3DataSources")]
    pub r#s_3_data_sources: Box<Vec<super::super::types::sagemaker::ModelContainerModelDataSourceS3DataSource>>,
}
