#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PatchDeploymentRecurringScheduleTimeZone {
    /// IANA Time Zone Database time zone, e.g. "America/New_York".
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// IANA Time Zone Database version number, e.g. "2019a".
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
