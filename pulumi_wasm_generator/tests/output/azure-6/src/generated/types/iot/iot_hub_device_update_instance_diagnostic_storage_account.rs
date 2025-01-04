#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IotHubDeviceUpdateInstanceDiagnosticStorageAccount {
    /// Connection String of the Diagnostic Storage Account.
    #[builder(into)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: Box<String>,
    /// Resource ID of the Diagnostic Storage Account.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
