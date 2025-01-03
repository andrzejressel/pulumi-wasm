#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPatchBaselineSource {
    /// Value of the yum repo configuration.
    #[builder(into)]
    #[serde(rename = "configuration")]
    pub r#configuration: Box<String>,
    /// Name specified to identify the patch source.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specific operating system versions a patch repository applies to.
    #[builder(into)]
    #[serde(rename = "products")]
    pub r#products: Box<Vec<String>>,
}
