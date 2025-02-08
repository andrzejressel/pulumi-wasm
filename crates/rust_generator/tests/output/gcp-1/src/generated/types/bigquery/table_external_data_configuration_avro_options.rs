#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableExternalDataConfigurationAvroOptions {
    /// If is set to true, indicates whether
    /// to interpret logical types as the corresponding BigQuery data type
    /// (for example, TIMESTAMP), instead of using the raw type (for example, INTEGER).
    #[builder(into)]
    #[serde(rename = "useAvroLogicalTypes")]
    pub r#use_avro_logical_types: Box<bool>,
}
