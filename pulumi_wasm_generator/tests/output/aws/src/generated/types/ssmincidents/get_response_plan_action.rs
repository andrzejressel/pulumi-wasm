#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetResponsePlanAction {
    /// The Systems Manager automation document to start as the runbook at the beginning of the incident. The following values are supported:
    #[builder(into)]
    #[serde(rename = "ssmAutomations")]
    pub r#ssm_automations: Box<Vec<super::super::types::ssmincidents::GetResponsePlanActionSsmAutomation>>,
}