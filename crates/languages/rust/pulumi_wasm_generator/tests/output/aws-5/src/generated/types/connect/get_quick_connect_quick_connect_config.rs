#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetQuickConnectQuickConnectConfig {
    /// Phone configuration of the Quick Connect. This is returned only if `quick_connect_type` is `PHONE_NUMBER`. The `phone_config` block is documented below.
    #[builder(into)]
    #[serde(rename = "phoneConfigs")]
    pub r#phone_configs: Box<Vec<super::super::types::connect::GetQuickConnectQuickConnectConfigPhoneConfig>>,
    /// Queue configuration of the Quick Connect. This is returned only if `quick_connect_type` is `QUEUE`. The `queue_config` block is documented below.
    #[builder(into)]
    #[serde(rename = "queueConfigs")]
    pub r#queue_configs: Box<Vec<super::super::types::connect::GetQuickConnectQuickConnectConfigQueueConfig>>,
    /// Configuration type of the Quick Connect. Valid values are `PHONE_NUMBER`, `QUEUE`, `USER`.
    #[builder(into)]
    #[serde(rename = "quickConnectType")]
    pub r#quick_connect_type: Box<String>,
    /// User configuration of the Quick Connect. This is returned only if `quick_connect_type` is `USER`. The `user_config` block is documented below.
    #[builder(into)]
    #[serde(rename = "userConfigs")]
    pub r#user_configs: Box<Vec<super::super::types::connect::GetQuickConnectQuickConnectConfigUserConfig>>,
}
