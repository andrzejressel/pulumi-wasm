#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SubscriptionPushConfigNoWrapper {
    /// When true, writes the Pub/Sub message metadata to
    /// `x-goog-pubsub-<KEY>:<VAL>` headers of the HTTP request. Writes the
    /// Pub/Sub message attributes to `<KEY>:<VAL>` headers of the HTTP request.
    #[builder(into)]
    #[serde(rename = "writeMetadata")]
    pub r#write_metadata: Box<bool>,
}
