#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinuxWebAppStickySetting {
    /// A list of `app_setting` names that the Linux Web App will not swap between Slots when a swap operation is triggered.
    #[builder(into)]
    #[serde(rename = "appSettingNames")]
    pub r#app_setting_names: Box<Vec<String>>,
    /// A list of `connection_string` names that the Linux Web App will not swap between Slots when a swap operation is triggered.
    #[builder(into)]
    #[serde(rename = "connectionStringNames")]
    pub r#connection_string_names: Box<Vec<String>>,
}
