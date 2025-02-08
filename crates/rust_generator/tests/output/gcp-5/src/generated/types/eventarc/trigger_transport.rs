#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TriggerTransport {
    /// The Pub/Sub topic and subscription used by Eventarc as delivery intermediary.
    #[builder(into, default)]
    #[serde(rename = "pubsub")]
    pub r#pubsub: Box<Option<super::super::types::eventarc::TriggerTransportPubsub>>,
}
