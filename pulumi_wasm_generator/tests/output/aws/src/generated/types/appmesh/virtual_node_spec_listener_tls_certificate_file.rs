#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecListenerTlsCertificateFile {
    /// Certificate trust chain for a certificate stored on the file system of the mesh endpoint that the proxy is running on. Must be between 1 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "certificateChain")]
    pub r#certificate_chain: Box<String>,
    /// Private key for a certificate stored on the file system of the virtual node that the proxy is running on. Must be between 1 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<String>,
}
