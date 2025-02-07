#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertRuleScheduledIncident {
    /// Whether to create an incident from alerts triggered by this Sentinel Scheduled Alert Rule?
    #[builder(into)]
    #[serde(rename = "createIncidentEnabled")]
    pub r#create_incident_enabled: Box<bool>,
    /// A `grouping` block as defined below.
    #[builder(into)]
    #[serde(rename = "grouping")]
    pub r#grouping: Box<super::super::types::sentinel::AlertRuleScheduledIncidentGrouping>,
}
