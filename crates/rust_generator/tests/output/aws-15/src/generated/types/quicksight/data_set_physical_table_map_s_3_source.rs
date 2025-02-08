#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataSetPhysicalTableMapS3Source {
    /// ARN of the data source.
    #[builder(into)]
    #[serde(rename = "dataSourceArn")]
    pub r#data_source_arn: Box<String>,
    /// Column schema of the table. See input_columns.
    #[builder(into)]
    #[serde(rename = "inputColumns")]
    pub r#input_columns: Box<Vec<super::super::types::quicksight::DataSetPhysicalTableMapS3SourceInputColumn>>,
    /// Information about the format for the S3 source file or files. See upload_settings.
    #[builder(into)]
    #[serde(rename = "uploadSettings")]
    pub r#upload_settings: Box<super::super::types::quicksight::DataSetPhysicalTableMapS3SourceUploadSettings>,
}
