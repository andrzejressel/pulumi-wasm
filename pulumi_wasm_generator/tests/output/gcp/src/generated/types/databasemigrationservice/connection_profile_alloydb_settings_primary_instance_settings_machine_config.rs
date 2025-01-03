#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionProfileAlloydbSettingsPrimaryInstanceSettingsMachineConfig {
    /// The number of CPU's in the VM instance.
    #[builder(into)]
    #[serde(rename = "cpuCount")]
    pub r#cpu_count: Box<i32>,
}
