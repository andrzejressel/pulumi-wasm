#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeaturesTemplateDeployment {
    #[builder(into)]
    #[serde(rename = "deleteNestedItemsDuringDeletion")]
    pub r#delete_nested_items_during_deletion: Box<bool>,
}
