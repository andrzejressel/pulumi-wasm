#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyVmWorkloadSettings {
    /// The compression setting for the VM Workload Backup Policy. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "compressionEnabled")]
    pub r#compression_enabled: Box<Option<bool>>,
    /// The timezone for the VM Workload Backup Policy. [The possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/).
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<String>,
}