#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ModelPrimaryContainerModelDataSource {
    /// The S3 location of model data to deploy.
    #[builder(into)]
    #[serde(rename = "s3DataSources")]
    pub r#s_3_data_sources: Box<Vec<super::super::types::sagemaker::ModelPrimaryContainerModelDataSourceS3DataSource>>,
}
