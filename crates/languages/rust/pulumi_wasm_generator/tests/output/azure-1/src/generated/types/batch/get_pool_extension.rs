#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolExtension {
    /// Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true.
    #[builder(into)]
    #[serde(rename = "autoUpgradeMinorVersion")]
    pub r#auto_upgrade_minor_version: Box<bool>,
    /// The name of the user account.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The extension can contain either `protected_settings` or `provision_after_extensions` or no protected settings at all.
    #[builder(into)]
    #[serde(rename = "protectedSettings")]
    pub r#protected_settings: Box<String>,
    /// The collection of extension names. Collection of extension names after which this extension needs to be provisioned.
    #[builder(into)]
    #[serde(rename = "provisionAfterExtensions")]
    pub r#provision_after_extensions: Box<Vec<String>>,
    /// The name of the extension handler publisher.The name of the extension handler publisher.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
    /// JSON formatted public settings for the extension.
    #[builder(into)]
    #[serde(rename = "settingsJson")]
    pub r#settings_json: Box<String>,
    /// The type of container configuration.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The version of script handler.
    #[builder(into)]
    #[serde(rename = "typeHandlerVersion")]
    pub r#type_handler_version: Box<String>,
}
