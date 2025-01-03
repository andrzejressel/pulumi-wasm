#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NoteRelatedUrl {
    /// Label to describe usage of the URL
    #[builder(into, default)]
    #[serde(rename = "label")]
    pub r#label: Box<Option<String>>,
    /// Specific URL associated with the resource.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
