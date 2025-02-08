#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ActivityLogAlertCriteriaServiceHealth {
    /// Events this alert will monitor Possible values are `Incident`, `Maintenance`, `Informational`, `ActionRequired` and `Security`.
    #[builder(into, default)]
    #[serde(rename = "events")]
    pub r#events: Box<Option<Vec<String>>>,
    /// Locations this alert will monitor. For example, `West Europe`.
    #[builder(into, default)]
    #[serde(rename = "locations")]
    pub r#locations: Box<Option<Vec<String>>>,
    /// Services this alert will monitor. For example, `Activity Logs & Alerts`, `Action Groups`. Defaults to all Services.
    #[builder(into, default)]
    #[serde(rename = "services")]
    pub r#services: Box<Option<Vec<String>>>,
}
