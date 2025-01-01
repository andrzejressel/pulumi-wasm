#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRegionInstanceGroupManagerStandbyPolicy {
    /// Specifies the number of seconds that the MIG should wait to suspend or stop a VM after that VM was created. The initial delay gives the initialization script the time to prepare your VM for a quick scale out. The value of initial delay must be between 0 and 3600 seconds. The default value is 0.
    #[builder(into)]
    #[serde(rename = "initialDelaySec")]
    pub r#initial_delay_sec: Box<i32>,
    /// Defines how a MIG resumes or starts VMs from a standby pool when the group scales out. The default mode is "MANUAL".
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
