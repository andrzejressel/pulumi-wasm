#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowAccessControlTriggerOpenAuthenticationPolicy {
    /// A `claim` block as defined below.
    #[builder(into)]
    #[serde(rename = "claims")]
    pub r#claims: Box<Vec<super::super::types::logicapps::WorkflowAccessControlTriggerOpenAuthenticationPolicyClaim>>,
    /// The OAuth policy name for the Logic App Workflow.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
