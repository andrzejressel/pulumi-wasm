#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentDataSourceDataSourceConfiguration {
    /// Details about the configuration of the S3 object containing the data source. See `s3_data_source_configuration` block for details.
    #[builder(into, default)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<Option<super::super::types::bedrock::AgentDataSourceDataSourceConfigurationS3Configuration>>,
    /// Type of storage for the data source. Valid values: `S3`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
