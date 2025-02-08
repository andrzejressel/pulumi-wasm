#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PolicyVmWorkloadProtectionPolicyRetentionMonthly {
    /// The number of monthly backups to keep. Must be between `1` and `1188`.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// The retention schedule format type for monthly retention policy. Possible values are `Daily` and `Weekly`.
    #[builder(into)]
    #[serde(rename = "formatType")]
    pub r#format_type: Box<String>,
    /// The monthday backups to retain. Possible values are between `0` and `28`.
    #[builder(into, default)]
    #[serde(rename = "monthdays")]
    pub r#monthdays: Box<Option<Vec<i32>>>,
    /// The weekday backups to retain. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` or `Saturday`.
    #[builder(into, default)]
    #[serde(rename = "weekdays")]
    pub r#weekdays: Box<Option<Vec<String>>>,
    /// The weeks of the month to retain backups of. Possible values are `First`, `Second`, `Third`, `Fourth` and `Last`.
    #[builder(into, default)]
    #[serde(rename = "weeks")]
    pub r#weeks: Box<Option<Vec<String>>>,
}
