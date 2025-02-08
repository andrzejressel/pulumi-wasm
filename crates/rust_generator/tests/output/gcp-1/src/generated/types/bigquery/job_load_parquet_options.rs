#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobLoadParquetOptions {
    /// If sourceFormat is set to PARQUET, indicates whether to use schema inference specifically for Parquet LIST logical type.
    #[builder(into, default)]
    #[serde(rename = "enableListInference")]
    pub r#enable_list_inference: Box<Option<bool>>,
    /// If sourceFormat is set to PARQUET, indicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default.
    #[builder(into, default)]
    #[serde(rename = "enumAsString")]
    pub r#enum_as_string: Box<Option<bool>>,
}
