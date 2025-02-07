#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupInstanceRefreshPreferencesAlarmSpecification {
    /// List of Cloudwatch alarms. If any of these alarms goes into ALARM state, Instance Refresh is failed.
    #[builder(into, default)]
    #[serde(rename = "alarms")]
    pub r#alarms: Box<Option<Vec<String>>>,
}
