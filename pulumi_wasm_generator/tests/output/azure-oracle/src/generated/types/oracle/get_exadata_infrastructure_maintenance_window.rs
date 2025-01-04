#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetExadataInfrastructureMaintenanceWindow {
    /// If true, enables the configuration of a custom action timeout (waiting period) between database servers patching operations.
    #[builder(into)]
    #[serde(rename = "customActionTimeoutEnabled")]
    pub r#custom_action_timeout_enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "customActionTimeoutInMins")]
    pub r#custom_action_timeout_in_mins: Box<i32>,
    /// Days during the week when maintenance should be performed.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Box<Vec<String>>,
    /// The window of hours during the day when maintenance should be performed.
    #[builder(into)]
    #[serde(rename = "hoursOfDays")]
    pub r#hours_of_days: Box<Vec<i32>>,
    /// Lead time window allows user to set a lead time to prepare for a down time.
    #[builder(into)]
    #[serde(rename = "leadTimeInWeeks")]
    pub r#lead_time_in_weeks: Box<i32>,
    /// If true, enables the monthly patching option.
    #[builder(into)]
    #[serde(rename = "monthlyPatchingEnabled")]
    pub r#monthly_patching_enabled: Box<bool>,
    /// A `months` block as defined below.
    #[builder(into)]
    #[serde(rename = "months")]
    pub r#months: Box<Vec<String>>,
    /// Cloud Exadata Infrastructure node patching method.
    #[builder(into)]
    #[serde(rename = "patchingMode")]
    pub r#patching_mode: Box<String>,
    /// The maintenance window scheduling preference.
    #[builder(into)]
    #[serde(rename = "preference")]
    pub r#preference: Box<String>,
    /// Weeks during the month when maintenance should be performed.
    #[builder(into)]
    #[serde(rename = "weeksOfMonths")]
    pub r#weeks_of_months: Box<Vec<i32>>,
}
