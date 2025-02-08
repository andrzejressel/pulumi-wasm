#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StackApplicationSettings {
    /// Whether application settings should be persisted.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Name of the settings group.
    /// Required when `enabled` is `true`.
    /// Can be up to 100 characters.
    #[builder(into, default)]
    #[serde(rename = "settingsGroup")]
    pub r#settings_group: Box<Option<String>>,
}
