#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PoolExtension {
    /// Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true.
    #[builder(into, default)]
    #[serde(rename = "autoUpgradeMinorVersion")]
    pub r#auto_upgrade_minor_version: Box<Option<bool>>,
    /// Indicates whether the extension should be automatically upgraded by the platform if there is a newer version available. Supported values are `true` and `false`.
    /// 
    /// > **NOTE:** When `automatic_upgrade_enabled` is set to `true`, the `type_handler_version` is automatically updated by the Azure platform when a new version is available and any change in `type_handler_version` should be manually ignored by user.
    #[builder(into, default)]
    #[serde(rename = "automaticUpgradeEnabled")]
    pub r#automatic_upgrade_enabled: Box<Option<bool>>,
    /// The name of the virtual machine extension.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// JSON formatted protected settings for the extension, the value should be encoded with `jsonencode` function. The extension can contain either `protected_settings` or `provision_after_extensions` or no protected settings at all.
    #[builder(into, default)]
    #[serde(rename = "protectedSettings")]
    pub r#protected_settings: Box<Option<String>>,
    /// The collection of extension names. Collection of extension names after which this extension needs to be provisioned.
    #[builder(into, default)]
    #[serde(rename = "provisionAfterExtensions")]
    pub r#provision_after_extensions: Box<Option<Vec<String>>>,
    /// The name of the extension handler publisher.The name of the extension handler publisher.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
    /// JSON formatted public settings for the extension, the value should be encoded with `jsonencode` function.
    #[builder(into, default)]
    #[serde(rename = "settingsJson")]
    pub r#settings_json: Box<Option<String>>,
    /// The type of the extensions.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The version of script handler.
    #[builder(into, default)]
    #[serde(rename = "typeHandlerVersion")]
    pub r#type_handler_version: Box<Option<String>>,
}
