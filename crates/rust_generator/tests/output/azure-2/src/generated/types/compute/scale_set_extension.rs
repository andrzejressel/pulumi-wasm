#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScaleSetExtension {
    /// Specifies whether or not to use the latest minor version available.
    #[builder(into, default)]
    #[serde(rename = "autoUpgradeMinorVersion")]
    pub r#auto_upgrade_minor_version: Box<Option<bool>>,
    /// Specifies the name of the extension.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The protected_settings passed to the extension, like settings, these are specified as a JSON object in a string.
    #[builder(into, default)]
    #[serde(rename = "protectedSettings")]
    pub r#protected_settings: Box<Option<String>>,
    /// Specifies a dependency array of extensions required to be executed before, the array stores the name of each extension.
    #[builder(into, default)]
    #[serde(rename = "provisionAfterExtensions")]
    pub r#provision_after_extensions: Box<Option<Vec<String>>>,
    /// The publisher of the extension, available publishers can be found by using the Azure CLI.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
    /// The settings passed to the extension, these are specified as a JSON object in a string.
    #[builder(into, default)]
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<String>>,
    /// The type of extension, available types for a publisher can be found using the Azure CLI.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Specifies the version of the extension to use, available versions can be found using the Azure CLI.
    #[builder(into)]
    #[serde(rename = "typeHandlerVersion")]
    pub r#type_handler_version: Box<String>,
}
