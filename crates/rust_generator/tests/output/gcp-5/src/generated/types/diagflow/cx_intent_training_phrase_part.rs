#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxIntentTrainingPhrasePart {
    /// The parameter used to annotate this part of the training phrase. This field is required for annotated parts of the training phrase.
    #[builder(into, default)]
    #[serde(rename = "parameterId")]
    pub r#parameter_id: Box<Option<String>>,
    /// The text for this part.
    #[builder(into)]
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}
