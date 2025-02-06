#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecServiceDiscoveryAwsCloudMap {
    #[builder(into)]
    #[serde(rename = "attributes")]
    pub r#attributes: Box<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "namespaceName")]
    pub r#namespace_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Box<String>,
}
