#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DicomServiceAuthentication {
    /// The intended audience to receive authentication tokens for the service. The default value is <https://dicom.azurehealthcareapis.azure.com>
    #[builder(into, default)]
    #[serde(rename = "audiences")]
    pub r#audiences: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "authority")]
    pub r#authority: Box<Option<String>>,
}