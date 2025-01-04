#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutoscaleSettingNotification {
    /// A `email` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<super::super::types::monitoring::AutoscaleSettingNotificationEmail>>,
    /// One or more `webhook` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "webhooks")]
    pub r#webhooks: Box<Option<Vec<super::super::types::monitoring::AutoscaleSettingNotificationWebhook>>>,
}
