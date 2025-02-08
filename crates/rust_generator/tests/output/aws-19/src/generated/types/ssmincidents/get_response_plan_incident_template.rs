#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetResponsePlanIncidentTemplate {
    /// A string used to stop Incident Manager from creating multiple incident records for the same incident.
    #[builder(into)]
    #[serde(rename = "dedupeString")]
    pub r#dedupe_string: Box<String>,
    /// The impact value of a generated incident. The following values are supported:
    #[builder(into)]
    #[serde(rename = "impact")]
    pub r#impact: Box<i32>,
    /// The tags assigned to an incident template. When an incident starts, Incident Manager assigns the tags specified in the template to the incident.
    #[builder(into)]
    #[serde(rename = "incidentTags")]
    pub r#incident_tags: Box<std::collections::HashMap<String, String>>,
    /// The Amazon Simple Notification Service (Amazon SNS) targets that this incident notifies when it is updated. The `notification_target` configuration block supports the following argument:
    #[builder(into)]
    #[serde(rename = "notificationTargets")]
    pub r#notification_targets: Box<Vec<super::super::types::ssmincidents::GetResponsePlanIncidentTemplateNotificationTarget>>,
    /// The summary of an incident.
    #[builder(into)]
    #[serde(rename = "summary")]
    pub r#summary: Box<String>,
    /// The title of a generated incident.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Box<String>,
}
