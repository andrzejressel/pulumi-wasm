#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicyProvisioningModelMix {
    /// The base capacity that will always use Standard VMs to avoid risk of more preemption than the minimum capacity you need.
    #[builder(into, default)]
    #[serde(rename = "standardCapacityBase")]
    pub r#standard_capacity_base: Box<Option<i32>>,
    /// The percentage of target capacity that should use Standard VM. The remaining percentage will use Spot VMs.
    #[builder(into, default)]
    #[serde(rename = "standardCapacityPercentAboveBase")]
    pub r#standard_capacity_percent_above_base: Box<Option<i32>>,
}
