#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetastoreServiceTelemetryConfig {
    /// The output format of the Dataproc Metastore service's logs.
    /// Default value is `JSON`.
    /// Possible values are: `LEGACY`, `JSON`.
    #[builder(into, default)]
    #[serde(rename = "logFormat")]
    pub r#log_format: Box<Option<String>>,
}
