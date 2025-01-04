#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeploymentFrontendPublic {
    /// Specifies a list of Public IP Resource ID to this NGINX Deployment. Changing this forces a new NGINX Deployment to be created.
    #[builder(into, default)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Box<Option<Vec<String>>>,
}
