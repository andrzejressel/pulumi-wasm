#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyExplicitProxy {
    /// Whether the pac file port and url need to be provided.
    #[builder(into, default)]
    #[serde(rename = "enablePacFile")]
    pub r#enable_pac_file: Box<Option<bool>>,
    /// Whether the explicit proxy is enabled for this Firewall Policy.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The port number for explicit http protocol.
    #[builder(into, default)]
    #[serde(rename = "httpPort")]
    pub r#http_port: Box<Option<i32>>,
    /// The port number for explicit proxy https protocol.
    #[builder(into, default)]
    #[serde(rename = "httpsPort")]
    pub r#https_port: Box<Option<i32>>,
    /// Specifies a SAS URL for PAC file.
    #[builder(into, default)]
    #[serde(rename = "pacFile")]
    pub r#pac_file: Box<Option<String>>,
    /// Specifies a port number for firewall to serve PAC file.
    #[builder(into, default)]
    #[serde(rename = "pacFilePort")]
    pub r#pac_file_port: Box<Option<i32>>,
}
