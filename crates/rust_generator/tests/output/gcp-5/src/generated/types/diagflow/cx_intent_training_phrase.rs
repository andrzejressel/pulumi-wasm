#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CxIntentTrainingPhrase {
    /// (Output)
    /// The unique identifier of the training phrase.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase.
    /// Note: The API does not automatically annotate training phrases like the Dialogflow Console does.
    /// Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated.
    /// If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set.
    /// If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways:
    /// Part.text is set to a part of the phrase that has no parameters.
    /// Part.text is set to a part of the phrase that you want to annotate, and the parameterId field is set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "parts")]
    pub r#parts: Box<Vec<super::super::types::diagflow::CxIntentTrainingPhrasePart>>,
    /// Indicates how many times this example was added to the intent.
    #[builder(into, default)]
    #[serde(rename = "repeatCount")]
    pub r#repeat_count: Box<Option<i32>>,
}
