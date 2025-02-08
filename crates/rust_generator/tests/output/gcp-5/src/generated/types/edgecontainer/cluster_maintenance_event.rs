#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterMaintenanceEvent {
    /// (Output)
    /// The time when the maintenance event request was created.
    #[builder(into, default)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<Option<String>>,
    /// (Output)
    /// The time when the maintenance event ended, either successfully or not. If
    /// the maintenance event is split into multiple maintenance windows,
    /// end_time is only updated when the whole flow ends.
    #[builder(into, default)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<Option<String>>,
    /// (Output)
    /// The operation for running the maintenance event. Specified in the format
    /// projects/*/locations/*/operations/*. If the maintenance event is split
    /// into multiple operations (e.g. due to maintenance windows), the latest
    /// one is recorded.
    #[builder(into, default)]
    #[serde(rename = "operation")]
    pub r#operation: Box<Option<String>>,
    /// (Output)
    /// The schedule of the maintenance event.
    #[builder(into, default)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<Option<String>>,
    /// (Output)
    /// The time when the maintenance event started.
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
    /// (Output)
    /// Indicates the maintenance event state.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// (Output)
    /// The target version of the cluster.
    #[builder(into, default)]
    #[serde(rename = "targetVersion")]
    pub r#target_version: Box<Option<String>>,
    /// (Output)
    /// Indicates the maintenance event type.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    /// (Output)
    /// The time when the maintenance event message was updated.
    #[builder(into, default)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<Option<String>>,
    /// (Output)
    /// UUID of the maintenance event.
    #[builder(into, default)]
    #[serde(rename = "uuid")]
    pub r#uuid: Box<Option<String>>,
}
