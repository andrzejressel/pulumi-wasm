#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MultiplexProgramMultiplexProgramSettingsServiceDescriptor {
    /// Unique provider name.
    #[builder(into)]
    #[serde(rename = "providerName")]
    pub r#provider_name: Box<String>,
    /// Unique service name.
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Box<String>,
}
