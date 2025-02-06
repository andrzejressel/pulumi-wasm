#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchema {
    /// Describes the mapping of each data element in the streaming source to the corresponding column in the in-application stream.
    #[builder(into)]
    #[serde(rename = "recordColumns")]
    pub r#record_columns: Box<Vec<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchemaRecordColumn>>,
    /// Specifies the encoding of the records in the streaming source. For example, `UTF-8`.
    #[builder(into, default)]
    #[serde(rename = "recordEncoding")]
    pub r#record_encoding: Box<Option<String>>,
    /// Specifies the format of the records on the streaming source.
    #[builder(into)]
    #[serde(rename = "recordFormat")]
    pub r#record_format: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchemaRecordFormat>,
}
