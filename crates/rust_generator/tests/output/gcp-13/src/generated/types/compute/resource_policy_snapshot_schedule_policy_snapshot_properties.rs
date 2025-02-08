#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResourcePolicySnapshotSchedulePolicySnapshotProperties {
    /// Creates the new snapshot in the snapshot chain labeled with the
    /// specified name. The chain name must be 1-63 characters long and comply
    /// with RFC1035.
    #[builder(into, default)]
    #[serde(rename = "chainName")]
    pub r#chain_name: Box<Option<String>>,
    /// Whether to perform a 'guest aware' snapshot.
    #[builder(into, default)]
    #[serde(rename = "guestFlush")]
    pub r#guest_flush: Box<Option<bool>>,
    /// A set of key-value pairs.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Cloud Storage bucket location to store the auto snapshot
    /// (regional or multi-regional)
    #[builder(into, default)]
    #[serde(rename = "storageLocations")]
    pub r#storage_locations: Box<Option<String>>,
}
