#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetManagedPrefixListEntry {
    #[builder(into)]
    #[serde(rename = "cidr")]
    pub r#cidr: Box<String>,
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
}