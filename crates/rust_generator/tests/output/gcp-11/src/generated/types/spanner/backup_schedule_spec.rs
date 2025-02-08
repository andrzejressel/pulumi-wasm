#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BackupScheduleSpec {
    /// Cron style schedule specification..
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cronSpec")]
    pub r#cron_spec: Box<Option<super::super::types::spanner::BackupScheduleSpecCronSpec>>,
}
