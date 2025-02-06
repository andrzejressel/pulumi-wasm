#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableExternalDataConfigurationParquetOptions {
    /// Indicates whether to use schema inference specifically for Parquet LIST logical type.
    #[builder(into, default)]
    #[serde(rename = "enableListInference")]
    pub r#enable_list_inference: Box<Option<bool>>,
    /// Indicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default.
    #[builder(into, default)]
    #[serde(rename = "enumAsString")]
    pub r#enum_as_string: Box<Option<bool>>,
}
