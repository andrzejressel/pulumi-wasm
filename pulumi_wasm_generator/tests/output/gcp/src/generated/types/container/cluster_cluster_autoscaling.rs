#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterAutoscaling {
    /// Contains defaults for a node pool created by NAP. A subset of fields also apply to
    /// GKE Autopilot clusters.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "autoProvisioningDefaults")]
    pub r#auto_provisioning_defaults: Box<Option<super::super::types::container::ClusterClusterAutoscalingAutoProvisioningDefaults>>,
    /// The list of Google Compute Engine 
    /// [zones](https://cloud.google.com/compute/docs/zones#available) in which the
    /// NodePool's nodes can be created by NAP.
    #[builder(into, default)]
    #[serde(rename = "autoProvisioningLocations")]
    pub r#auto_provisioning_locations: Box<Option<Vec<String>>>,
    /// Configuration
    /// options for the [Autoscaling profile](https://cloud.google.com/kubernetes-engine/docs/concepts/cluster-autoscaler#autoscaling_profiles)
    /// feature, which lets you choose whether the cluster autoscaler should optimize for resource utilization or resource availability
    /// when deciding to remove nodes from a cluster. Can be `BALANCED` or `OPTIMIZE_UTILIZATION`. Defaults to `BALANCED`.
    #[builder(into, default)]
    #[serde(rename = "autoscalingProfile")]
    pub r#autoscaling_profile: Box<Option<String>>,
    /// Whether node auto-provisioning is enabled. Must be supplied for GKE Standard clusters, `true` is implied
    /// for autopilot clusters. Resource limits for `cpu` and `memory` must be defined to enable node auto-provisioning for GKE Standard.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Global constraints for machine resources in the
    /// cluster. Configuring the `cpu` and `memory` types is required if node
    /// auto-provisioning is enabled. These limits will apply to node pool autoscaling
    /// in addition to node auto-provisioning. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "resourceLimits")]
    pub r#resource_limits: Box<Option<Vec<super::super::types::container::ClusterClusterAutoscalingResourceLimit>>>,
}
