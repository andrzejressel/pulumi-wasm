#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsGlobalConfigurationInputLossBehaviorInputLossImageSlate {
    #[builder(into, default)]
    #[serde(rename = "passwordParam")]
    pub r#password_param: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}