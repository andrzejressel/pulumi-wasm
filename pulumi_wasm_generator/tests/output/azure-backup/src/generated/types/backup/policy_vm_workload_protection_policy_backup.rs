#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyVmWorkloadProtectionPolicyBackup {
    /// The backup frequency for the VM Workload Backup Policy. Possible values are `Daily` and `Weekly`.
    #[builder(into, default)]
    #[serde(rename = "frequency")]
    pub r#frequency: Box<Option<String>>,
    /// The backup frequency in minutes for the VM Workload Backup Policy. Possible values are `15`, `30`, `60`, `120`, `240`, `480`, `720` and `1440`.
    #[builder(into, default)]
    #[serde(rename = "frequencyInMinutes")]
    pub r#frequency_in_minutes: Box<Option<i32>>,
    /// The time of day to perform the backup in 24hour format.
    #[builder(into, default)]
    #[serde(rename = "time")]
    pub r#time: Box<Option<String>>,
    /// The days of the week to perform backups on. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` or `Saturday`. This is used when `frequency` is `Weekly`.
    #[builder(into, default)]
    #[serde(rename = "weekdays")]
    pub r#weekdays: Box<Option<Vec<String>>>,
}
