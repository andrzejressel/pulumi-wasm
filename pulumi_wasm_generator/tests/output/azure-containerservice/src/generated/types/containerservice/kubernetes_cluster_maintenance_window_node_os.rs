#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterMaintenanceWindowNodeOs {
    /// The day of the month for the maintenance run. Required in combination with AbsoluteMonthly frequency. Value between 0 and 31 (inclusive).
    #[builder(into, default)]
    #[serde(rename = "dayOfMonth")]
    pub r#day_of_month: Box<Option<i32>>,
    /// The day of the week for the maintenance run. Required in combination with weekly frequency. Possible values are `Friday`, `Monday`, `Saturday`, `Sunday`, `Thursday`, `Tuesday` and `Wednesday`.
    #[builder(into, default)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<Option<String>>,
    /// The duration of the window for maintenance to run in hours. Possible options are between `4` to `24`.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<i32>,
    /// Frequency of maintenance. Possible options are `Daily`, `Weekly`, `AbsoluteMonthly` and `RelativeMonthly`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: Box<String>,
    /// The interval for maintenance runs. Depending on the frequency this interval is week or month based.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<i32>,
    /// One or more `not_allowed` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "notAlloweds")]
    pub r#not_alloweds: Box<Option<Vec<super::super::types::containerservice::KubernetesClusterMaintenanceWindowNodeOsNotAllowed>>>,
    /// The date on which the maintenance window begins to take effect.
    #[builder(into, default)]
    #[serde(rename = "startDate")]
    pub r#start_date: Box<Option<String>>,
    /// The time for maintenance to begin, based on the timezone determined by `utc_offset`. Format is `HH:mm`.
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
    /// Used to determine the timezone for cluster maintenance.
    #[builder(into, default)]
    #[serde(rename = "utcOffset")]
    pub r#utc_offset: Box<Option<String>>,
    /// The week in the month used for the maintenance run. Options are `First`, `Second`, `Third`, `Fourth`, and `Last`.
    #[builder(into, default)]
    #[serde(rename = "weekIndex")]
    pub r#week_index: Box<Option<String>>,
}
