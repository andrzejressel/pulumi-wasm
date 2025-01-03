#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicy {
    /// List of instance selection options that the group will use when creating new VMs.
    #[builder(into, default)]
    #[serde(rename = "instanceSelectionLists")]
    pub r#instance_selection_lists: Box<Option<Vec<super::super::types::dataproc::ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicyInstanceSelectionList>>>,
    /// A list of instance selection results in the group.
    #[builder(into, default)]
    #[serde(rename = "instanceSelectionResults")]
    pub r#instance_selection_results: Box<Option<Vec<super::super::types::dataproc::ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicyInstanceSelectionResult>>>,
    /// Defines how Dataproc should create VMs with a mixture of provisioning models.
    #[builder(into, default)]
    #[serde(rename = "provisioningModelMix")]
    pub r#provisioning_model_mix: Box<Option<super::super::types::dataproc::ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicyProvisioningModelMix>>,
}
