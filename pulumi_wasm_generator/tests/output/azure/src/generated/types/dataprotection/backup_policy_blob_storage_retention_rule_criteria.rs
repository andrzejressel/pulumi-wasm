#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupPolicyBlobStorageRetentionRuleCriteria {
    /// Possible values are `AllBackup`, `FirstOfDay`, `FirstOfWeek`, `FirstOfMonth` and `FirstOfYear`. These values mean the first successful backup of the day/week/month/year. Changing this forces a new Backup Policy Blob Storage to be created.
    #[builder(into, default)]
    #[serde(rename = "absoluteCriteria")]
    pub r#absolute_criteria: Box<Option<String>>,
    /// Must be between `0` and `28`. `0` for last day within the month. Changing this forces a new Backup Policy Blob Storage to be created.
    #[builder(into, default)]
    #[serde(rename = "daysOfMonths")]
    pub r#days_of_months: Box<Option<Vec<i32>>>,
    /// Possible values are `Monday`, `Tuesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`. Changing this forces a new Backup Policy Blob Storage to be created.
    #[builder(into, default)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Box<Option<Vec<String>>>,
    /// Possible values are `January`, `February`, `March`, `April`, `May`, `June`, `July`, `August`, `September`, `October`, `November` and `December`. Changing this forces a new Backup Policy Blob Storage to be created.
    #[builder(into, default)]
    #[serde(rename = "monthsOfYears")]
    pub r#months_of_years: Box<Option<Vec<String>>>,
    /// Specifies a list of backup times for backup in the `RFC3339` format. Changing this forces a new Backup Policy Blob Storage to be created.
    #[builder(into, default)]
    #[serde(rename = "scheduledBackupTimes")]
    pub r#scheduled_backup_times: Box<Option<Vec<String>>>,
    /// Possible values are `First`, `Second`, `Third`, `Fourth` and `Last`. Changing this forces a new Backup Policy Blob Storage to be created.
    #[builder(into, default)]
    #[serde(rename = "weeksOfMonths")]
    pub r#weeks_of_months: Box<Option<Vec<String>>>,
}