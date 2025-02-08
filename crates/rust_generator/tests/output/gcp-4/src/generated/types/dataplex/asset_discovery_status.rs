#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AssetDiscoveryStatus {
    /// The duration of the last discovery run.
    #[builder(into, default)]
    #[serde(rename = "lastRunDuration")]
    pub r#last_run_duration: Box<Option<String>>,
    /// The start time of the last discovery run.
    #[builder(into, default)]
    #[serde(rename = "lastRunTime")]
    pub r#last_run_time: Box<Option<String>>,
    /// Additional information about the current state.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// Output only. Current state of the asset. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// Data Stats of the asset reported by discovery.
    #[builder(into, default)]
    #[serde(rename = "stats")]
    pub r#stats: Box<Option<Vec<super::super::types::dataplex::AssetDiscoveryStatusStat>>>,
    /// Output only. The time when the asset was last updated.
    #[builder(into, default)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<Option<String>>,
}
