#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionDataQualityBaselineConfig {
    /// The constraints resource for a monitoring job. Fields are documented below.
    #[builder(into, default)]
    #[serde(rename = "constraintsResource")]
    pub r#constraints_resource: Box<Option<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityBaselineConfigConstraintsResource>>,
    /// The statistics resource for a monitoring job. Fields are documented below.
    #[builder(into, default)]
    #[serde(rename = "statisticsResource")]
    pub r#statistics_resource: Box<Option<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityBaselineConfigStatisticsResource>>,
}