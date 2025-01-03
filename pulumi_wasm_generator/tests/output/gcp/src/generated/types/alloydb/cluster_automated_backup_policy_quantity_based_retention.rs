#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterAutomatedBackupPolicyQuantityBasedRetention {
    /// The number of backups to retain.
    #[builder(into, default)]
    #[serde(rename = "count")]
    pub r#count: Box<Option<i32>>,
}
