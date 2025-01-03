#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct QuickConnectQuickConnectConfig {
    /// Specifies the phone configuration of the Quick Connect. This is required only if `quick_connect_type` is `PHONE_NUMBER`. The `phone_config` block is documented below.
    #[builder(into, default)]
    #[serde(rename = "phoneConfigs")]
    pub r#phone_configs: Box<Option<Vec<super::super::types::connect::QuickConnectQuickConnectConfigPhoneConfig>>>,
    /// Specifies the queue configuration of the Quick Connect. This is required only if `quick_connect_type` is `QUEUE`. The `queue_config` block is documented below.
    #[builder(into, default)]
    #[serde(rename = "queueConfigs")]
    pub r#queue_configs: Box<Option<Vec<super::super::types::connect::QuickConnectQuickConnectConfigQueueConfig>>>,
    /// Specifies the configuration type of the quick connect. valid values are `PHONE_NUMBER`, `QUEUE`, `USER`.
    #[builder(into)]
    #[serde(rename = "quickConnectType")]
    pub r#quick_connect_type: Box<String>,
    /// Specifies the user configuration of the Quick Connect. This is required only if `quick_connect_type` is `USER`. The `user_config` block is documented below.
    #[builder(into, default)]
    #[serde(rename = "userConfigs")]
    pub r#user_configs: Box<Option<Vec<super::super::types::connect::QuickConnectQuickConnectConfigUserConfig>>>,
}
