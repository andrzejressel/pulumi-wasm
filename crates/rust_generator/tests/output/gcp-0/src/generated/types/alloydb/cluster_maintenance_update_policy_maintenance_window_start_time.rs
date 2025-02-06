#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterMaintenanceUpdatePolicyMaintenanceWindowStartTime {
    /// Hours of day in 24 hour format. Should be from 0 to 23.
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: Box<i32>,
    /// Minutes of hour of day. Currently, only the value 0 is supported.
    #[builder(into, default)]
    #[serde(rename = "minutes")]
    pub r#minutes: Box<Option<i32>>,
    /// Fractions of seconds in nanoseconds. Currently, only the value 0 is supported.
    #[builder(into, default)]
    #[serde(rename = "nanos")]
    pub r#nanos: Box<Option<i32>>,
    /// Seconds of minutes of the time. Currently, only the value 0 is supported.
    #[builder(into, default)]
    #[serde(rename = "seconds")]
    pub r#seconds: Box<Option<i32>>,
}
