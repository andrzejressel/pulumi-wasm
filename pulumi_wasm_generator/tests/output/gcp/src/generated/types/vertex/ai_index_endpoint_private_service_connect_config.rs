#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiIndexEndpointPrivateServiceConnectConfig {
    /// If set to true, the IndexEndpoint is created without private service access.
    #[builder(into)]
    #[serde(rename = "enablePrivateServiceConnect")]
    pub r#enable_private_service_connect: Box<bool>,
    /// A list of Projects from which the forwarding rule will target the service attachment.
    #[builder(into, default)]
    #[serde(rename = "projectAllowlists")]
    pub r#project_allowlists: Box<Option<Vec<String>>>,
}
