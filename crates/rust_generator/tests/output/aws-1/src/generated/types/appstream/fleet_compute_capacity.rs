#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetComputeCapacity {
    /// Number of currently available instances that can be used to stream sessions.
    #[builder(into, default)]
    #[serde(rename = "available")]
    pub r#available: Box<Option<i32>>,
    /// Desired number of streaming instances.
    #[builder(into, default)]
    #[serde(rename = "desiredInstances")]
    pub r#desired_instances: Box<Option<i32>>,
    /// Desired number of user sessions for a multi-session fleet. This is not allowed for single-session fleets.
    #[builder(into, default)]
    #[serde(rename = "desiredSessions")]
    pub r#desired_sessions: Box<Option<i32>>,
    /// Number of instances in use for streaming.
    #[builder(into, default)]
    #[serde(rename = "inUse")]
    pub r#in_use: Box<Option<i32>>,
    /// Total number of simultaneous streaming instances that are running.
    #[builder(into, default)]
    #[serde(rename = "running")]
    pub r#running: Box<Option<i32>>,
}
