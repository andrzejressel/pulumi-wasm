#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EngineModelDefaultVersion {
    /// The name specified for the version when it was created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
