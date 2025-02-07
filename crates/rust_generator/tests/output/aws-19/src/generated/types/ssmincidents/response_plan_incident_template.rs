#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResponsePlanIncidentTemplate {
    /// A string used to stop Incident Manager from creating multiple incident records for the same incident.
    #[builder(into, default)]
    #[serde(rename = "dedupeString")]
    pub r#dedupe_string: Box<Option<String>>,
    /// The impact value of a generated incident. The following values are supported:
    #[builder(into)]
    #[serde(rename = "impact")]
    pub r#impact: Box<i32>,
    /// The tags assigned to an incident template. When an incident starts, Incident Manager assigns the tags specified in the template to the incident.
    #[builder(into, default)]
    #[serde(rename = "incidentTags")]
    pub r#incident_tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// The Amazon Simple Notification Service (Amazon SNS) targets that this incident notifies when it is updated. The `notification_target` configuration block supports the following argument:
    #[builder(into, default)]
    #[serde(rename = "notificationTargets")]
    pub r#notification_targets: Box<Option<Vec<super::super::types::ssmincidents::ResponsePlanIncidentTemplateNotificationTarget>>>,
    /// The summary of an incident.
    #[builder(into, default)]
    #[serde(rename = "summary")]
    pub r#summary: Box<Option<String>>,
    /// The title of a generated incident.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Box<String>,
}
