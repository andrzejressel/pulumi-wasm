#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsFunctionAppBackup {
    /// Should this backup job be enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The name which should be used for this Backup.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A `schedule` block as defined below.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<super::super::types::appservice::WindowsFunctionAppBackupSchedule>,
    /// The SAS URL to the container.
    #[builder(into)]
    #[serde(rename = "storageAccountUrl")]
    pub r#storage_account_url: Box<String>,
}
