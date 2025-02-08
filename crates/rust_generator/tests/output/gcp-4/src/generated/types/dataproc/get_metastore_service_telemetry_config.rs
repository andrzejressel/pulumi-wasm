#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetMetastoreServiceTelemetryConfig {
    /// The output format of the Dataproc Metastore service's logs. Default value: "JSON" Possible values: ["LEGACY", "JSON"]
    #[builder(into)]
    #[serde(rename = "logFormat")]
    pub r#log_format: Box<String>,
}
