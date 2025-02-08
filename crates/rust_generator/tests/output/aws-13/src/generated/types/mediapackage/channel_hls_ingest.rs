#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelHlsIngest {
    /// A list of the ingest endpoints
    #[builder(into, default)]
    #[serde(rename = "ingestEndpoints")]
    pub r#ingest_endpoints: Box<Option<Vec<super::super::types::mediapackage::ChannelHlsIngestIngestEndpoint>>>,
}
