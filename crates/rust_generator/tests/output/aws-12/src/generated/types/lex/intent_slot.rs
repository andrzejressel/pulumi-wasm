#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct IntentSlot {
    /// A description of the bot. Must be less than or equal to 200 characters in length.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The name of the intent slot that you want to create. The name is case sensitive. Must be less than or equal to 100 characters in length.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Directs Lex the order in which to elicit this slot value from the user.
    /// For example, if the intent has two slots with priorities 1 and 2, AWS Lex first elicits a value for
    /// the slot with priority 1. If multiple slots share the same priority, the order in which Lex elicits
    /// values is arbitrary. Must be between 1 and 100.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    /// The response card. Amazon Lex will substitute session attributes and
    /// slot values into the response card. For more information, see
    /// [Example: Using a Response Card](https://docs.aws.amazon.com/lex/latest/dg/ex-resp-card.html). Must be less than or equal to 50000 characters in length.
    #[builder(into, default)]
    #[serde(rename = "responseCard")]
    pub r#response_card: Box<Option<String>>,
    /// If you know a specific pattern with which users might respond to
    /// an Amazon Lex request for a slot value, you can provide those utterances to improve accuracy. This
    /// is optional. In most cases, Amazon Lex is capable of understanding user utterances. Must have between 1 and 10 items in the list, and each item must be less than or equal to 200 characters in length.
    #[builder(into, default)]
    #[serde(rename = "sampleUtterances")]
    pub r#sample_utterances: Box<Option<Vec<String>>>,
    /// Specifies whether the slot is required or optional.
    #[builder(into)]
    #[serde(rename = "slotConstraint")]
    pub r#slot_constraint: Box<String>,
    /// The type of the slot, either a custom slot type that you defined or one of
    /// the built-in slot types. Must be less than or equal to 100 characters in length.
    #[builder(into)]
    #[serde(rename = "slotType")]
    pub r#slot_type: Box<String>,
    /// The version of the slot type. Must be less than or equal to 64 characters in length.
    #[builder(into, default)]
    #[serde(rename = "slotTypeVersion")]
    pub r#slot_type_version: Box<Option<String>>,
    /// The prompt that Amazon Lex uses to elicit the slot value
    /// from the user. Attributes are documented under prompt.
    #[builder(into, default)]
    #[serde(rename = "valueElicitationPrompt")]
    pub r#value_elicitation_prompt: Box<Option<super::super::types::lex::IntentSlotValueElicitationPrompt>>,
}
