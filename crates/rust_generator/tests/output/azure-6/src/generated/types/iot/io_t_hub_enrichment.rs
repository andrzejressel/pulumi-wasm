#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IoTHubEnrichment {
    /// The list of endpoints which will be enriched.
    #[builder(into)]
    #[serde(rename = "endpointNames")]
    pub r#endpoint_names: Box<Vec<String>>,
    /// The key of the enrichment.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The value of the enrichment. Value can be any static string, the name of the IoT Hub sending the message (use `$iothubname`) or information from the device twin (ex: `$twin.tags.latitude`)
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
