#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChatEngineChatEngineMetadata {
    /// (Output)
    /// The resource name of a Dialogflow agent, that this Chat Engine refers to.
    #[builder(into, default)]
    #[serde(rename = "dialogflowAgent")]
    pub r#dialogflow_agent: Box<Option<String>>,
}
