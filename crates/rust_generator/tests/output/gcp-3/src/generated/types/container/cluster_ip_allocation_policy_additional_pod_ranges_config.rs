#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterIpAllocationPolicyAdditionalPodRangesConfig {
    /// The names of the Pod ranges to add to the cluster.
    #[builder(into)]
    #[serde(rename = "podRangeNames")]
    pub r#pod_range_names: Box<Vec<String>>,
}
