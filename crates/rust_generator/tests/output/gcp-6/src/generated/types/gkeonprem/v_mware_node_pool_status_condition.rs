#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VMwareNodePoolStatusCondition {
    /// (Output)
    /// Last time the condition transit from one status to another.
    #[builder(into, default)]
    #[serde(rename = "lastTransitionTime")]
    pub r#last_transition_time: Box<Option<String>>,
    /// (Output)
    /// Human-readable message indicating details about last transition.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// (Output)
    /// Machine-readable message indicating details about last transition.
    #[builder(into, default)]
    #[serde(rename = "reason")]
    pub r#reason: Box<Option<String>>,
    /// (Output)
    /// The lifecycle state of the condition.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// (Output)
    /// Type of the condition.
    /// (e.g., ClusterRunning, NodePoolRunning or ServerSidePreflightReady)
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
