#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AutoscaleSettingProfileFixedDate {
    /// Specifies the end date for the profile, formatted as an RFC3339 date string.
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: Box<String>,
    /// Specifies the start date for the profile, formatted as an RFC3339 date string.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Box<String>,
    /// The Time Zone of the `start` and `end` times. A list of [possible values can be found here](https://learn.microsoft.com/en-us/rest/api/monitor/autoscale-settings/create-or-update?view=rest-monitor-2022-10-01&tabs=HTTP#recurrentschedule). Defaults to `UTC`.
    #[builder(into, default)]
    #[serde(rename = "timezone")]
    pub r#timezone: Box<Option<String>>,
}
