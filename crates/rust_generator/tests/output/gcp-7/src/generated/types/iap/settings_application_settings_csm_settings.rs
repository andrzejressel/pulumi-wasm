#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SettingsApplicationSettingsCsmSettings {
    /// Audience claim set in the generated RCToken. This value is not validated by IAP.
    #[builder(into, default)]
    #[serde(rename = "rctokenAud")]
    pub r#rctoken_aud: Box<Option<String>>,
}
