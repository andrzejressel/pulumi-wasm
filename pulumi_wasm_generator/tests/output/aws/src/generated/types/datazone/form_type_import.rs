#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FormTypeImport {
    /// Name of the form type. Must be the name of the structure in smithy document.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Revision of the Form Type.
    #[builder(into)]
    #[serde(rename = "revision")]
    pub r#revision: Box<String>,
}