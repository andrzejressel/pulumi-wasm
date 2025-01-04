#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionInstanceGroupManagerInstanceLifecyclePolicy {
    /// , Default behavior for all instance or health check failures. Valid options are: `REPAIR`, `DO_NOTHING`. If `DO_NOTHING` then instances will not be repaired. If `REPAIR` (default), then failed instances will be repaired.
    /// 
    /// - - -
    /// <a name="nested_instance_flexibility_policy"></a>The `instance_flexibility_policy` block supports:
    /// 
    #[builder(into, default)]
    #[serde(rename = "defaultActionOnFailure")]
    pub r#default_action_on_failure: Box<Option<String>>,
    /// , Specifies whether to apply the group's latest configuration when repairing a VM. Valid options are: `YES`, `NO`. If `YES` and you updated the group's instance template or per-instance configurations after the VM was created, then these changes are applied when VM is repaired. If `NO` (default), then updates are applied in accordance with the group's update policy type.
    #[builder(into, default)]
    #[serde(rename = "forceUpdateOnRepair")]
    pub r#force_update_on_repair: Box<Option<String>>,
}
