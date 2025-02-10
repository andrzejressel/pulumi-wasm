/// Manages an Azure Stack HCI Extension.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-hci-ext")
///             .build_struct(),
///     );
///     let exampleHciExtension = hci_extension::create(
///         "exampleHciExtension",
///         HciExtensionArgs::builder()
///             .arc_setting_id(
///                 "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-hci/providers/Microsoft.AzureStackHCI/clusters/hci-cl/arcSettings/default",
///             )
///             .auto_upgrade_minor_version_enabled(true)
///             .automatic_upgrade_enabled(true)
///             .name("AzureMonitorWindowsAgent")
///             .publisher("Microsoft.Azure.Monitor")
///             .type_("MicrosoftMonitoringAgent")
///             .type_handler_version("1.22.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Stack HCI Extension can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:stack/hciExtension:HciExtension example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.AzureStackHCI/clusters/cluster1/arcSettings/default/extensions/extension1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hci_extension {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HciExtensionArgs {
        /// The ID of the Azure Stack HCI Cluster Arc Setting. Changing this forces a new resource to be created.
        #[builder(into)]
        pub arc_setting_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true. Changing this forces a new resource to be created. Possible values are `true` and `false`. Defaults to `true`.
        #[builder(into, default)]
        pub auto_upgrade_minor_version_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Indicates whether the extension should be automatically upgraded by the platform if there is a newer version available. Possible values are `true` and `false`. Defaults to `true`.
        #[builder(into, default)]
        pub automatic_upgrade_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Azure Stack HCI Extension. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The json formatted protected settings for the extension.
        #[builder(into, default)]
        pub protected_settings: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the extension handler publisher, such as `Microsoft.Azure.Monitor`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub publisher: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The json formatted public settings for the extension.
        #[builder(into, default)]
        pub settings: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the type of the extension. For example `CustomScriptExtension` or `AzureMonitorLinuxAgent`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the version of the script handler.
        ///
        /// > **NOTE:** `type_handler_version` cannot be set when `automatic_upgrade_enabled` is set to `true`.
        #[builder(into, default)]
        pub type_handler_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HciExtensionResult {
        /// The ID of the Azure Stack HCI Cluster Arc Setting. Changing this forces a new resource to be created.
        pub arc_setting_id: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true. Changing this forces a new resource to be created. Possible values are `true` and `false`. Defaults to `true`.
        pub auto_upgrade_minor_version_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Indicates whether the extension should be automatically upgraded by the platform if there is a newer version available. Possible values are `true` and `false`. Defaults to `true`.
        pub automatic_upgrade_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Azure Stack HCI Extension. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The json formatted protected settings for the extension.
        pub protected_settings: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the extension handler publisher, such as `Microsoft.Azure.Monitor`. Changing this forces a new resource to be created.
        pub publisher: pulumi_gestalt_rust::Output<String>,
        /// The json formatted public settings for the extension.
        pub settings: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the type of the extension. For example `CustomScriptExtension` or `AzureMonitorLinuxAgent`. Changing this forces a new resource to be created.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Specifies the version of the script handler.
        ///
        /// > **NOTE:** `type_handler_version` cannot be set when `automatic_upgrade_enabled` is set to `true`.
        pub type_handler_version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HciExtensionArgs,
    ) -> HciExtensionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arc_setting_id_binding = args.arc_setting_id.get_output(context);
        let auto_upgrade_minor_version_enabled_binding = args
            .auto_upgrade_minor_version_enabled
            .get_output(context);
        let automatic_upgrade_enabled_binding = args
            .automatic_upgrade_enabled
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let protected_settings_binding = args.protected_settings.get_output(context);
        let publisher_binding = args.publisher.get_output(context);
        let settings_binding = args.settings.get_output(context);
        let type__binding = args.type_.get_output(context);
        let type_handler_version_binding = args.type_handler_version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:stack/hciExtension:HciExtension".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arcSettingId".into(),
                    value: arc_setting_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoUpgradeMinorVersionEnabled".into(),
                    value: auto_upgrade_minor_version_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticUpgradeEnabled".into(),
                    value: automatic_upgrade_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protectedSettings".into(),
                    value: protected_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publisher".into(),
                    value: publisher_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settings".into(),
                    value: settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "typeHandlerVersion".into(),
                    value: type_handler_version_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HciExtensionResult {
            arc_setting_id: o.get_field("arcSettingId"),
            auto_upgrade_minor_version_enabled: o
                .get_field("autoUpgradeMinorVersionEnabled"),
            automatic_upgrade_enabled: o.get_field("automaticUpgradeEnabled"),
            name: o.get_field("name"),
            protected_settings: o.get_field("protectedSettings"),
            publisher: o.get_field("publisher"),
            settings: o.get_field("settings"),
            type_: o.get_field("type"),
            type_handler_version: o.get_field("typeHandlerVersion"),
        }
    }
}
