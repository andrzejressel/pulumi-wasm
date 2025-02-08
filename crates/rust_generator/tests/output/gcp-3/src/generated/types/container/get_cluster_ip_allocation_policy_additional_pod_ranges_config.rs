#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterIpAllocationPolicyAdditionalPodRangesConfig {
    /// Name for pod secondary ipv4 range which has the actual range defined ahead.
    #[builder(into)]
    #[serde(rename = "podRangeNames")]
    pub r#pod_range_names: Box<Vec<String>>,
}
