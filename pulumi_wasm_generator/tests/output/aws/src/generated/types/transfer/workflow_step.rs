#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowStep {
    /// Details for a step that performs a file copy. See Copy Step Details below.
    #[builder(into, default)]
    #[serde(rename = "copyStepDetails")]
    pub r#copy_step_details: Box<Option<super::super::types::transfer::WorkflowStepCopyStepDetails>>,
    /// Details for a step that invokes a lambda function.
    #[builder(into, default)]
    #[serde(rename = "customStepDetails")]
    pub r#custom_step_details: Box<Option<super::super::types::transfer::WorkflowStepCustomStepDetails>>,
    /// Details for a step that decrypts the file.
    #[builder(into, default)]
    #[serde(rename = "decryptStepDetails")]
    pub r#decrypt_step_details: Box<Option<super::super::types::transfer::WorkflowStepDecryptStepDetails>>,
    /// Details for a step that deletes the file.
    #[builder(into, default)]
    #[serde(rename = "deleteStepDetails")]
    pub r#delete_step_details: Box<Option<super::super::types::transfer::WorkflowStepDeleteStepDetails>>,
    /// Details for a step that creates one or more tags.
    #[builder(into, default)]
    #[serde(rename = "tagStepDetails")]
    pub r#tag_step_details: Box<Option<super::super::types::transfer::WorkflowStepTagStepDetails>>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
