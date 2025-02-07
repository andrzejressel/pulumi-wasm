#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KxClusterScalingGroupConfiguration {
    /// The number of vCPUs that you want to reserve for each node of this kdb cluster on the scaling group host.
    #[builder(into, default)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<Option<f64>>,
    /// An optional hard limit on the amount of memory a kdb cluster can use.
    #[builder(into, default)]
    #[serde(rename = "memoryLimit")]
    pub r#memory_limit: Box<Option<i32>>,
    /// A reservation of the minimum amount of memory that should be available on the scaling group for a kdb cluster to be successfully placed in a scaling group.
    #[builder(into)]
    #[serde(rename = "memoryReservation")]
    pub r#memory_reservation: Box<i32>,
    /// The number of kdb cluster nodes.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<i32>,
    /// A unique identifier for the kdb scaling group.
    #[builder(into)]
    #[serde(rename = "scalingGroupName")]
    pub r#scaling_group_name: Box<String>,
}
