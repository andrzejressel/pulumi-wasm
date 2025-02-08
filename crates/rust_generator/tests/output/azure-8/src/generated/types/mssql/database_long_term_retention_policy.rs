#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseLongTermRetentionPolicy {
    #[builder(into, default)]
    #[serde(rename = "immutableBackupsEnabled")]
    pub r#immutable_backups_enabled: Box<Option<bool>>,
    /// The monthly retention policy for an LTR backup in an ISO 8601 format. Valid value is between 1 to 120 months. e.g. `P1Y`, `P1M`, `P4W` or `P30D`. Defaults to `PT0S`.
    #[builder(into, default)]
    #[serde(rename = "monthlyRetention")]
    pub r#monthly_retention: Box<Option<String>>,
    /// The week of year to take the yearly backup. Value has to be between `1` and `52`.
    #[builder(into, default)]
    #[serde(rename = "weekOfYear")]
    pub r#week_of_year: Box<Option<i32>>,
    /// The weekly retention policy for an LTR backup in an ISO 8601 format. Valid value is between 1 to 520 weeks. e.g. `P1Y`, `P1M`, `P1W` or `P7D`. Defaults to `PT0S`.
    #[builder(into, default)]
    #[serde(rename = "weeklyRetention")]
    pub r#weekly_retention: Box<Option<String>>,
    /// The yearly retention policy for an LTR backup in an ISO 8601 format. Valid value is between 1 to 10 years. e.g. `P1Y`, `P12M`, `P52W` or `P365D`. Defaults to `PT0S`.
    #[builder(into, default)]
    #[serde(rename = "yearlyRetention")]
    pub r#yearly_retention: Box<Option<String>>,
}
