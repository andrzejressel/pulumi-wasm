#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecListenerTlCertificateFile {
    #[builder(into)]
    #[serde(rename = "certificateChain")]
    pub r#certificate_chain: Box<String>,
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<String>,
}