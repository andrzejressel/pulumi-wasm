#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetCloudExadataInfrastructurePropertyMaintenanceWindow {
    /// Determines the amount of time the system will wait before the start of each
    /// database server patching operation. Custom action timeout is in minutes and
    /// valid value is between 15 to 120 (inclusive).
    #[builder(into)]
    #[serde(rename = "customActionTimeoutMins")]
    pub r#custom_action_timeout_mins: Box<i32>,
    /// Days during the week when maintenance should be performed.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Box<Vec<String>>,
    /// The window of hours during the day when maintenance should be performed.
    /// The window is a 4 hour slot. Valid values are:
    ///   0 - represents time slot 0:00 - 3:59 UTC
    ///   4 - represents time slot 4:00 - 7:59 UTC
    ///   8 - represents time slot 8:00 - 11:59 UTC
    ///   12 - represents time slot 12:00 - 15:59 UTC
    ///   16 - represents time slot 16:00 - 19:59 UTC
    ///   20 - represents time slot 20:00 - 23:59 UTC
    #[builder(into)]
    #[serde(rename = "hoursOfDays")]
    pub r#hours_of_days: Box<Vec<i32>>,
    /// If true, enables the configuration of a custom action timeout (waiting
    /// period) between database server patching operations.
    #[builder(into)]
    #[serde(rename = "isCustomActionTimeoutEnabled")]
    pub r#is_custom_action_timeout_enabled: Box<bool>,
    /// Lead time window allows user to set a lead time to prepare for a down time.
    /// The lead time is in weeks and valid value is between 1 to 4.
    #[builder(into)]
    #[serde(rename = "leadTimeWeek")]
    pub r#lead_time_week: Box<i32>,
    /// Months during the year when maintenance should be performed.
    #[builder(into)]
    #[serde(rename = "months")]
    pub r#months: Box<Vec<String>>,
    /// Cloud CloudExadataInfrastructure node patching method, either "ROLLING"
    ///  or "NONROLLING". Default value is ROLLING. 
    ///  Possible values:
    ///  PATCHING_MODE_UNSPECIFIED
    /// ROLLING
    /// NON_ROLLING
    #[builder(into)]
    #[serde(rename = "patchingMode")]
    pub r#patching_mode: Box<String>,
    /// The maintenance window scheduling preference. 
    ///  Possible values:
    ///  MAINTENANCE_WINDOW_PREFERENCE_UNSPECIFIED
    /// CUSTOM_PREFERENCE
    /// NO_PREFERENCE
    #[builder(into)]
    #[serde(rename = "preference")]
    pub r#preference: Box<String>,
    /// Weeks during the month when maintenance should be performed. Weeks start on
    /// the 1st, 8th, 15th, and 22nd days of the month, and have a duration of 7
    /// days. Weeks start and end based on calendar dates, not days of the week.
    #[builder(into)]
    #[serde(rename = "weeksOfMonths")]
    pub r#weeks_of_months: Box<Vec<i32>>,
}
