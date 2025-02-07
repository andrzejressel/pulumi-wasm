#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NodeGroupAutoscalingPolicy {
    /// Maximum size of the node group. Set to a value less than or equal
    /// to 100 and greater than or equal to min-nodes.
    #[builder(into, default)]
    #[serde(rename = "maxNodes")]
    pub r#max_nodes: Box<Option<i32>>,
    /// Minimum size of the node group. Must be less
    /// than or equal to max-nodes. The default value is 0.
    #[builder(into, default)]
    #[serde(rename = "minNodes")]
    pub r#min_nodes: Box<Option<i32>>,
    /// The autoscaling mode. Set to one of the following:
    /// - OFF: Disables the autoscaler.
    /// - ON: Enables scaling in and scaling out.
    /// - ONLY_SCALE_OUT: Enables only scaling out.
    /// You must use this mode if your node groups are configured to
    /// restart their hosted VMs on minimal servers.
    /// Possible values are: `OFF`, `ON`, `ONLY_SCALE_OUT`.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}
