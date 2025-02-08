#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SettingsAccessSettingsCorsSettings {
    /// Configuration to allow HTTP OPTIONS calls to skip authorization.
    /// If undefined, IAP will not apply any special logic to OPTIONS requests.
    #[builder(into, default)]
    #[serde(rename = "allowHttpOptions")]
    pub r#allow_http_options: Box<Option<bool>>,
}
