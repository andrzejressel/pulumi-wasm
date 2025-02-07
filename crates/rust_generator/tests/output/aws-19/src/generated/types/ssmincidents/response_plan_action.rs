#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResponsePlanAction {
    /// The Systems Manager automation document to start as the runbook at the beginning of the incident. The following values are supported:
    #[builder(into, default)]
    #[serde(rename = "ssmAutomations")]
    pub r#ssm_automations: Box<Option<Vec<super::super::types::ssmincidents::ResponsePlanActionSsmAutomation>>>,
}
