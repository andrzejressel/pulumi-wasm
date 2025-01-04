#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupPlanBackupScheduleRpoConfigExclusionWindowStartTime {
    /// Hours of day in 24 hour format.
    #[builder(into, default)]
    #[serde(rename = "hours")]
    pub r#hours: Box<Option<i32>>,
    /// Minutes of hour of day.
    #[builder(into, default)]
    #[serde(rename = "minutes")]
    pub r#minutes: Box<Option<i32>>,
    /// Fractions of seconds in nanoseconds.
    #[builder(into, default)]
    #[serde(rename = "nanos")]
    pub r#nanos: Box<Option<i32>>,
    /// Seconds of minutes of the time.
    #[builder(into, default)]
    #[serde(rename = "seconds")]
    pub r#seconds: Box<Option<i32>>,
}
