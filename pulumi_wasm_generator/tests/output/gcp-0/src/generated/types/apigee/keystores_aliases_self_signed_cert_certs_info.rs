#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KeystoresAliasesSelfSignedCertCertsInfo {
    /// (Output)
    /// List of all properties in the object.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "certInfos")]
    pub r#cert_infos: Box<Option<Vec<super::super::types::apigee::KeystoresAliasesSelfSignedCertCertsInfoCertInfo>>>,
}
