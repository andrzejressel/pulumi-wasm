#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationProtectedFile {
    /// Specifies the base-64 encoded contents of this config file (Sensitive).
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Box<String>,
    /// Specifies the path of this config file.
    #[builder(into)]
    #[serde(rename = "virtualPath")]
    pub r#virtual_path: Box<String>,
}
