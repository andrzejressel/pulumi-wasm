#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PlanStageTargetContactTargetInfo {
    /// The Amazon Resource Name (ARN) of the contact.
    #[builder(into, default)]
    #[serde(rename = "contactId")]
    pub r#contact_id: Box<Option<String>>,
    /// A Boolean value determining if the contact's acknowledgement stops the progress of stages in the plan.
    #[builder(into)]
    #[serde(rename = "isEssential")]
    pub r#is_essential: Box<bool>,
}