#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConfigurationWindow {
    /// The duration of the maintenance window in HH:mm format.
    #[builder(into, default)]
    #[serde(rename = "duration")]
    pub r#duration: Box<Option<String>>,
    /// Effective expiration date of the maintenance window in YYYY-MM-DD hh:mm format.
    #[builder(into, default)]
    #[serde(rename = "expirationDateTime")]
    pub r#expiration_date_time: Box<Option<String>>,
    /// The rate at which a maintenance window is expected to recur. The rate can be expressed as daily, weekly, or monthly schedules.
    #[builder(into, default)]
    #[serde(rename = "recurEvery")]
    pub r#recur_every: Box<Option<String>>,
    /// Effective start date of the maintenance window in YYYY-MM-DD hh:mm format.
    #[builder(into)]
    #[serde(rename = "startDateTime")]
    pub r#start_date_time: Box<String>,
    /// The time zone for the maintenance window. A list of timezones can be obtained by executing [System.TimeZoneInfo]::GetSystemTimeZones() in PowerShell.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<String>,
}
