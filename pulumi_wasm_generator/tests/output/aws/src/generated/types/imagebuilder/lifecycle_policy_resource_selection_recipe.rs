#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LifecyclePolicyResourceSelectionRecipe {
    /// The name of an Image Builder recipe that the lifecycle policy uses for resource selection.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The version of the Image Builder recipe specified by the name field.
    #[builder(into)]
    #[serde(rename = "semanticVersion")]
    pub r#semantic_version: Box<String>,
}