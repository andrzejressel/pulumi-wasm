#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GlossaryTermTermRelations {
    /// String array that calssifies the term relations.
    #[builder(into, default)]
    #[serde(rename = "classifies")]
    pub r#classifies: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "isAs")]
    pub r#is_as: Box<Option<Vec<String>>>,
}