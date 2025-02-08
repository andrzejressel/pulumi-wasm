#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationFailureConditionalDefaultBranchNextStepDialogAction {
    /// If the dialog action is `ElicitSlot`, defines the slot to elicit from the user.
    #[builder(into, default)]
    #[serde(rename = "slotToElicit")]
    pub r#slot_to_elicit: Box<Option<String>>,
    /// Whether the next message for the intent is _not_ used.
    #[builder(into, default)]
    #[serde(rename = "suppressNextMessage")]
    pub r#suppress_next_message: Box<Option<bool>>,
    /// Action that the bot should execute. Valid values are `ElicitIntent`, `StartIntent`, `ElicitSlot`, `EvaluateConditional`, `InvokeDialogCodeHook`, `ConfirmIntent`, `FulfillIntent`, `CloseIntent`, `EndConversation`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
