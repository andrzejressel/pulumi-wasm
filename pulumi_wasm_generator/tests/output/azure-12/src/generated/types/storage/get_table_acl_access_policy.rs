#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTableAclAccessPolicy {
    #[builder(into)]
    #[serde(rename = "expiry")]
    pub r#expiry: Box<String>,
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<String>,
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Box<String>,
}
