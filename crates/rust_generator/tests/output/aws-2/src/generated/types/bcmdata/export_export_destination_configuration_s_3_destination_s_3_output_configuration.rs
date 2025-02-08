#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ExportExportDestinationConfigurationS3DestinationS3OutputConfiguration {
    /// Compression type for the data export. Valid values `GZIP`, `PARQUET`.
    #[builder(into)]
    #[serde(rename = "compression")]
    pub r#compression: Box<String>,
    /// File format for the data export. Valid values `TEXT_OR_CSV` or `PARQUET`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Box<String>,
    /// Output type for the data export. Valid value `CUSTOM`.
    #[builder(into)]
    #[serde(rename = "outputType")]
    pub r#output_type: Box<String>,
    /// The rule to follow when generating a version of the data export file. You have the choice to overwrite the previous version or to be delivered in addition to the previous versions. Overwriting exports can save on Amazon S3 storage costs. Creating new export versions allows you to track the changes in cost and usage data over time. Valid values `CREATE_NEW_REPORT` or `OVERWRITE_REPORT`.
    #[builder(into)]
    #[serde(rename = "overwrite")]
    pub r#overwrite: Box<String>,
}
