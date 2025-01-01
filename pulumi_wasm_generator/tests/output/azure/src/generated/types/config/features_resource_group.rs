#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeaturesResourceGroup {
    #[builder(into, default)]
    #[serde(rename = "preventDeletionIfContainsResources")]
    pub r#prevent_deletion_if_contains_resources: Box<Option<bool>>,
}
