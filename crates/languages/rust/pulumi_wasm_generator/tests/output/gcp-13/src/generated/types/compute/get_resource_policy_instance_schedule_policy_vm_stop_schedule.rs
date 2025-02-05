#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetResourcePolicyInstanceSchedulePolicyVmStopSchedule {
    /// Specifies the frequency for the operation, using the unix-cron format.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<String>,
}
