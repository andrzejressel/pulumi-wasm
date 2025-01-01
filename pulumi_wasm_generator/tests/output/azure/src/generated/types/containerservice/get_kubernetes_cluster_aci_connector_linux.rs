#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKubernetesClusterAciConnectorLinux {
    /// The subnet name for the virtual nodes to run.
    #[builder(into)]
    #[serde(rename = "subnetName")]
    pub r#subnet_name: Box<String>,
}
