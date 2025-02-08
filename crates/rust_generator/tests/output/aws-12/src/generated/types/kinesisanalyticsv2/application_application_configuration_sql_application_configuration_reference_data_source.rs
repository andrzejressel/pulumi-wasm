#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSource {
    #[builder(into, default)]
    #[serde(rename = "referenceId")]
    pub r#reference_id: Box<Option<String>>,
    /// Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.
    #[builder(into)]
    #[serde(rename = "referenceSchema")]
    pub r#reference_schema: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchema>,
    /// Identifies the S3 bucket and object that contains the reference data.
    #[builder(into)]
    #[serde(rename = "s3ReferenceDataSource")]
    pub r#s_3_reference_data_source: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceS3ReferenceDataSource>,
    /// The name of the in-application table to create.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: Box<String>,
}
