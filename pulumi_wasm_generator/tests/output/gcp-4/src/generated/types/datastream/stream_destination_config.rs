#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamDestinationConfig {
    /// A configuration for how data should be loaded to Google BigQuery.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "bigqueryDestinationConfig")]
    pub r#bigquery_destination_config: Box<Option<super::super::types::datastream::StreamDestinationConfigBigqueryDestinationConfig>>,
    /// Destination connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}
    #[builder(into)]
    #[serde(rename = "destinationConnectionProfile")]
    pub r#destination_connection_profile: Box<String>,
    /// A configuration for how data should be loaded to Cloud Storage.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gcsDestinationConfig")]
    pub r#gcs_destination_config: Box<Option<super::super::types::datastream::StreamDestinationConfigGcsDestinationConfig>>,
}
