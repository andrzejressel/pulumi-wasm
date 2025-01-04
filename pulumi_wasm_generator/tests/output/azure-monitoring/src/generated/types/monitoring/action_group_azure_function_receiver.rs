#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ActionGroupAzureFunctionReceiver {
    /// The Azure resource ID of the function app.
    #[builder(into)]
    #[serde(rename = "functionAppResourceId")]
    pub r#function_app_resource_id: Box<String>,
    /// The function name in the function app.
    #[builder(into)]
    #[serde(rename = "functionName")]
    pub r#function_name: Box<String>,
    /// The HTTP trigger url where HTTP request sent to.
    #[builder(into)]
    #[serde(rename = "httpTriggerUrl")]
    pub r#http_trigger_url: Box<String>,
    /// The name of the Azure Function receiver.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Enables or disables the common alert schema.
    #[builder(into, default)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: Box<Option<bool>>,
}
