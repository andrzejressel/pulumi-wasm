#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ActionGroupLogicAppReceiver {
    /// The callback url where HTTP request sent to.
    #[builder(into)]
    #[serde(rename = "callbackUrl")]
    pub r#callback_url: Box<String>,
    /// The name of the logic app receiver.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Azure resource ID of the logic app.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Box<String>,
    /// Enables or disables the common alert schema.
    #[builder(into, default)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: Box<Option<bool>>,
}
