#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicMessageStoragePolicy {
    /// A list of IDs of GCP regions where messages that are published to
    /// the topic may be persisted in storage. Messages published by
    /// publishers running in non-allowed GCP regions (or running outside
    /// of GCP altogether) will be routed for storage in one of the
    /// allowed regions. An empty list means that no regions are allowed,
    /// and is not a valid configuration.
    #[builder(into)]
    #[serde(rename = "allowedPersistenceRegions")]
    pub r#allowed_persistence_regions: Box<Vec<String>>,
}
