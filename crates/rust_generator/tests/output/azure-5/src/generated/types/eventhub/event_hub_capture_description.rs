#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventHubCaptureDescription {
    /// A `destination` block as defined below.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::eventhub::EventHubCaptureDescriptionDestination>,
    /// Specifies if the Capture Description is Enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Specifies the Encoding used for the Capture Description. Possible values are `Avro` and `AvroDeflate`.
    #[builder(into)]
    #[serde(rename = "encoding")]
    pub r#encoding: Box<String>,
    /// Specifies the time interval in seconds at which the capture will happen. Values can be between `60` and `900` seconds. Defaults to `300` seconds.
    #[builder(into, default)]
    #[serde(rename = "intervalInSeconds")]
    pub r#interval_in_seconds: Box<Option<i32>>,
    /// Specifies the amount of data built up in your EventHub before a Capture Operation occurs. Value should be between `10485760` and `524288000` bytes. Defaults to `314572800` bytes.
    #[builder(into, default)]
    #[serde(rename = "sizeLimitInBytes")]
    pub r#size_limit_in_bytes: Box<Option<i32>>,
    /// Specifies if empty files should not be emitted if no events occur during the Capture time window. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "skipEmptyArchives")]
    pub r#skip_empty_archives: Box<Option<bool>>,
}
