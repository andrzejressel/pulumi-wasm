#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ImageRecipeSystemsManagerAgent {
    /// Whether to remove the Systems Manager Agent after the image has been built.
    #[builder(into)]
    #[serde(rename = "uninstallAfterBuild")]
    pub r#uninstall_after_build: Box<bool>,
}