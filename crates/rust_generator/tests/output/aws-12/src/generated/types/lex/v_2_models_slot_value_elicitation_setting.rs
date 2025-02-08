#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsSlotValueElicitationSetting {
    /// List of default values for a slot.
    /// See the `default_value_specification` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "defaultValueSpecifications")]
    pub r#default_value_specifications: Box<Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingDefaultValueSpecification>>>,
    /// Prompt that Amazon Lex uses to elicit the slot value from the user.
    /// See the `aws.lex.V2modelsIntent` resource for details on the `prompt_specification` argument reference - they are identical.
    #[builder(into)]
    #[serde(rename = "promptSpecification")]
    pub r#prompt_specification: Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecification>,
    #[builder(into, default)]
    #[serde(rename = "sampleUtterances")]
    pub r#sample_utterances: Box<Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingSampleUtterance>>>,
    /// Whether the slot is required or optional. Valid values are `Required` or `Optional`.
    #[builder(into)]
    #[serde(rename = "slotConstraint")]
    pub r#slot_constraint: Box<String>,
    /// Information about whether assisted slot resolution is turned on for the slot or not.
    /// See the `slot_resolution_setting` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "slotResolutionSettings")]
    pub r#slot_resolution_settings: Box<Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingSlotResolutionSetting>>>,
    /// Specifies the prompts that Amazon Lex uses while a bot is waiting for customer input.
    /// See the `wait_and_continue_specification` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "waitAndContinueSpecifications")]
    pub r#wait_and_continue_specifications: Box<Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecification>>>,
}
