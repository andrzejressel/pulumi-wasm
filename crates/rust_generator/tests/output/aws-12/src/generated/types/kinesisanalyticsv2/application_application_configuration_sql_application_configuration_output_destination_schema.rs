#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationOutputDestinationSchema {
    /// Specifies the format of the records on the output stream. Valid values: `CSV`, `JSON`.
    #[builder(into)]
    #[serde(rename = "recordFormatType")]
    pub r#record_format_type: Box<String>,
}
