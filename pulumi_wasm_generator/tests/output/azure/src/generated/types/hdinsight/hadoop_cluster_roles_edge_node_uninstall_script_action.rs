#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HadoopClusterRolesEdgeNodeUninstallScriptAction {
    /// The name of the uninstall script action.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The parameters for the script.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<String>>,
    /// The URI pointing to the script to run during the installation of the edge node.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}