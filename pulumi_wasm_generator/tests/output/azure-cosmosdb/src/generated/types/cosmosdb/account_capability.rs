#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountCapability {
    /// Specifies the name of the CosmosDB Account. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}