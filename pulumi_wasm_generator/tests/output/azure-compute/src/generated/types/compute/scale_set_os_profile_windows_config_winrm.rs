#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScaleSetOsProfileWindowsConfigWinrm {
    /// Specifies URL of the certificate with which new Virtual Machines is provisioned.
    #[builder(into, default)]
    #[serde(rename = "certificateUrl")]
    pub r#certificate_url: Box<Option<String>>,
    /// Specifies the protocol of listener
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
}