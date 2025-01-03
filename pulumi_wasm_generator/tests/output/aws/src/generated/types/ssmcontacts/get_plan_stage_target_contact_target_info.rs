#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPlanStageTargetContactTargetInfo {
    /// The Amazon Resource Name (ARN) of the contact or escalation plan.
    #[builder(into)]
    #[serde(rename = "contactId")]
    pub r#contact_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "isEssential")]
    pub r#is_essential: Box<bool>,
}
