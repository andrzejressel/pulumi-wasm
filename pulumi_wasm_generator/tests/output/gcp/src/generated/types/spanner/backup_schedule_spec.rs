#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupScheduleSpec {
    /// Cron style schedule specification..
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cronSpec")]
    pub r#cron_spec: Box<Option<super::super::types::spanner::BackupScheduleSpecCronSpec>>,
}
