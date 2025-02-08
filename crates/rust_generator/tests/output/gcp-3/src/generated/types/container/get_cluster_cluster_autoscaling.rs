#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterClusterAutoscaling {
    /// Contains defaults for a node pool created by NAP.
    #[builder(into)]
    #[serde(rename = "autoProvisioningDefaults")]
    pub r#auto_provisioning_defaults: Box<Vec<super::super::types::container::GetClusterClusterAutoscalingAutoProvisioningDefault>>,
    /// The list of Google Compute Engine zones in which the NodePool's nodes can be created by NAP.
    #[builder(into)]
    #[serde(rename = "autoProvisioningLocations")]
    pub r#auto_provisioning_locations: Box<Vec<String>>,
    /// Configuration options for the Autoscaling profile feature, which lets you choose whether the cluster autoscaler should optimize for resource utilization or resource availability when deciding to remove nodes from a cluster. Can be BALANCED or OPTIMIZE_UTILIZATION. Defaults to BALANCED.
    #[builder(into)]
    #[serde(rename = "autoscalingProfile")]
    pub r#autoscaling_profile: Box<String>,
    /// Whether node auto-provisioning is enabled. Resource limits for cpu and memory must be defined to enable node auto-provisioning.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Global constraints for machine resources in the cluster. Configuring the cpu and memory types is required if node auto-provisioning is enabled. These limits will apply to node pool autoscaling in addition to node auto-provisioning.
    #[builder(into)]
    #[serde(rename = "resourceLimits")]
    pub r#resource_limits: Box<Vec<super::super::types::container::GetClusterClusterAutoscalingResourceLimit>>,
}
