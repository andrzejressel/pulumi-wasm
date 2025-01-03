#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceGroupManagerStatus {
    /// Properties to set on all instances in the group. After setting
    /// allInstancesConfig on the group, you must update the group's instances to
    /// apply the configuration.
    #[builder(into, default)]
    #[serde(rename = "allInstancesConfigs")]
    pub r#all_instances_configs: Box<Option<Vec<super::super::types::compute::InstanceGroupManagerStatusAllInstancesConfig>>>,
    /// A bit indicating whether the managed instance group is in a stable state. A stable state means that: none of the instances in the managed instance group is currently undergoing any type of change (for example, creation, restart, or deletion); no future changes are scheduled for instances in the managed instance group; and the managed instance group itself is not being modified.
    #[builder(into, default)]
    #[serde(rename = "isStable")]
    pub r#is_stable: Box<Option<bool>>,
    /// Stateful status of the given Instance Group Manager.
    #[builder(into, default)]
    #[serde(rename = "statefuls")]
    pub r#statefuls: Box<Option<Vec<super::super::types::compute::InstanceGroupManagerStatusStateful>>>,
    /// A bit indicating whether version target has been reached in this managed instance group, i.e. all instances are in their target version. Instances' target version are specified by version field on Instance Group Manager.
    #[builder(into, default)]
    #[serde(rename = "versionTargets")]
    pub r#version_targets: Box<Option<Vec<super::super::types::compute::InstanceGroupManagerStatusVersionTarget>>>,
}
