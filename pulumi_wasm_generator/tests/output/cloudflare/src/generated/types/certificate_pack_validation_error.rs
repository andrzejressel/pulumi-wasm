#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificatePackValidationError {
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}
