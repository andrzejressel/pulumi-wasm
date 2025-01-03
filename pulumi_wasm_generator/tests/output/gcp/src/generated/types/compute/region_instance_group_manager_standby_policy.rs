#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionInstanceGroupManagerStandbyPolicy {
    /// Specifies the number of seconds that the MIG should wait to suspend or stop a VM after that VM was created. The initial delay gives the initialization script the time to prepare your VM for a quick scale out. The value of initial delay must be between 0 and 3600 seconds. The default value is 0.
    #[builder(into, default)]
    #[serde(rename = "initialDelaySec")]
    pub r#initial_delay_sec: Box<Option<i32>>,
    /// Defines how a MIG resumes or starts VMs from a standby pool when the group scales out. Valid options are: `MANUAL`, `SCALE_OUT_POOL`. If `MANUAL`(default), you have full control over which VMs are stopped and suspended in the MIG. If `SCALE_OUT_POOL`, the MIG uses the VMs from the standby pools to accelerate the scale out by resuming or starting them and then automatically replenishes the standby pool with new VMs to maintain the target sizes.
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}
