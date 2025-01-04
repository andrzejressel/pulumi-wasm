#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationSetSendingOptions {
    /// If `true`, email sending is enabled for the configuration set. If `false`, email sending is disabled for the configuration set.
    #[builder(into, default)]
    #[serde(rename = "sendingEnabled")]
    pub r#sending_enabled: Box<Option<bool>>,
}
