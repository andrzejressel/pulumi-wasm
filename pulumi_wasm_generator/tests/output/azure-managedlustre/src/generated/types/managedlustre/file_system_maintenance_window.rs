#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FileSystemMaintenanceWindow {
    /// The day of the week on which the maintenance window will occur. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` and `Saturday`.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<String>,
    /// The time of day (in UTC) to start the maintenance window.
    #[builder(into)]
    #[serde(rename = "timeOfDayInUtc")]
    pub r#time_of_day_in_utc: Box<String>,
}
