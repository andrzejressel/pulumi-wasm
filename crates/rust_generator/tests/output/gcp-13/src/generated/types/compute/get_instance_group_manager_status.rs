#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceGroupManagerStatus {
    /// Status of all-instances configuration on the group.
    #[builder(into)]
    #[serde(rename = "allInstancesConfigs")]
    pub r#all_instances_configs: Box<Vec<super::super::types::compute::GetInstanceGroupManagerStatusAllInstancesConfig>>,
    /// A bit indicating whether the managed instance group is in a stable state. A stable state means that: none of the instances in the managed instance group is currently undergoing any type of change (for example, creation, restart, or deletion); no future changes are scheduled for instances in the managed instance group; and the managed instance group itself is not being modified.
    #[builder(into)]
    #[serde(rename = "isStable")]
    pub r#is_stable: Box<bool>,
    /// Stateful status of the given Instance Group Manager.
    #[builder(into)]
    #[serde(rename = "statefuls")]
    pub r#statefuls: Box<Vec<super::super::types::compute::GetInstanceGroupManagerStatusStateful>>,
    /// A status of consistency of Instances' versions with their target version specified by version field on Instance Group Manager.
    #[builder(into)]
    #[serde(rename = "versionTargets")]
    pub r#version_targets: Box<Vec<super::super::types::compute::GetInstanceGroupManagerStatusVersionTarget>>,
}
