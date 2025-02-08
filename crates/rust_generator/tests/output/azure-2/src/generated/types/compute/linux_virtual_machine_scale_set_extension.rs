#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LinuxVirtualMachineScaleSetExtension {
    /// Should the latest version of the Extension be used at Deployment Time, if one is available? This won't auto-update the extension on existing installation. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "autoUpgradeMinorVersion")]
    pub r#auto_upgrade_minor_version: Box<Option<bool>>,
    /// Should the Extension be automatically updated whenever the Publisher releases a new version of this VM Extension?
    #[builder(into, default)]
    #[serde(rename = "automaticUpgradeEnabled")]
    pub r#automatic_upgrade_enabled: Box<Option<bool>>,
    /// A value which, when different to the previous value can be used to force-run the Extension even if the Extension Configuration hasn't changed.
    #[builder(into, default)]
    #[serde(rename = "forceUpdateTag")]
    pub r#force_update_tag: Box<Option<String>>,
    /// The name for the Virtual Machine Scale Set Extension.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A JSON String which specifies Sensitive Settings (such as Passwords) for the Extension.
    /// 
    /// > **Note:** Keys within the `protected_settings` block are notoriously case-sensitive, where the casing required (e.g. TitleCase vs snakeCase) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
    #[builder(into, default)]
    #[serde(rename = "protectedSettings")]
    pub r#protected_settings: Box<Option<String>>,
    /// A `protected_settings_from_key_vault` block as defined below.
    /// 
    /// > **Note:** `protected_settings_from_key_vault` cannot be used with `protected_settings`
    #[builder(into, default)]
    #[serde(rename = "protectedSettingsFromKeyVault")]
    pub r#protected_settings_from_key_vault: Box<Option<super::super::types::compute::LinuxVirtualMachineScaleSetExtensionProtectedSettingsFromKeyVault>>,
    /// An ordered list of Extension names which this should be provisioned after.
    #[builder(into, default)]
    #[serde(rename = "provisionAfterExtensions")]
    pub r#provision_after_extensions: Box<Option<Vec<String>>>,
    /// Specifies the Publisher of the Extension.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
    /// A JSON String which specifies Settings for the Extension.
    /// 
    /// > **Note:** Keys within the `settings` block are notoriously case-sensitive, where the casing required (e.g. TitleCase vs snakeCase) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
    #[builder(into, default)]
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<String>>,
    /// Specifies the Type of the Extension.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Specifies the version of the extension to use, available versions can be found using the Azure CLI.
    #[builder(into)]
    #[serde(rename = "typeHandlerVersion")]
    pub r#type_handler_version: Box<String>,
}
