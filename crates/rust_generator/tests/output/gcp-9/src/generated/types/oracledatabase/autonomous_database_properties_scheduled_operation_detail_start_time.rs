#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutonomousDatabasePropertiesScheduledOperationDetailStartTime {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
    /// to allow the value "24:00:00" for scenarios like business closing time.
    #[builder(into, default)]
    #[serde(rename = "hours")]
    pub r#hours: Box<Option<i32>>,
    /// Minutes of hour of day. Must be from 0 to 59.
    #[builder(into, default)]
    #[serde(rename = "minutes")]
    pub r#minutes: Box<Option<i32>>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    #[builder(into, default)]
    #[serde(rename = "nanos")]
    pub r#nanos: Box<Option<i32>>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may
    /// allow the value 60 if it allows leap-seconds.
    #[builder(into, default)]
    #[serde(rename = "seconds")]
    pub r#seconds: Box<Option<i32>>,
}
