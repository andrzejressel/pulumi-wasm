#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AnalyticsApplicationReferenceDataSources {
    /// The ARN of the Kinesis Analytics Application.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The S3 configuration for the reference data source. See S3 Reference below for more details.
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesS3>,
    /// The Schema format of the data in the streaming source. See Source Schema below for more details.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Box<super::super::types::kinesis::AnalyticsApplicationReferenceDataSourcesSchema>,
    /// The in-application Table Name.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: Box<String>,
}
