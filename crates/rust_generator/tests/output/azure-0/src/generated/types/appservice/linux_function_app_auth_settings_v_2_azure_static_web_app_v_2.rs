#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxFunctionAppAuthSettingsV2AzureStaticWebAppV2 {
    /// The ID of the Client to use to authenticate with Azure Static Web App Authentication.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
}
