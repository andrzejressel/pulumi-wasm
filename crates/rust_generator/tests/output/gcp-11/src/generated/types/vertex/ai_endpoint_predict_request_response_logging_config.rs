#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AiEndpointPredictRequestResponseLoggingConfig {
    /// BigQuery table for logging. If only given a project, a new dataset will be created with name `logging_<endpoint-display-name>_<endpoint-id>` where will be made BigQuery-dataset-name compatible (e.g. most special characters will become underscores). If no table name is given, a new table will be created with name `request_response_logging`
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "bigqueryDestination")]
    pub r#bigquery_destination: Box<Option<super::super::types::vertex::AiEndpointPredictRequestResponseLoggingConfigBigqueryDestination>>,
    /// If logging is enabled or not.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Percentage of requests to be logged, expressed as a fraction in range(0,1]
    #[builder(into, default)]
    #[serde(rename = "samplingRate")]
    pub r#sampling_rate: Box<Option<f64>>,
}
