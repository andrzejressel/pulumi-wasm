#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BackupPolicyMysqlFlexibleServerRetentionRuleCriteria {
    /// Possible values are `AllBackup`, `FirstOfDay`, `FirstOfWeek`, `FirstOfMonth` and `FirstOfYear`. These values mean the first successful backup of the day/week/month/year. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "absoluteCriteria")]
    pub r#absolute_criteria: Box<Option<String>>,
    /// Possible values are `Monday`, `Tuesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Box<Option<Vec<String>>>,
    /// Possible values are `January`, `February`, `March`, `April`, `May`, `June`, `July`, `August`, `September`, `October`, `November` and `December`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "monthsOfYears")]
    pub r#months_of_years: Box<Option<Vec<String>>>,
    /// Specifies a list of backup times for backup in the `RFC3339` format. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "scheduledBackupTimes")]
    pub r#scheduled_backup_times: Box<Option<Vec<String>>>,
    /// Possible values are `First`, `Second`, `Third`, `Fourth` and `Last`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "weeksOfMonths")]
    pub r#weeks_of_months: Box<Option<Vec<String>>>,
}
