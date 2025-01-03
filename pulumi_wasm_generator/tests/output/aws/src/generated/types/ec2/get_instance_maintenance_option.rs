#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceMaintenanceOption {
    /// Automatic recovery behavior of the instance.
    #[builder(into)]
    #[serde(rename = "autoRecovery")]
    pub r#auto_recovery: Box<String>,
}
