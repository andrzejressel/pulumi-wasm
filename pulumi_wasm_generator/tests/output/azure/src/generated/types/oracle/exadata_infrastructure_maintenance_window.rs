#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExadataInfrastructureMaintenanceWindow {
    /// Days during the week when maintenance should be performed. Valid values are: `0` - represents time slot `0:00 - 3:59 UTC - 4` - represents time slot `4:00 - 7:59 UTC - 8` - represents time slot 8:00 - 11:59 UTC - 12 - represents time slot 12:00 - 15:59 UTC - 16 - represents time slot 16:00 - 19:59 UTC - 20 - represents time slot `20:00 - 23:59 UTC`. Changing this forces a new Cloud Exadata Infrastructure to be created.
    #[builder(into, default)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Box<Option<Vec<String>>>,
    /// The window of hours during the day when maintenance should be performed. The window is a 4 hour slot. Changing this forces a new Cloud Exadata Infrastructure to be created.
    #[builder(into, default)]
    #[serde(rename = "hoursOfDays")]
    pub r#hours_of_days: Box<Option<Vec<i32>>>,
    /// Lead time window allows user to set a lead time to prepare for a down time. The lead time is in weeks and valid value is between `1` to `4`. Changing this forces a new Cloud Exadata Infrastructure to be created.
    #[builder(into, default)]
    #[serde(rename = "leadTimeInWeeks")]
    pub r#lead_time_in_weeks: Box<Option<i32>>,
    /// Months during the year when maintenance should be performed. Changing this forces a new Cloud Exadata Infrastructure to be created.
    #[builder(into, default)]
    #[serde(rename = "months")]
    pub r#months: Box<Option<Vec<String>>>,
    /// Cloud Exadata Infrastructure node patching method, either `ROLLING` or `NONROLLING`. Default value is `ROLLING`. IMPORTANT: Non-rolling infrastructure patching involves system down time. See [Oracle-Managed Infrastructure Maintenance Updates](https://docs.cloud.oracle.com/iaas/Content/Database/Concepts/examaintenance.htm#Oracle) for more information. Changing this forces a new Cloud Exadata Infrastructure to be created.
    #[builder(into, default)]
    #[serde(rename = "patchingMode")]
    pub r#patching_mode: Box<Option<String>>,
    /// The maintenance window scheduling preference. Changing this forces a new Cloud Exadata Infrastructure to be created.
    #[builder(into, default)]
    #[serde(rename = "preference")]
    pub r#preference: Box<Option<String>>,
    /// Weeks during the month when maintenance should be performed. Weeks start on the 1st, 8th, 15th, and 22nd days of the month, and have a duration of 7 days. Weeks start and end based on calendar dates, not days of the week. For example, to allow maintenance during the 2nd week of the month (from the 8th day to the 14th day of the month), use the value 2. Maintenance cannot be scheduled for the fifth week of months that contain more than 28 days. Note that this parameter works in conjunction with the daysOfWeek and hoursOfDay parameters to allow you to specify specific days of the week and hours that maintenance will be performed. Changing this forces a new Cloud Exadata Infrastructure to be created.
    #[builder(into, default)]
    #[serde(rename = "weeksOfMonths")]
    pub r#weeks_of_months: Box<Option<Vec<i32>>>,
}