#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsIntentSampleUtterance {
    /// Sample utterance that Amazon Lex uses to build its machine-learning model to recognize intents.
    #[builder(into)]
    #[serde(rename = "utterance")]
    pub r#utterance: Box<String>,
}
