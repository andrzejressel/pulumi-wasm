#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettingsDvbSdtSettings {
    #[builder(into, default)]
    #[serde(rename = "outputSdt")]
    pub r#output_sdt: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "repInterval")]
    pub r#rep_interval: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "serviceProviderName")]
    pub r#service_provider_name: Box<Option<String>>,
}