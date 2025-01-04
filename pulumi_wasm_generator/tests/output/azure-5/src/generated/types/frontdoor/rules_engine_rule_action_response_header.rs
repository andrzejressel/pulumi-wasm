#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RulesEngineRuleActionResponseHeader {
    /// can be set to `Overwrite`, `Append` or `Delete`.
    #[builder(into, default)]
    #[serde(rename = "headerActionType")]
    pub r#header_action_type: Box<Option<String>>,
    /// header name (string).
    #[builder(into, default)]
    #[serde(rename = "headerName")]
    pub r#header_name: Box<Option<String>>,
    /// value name (string).
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
