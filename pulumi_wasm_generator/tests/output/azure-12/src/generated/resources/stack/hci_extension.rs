/// Manages an Azure Stack HCI Extension.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod hci_extension {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HciExtensionArgs {
        /// The ID of the Azure Stack HCI Cluster Arc Setting. Changing this forces a new resource to be created.
        #[builder(into)]
        pub arc_setting_id: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true. Changing this forces a new resource to be created. Possible values are `true` and `false`. Defaults to `true`.
        #[builder(into, default)]
        pub auto_upgrade_minor_version_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Indicates whether the extension should be automatically upgraded by the platform if there is a newer version available. Possible values are `true` and `false`. Defaults to `true`.
        #[builder(into, default)]
        pub automatic_upgrade_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Azure Stack HCI Extension. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The json formatted protected settings for the extension.
        #[builder(into, default)]
        pub protected_settings: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the extension handler publisher, such as `Microsoft.Azure.Monitor`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub publisher: pulumi_wasm_rust::Output<String>,
        /// The json formatted public settings for the extension.
        #[builder(into, default)]
        pub settings: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the type of the extension. For example `CustomScriptExtension` or `AzureMonitorLinuxAgent`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of the script handler.
        ///
        /// > **NOTE:** `type_handler_version` cannot be set when `automatic_upgrade_enabled` is set to `true`.
        #[builder(into, default)]
        pub type_handler_version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HciExtensionResult {
        /// The ID of the Azure Stack HCI Cluster Arc Setting. Changing this forces a new resource to be created.
        pub arc_setting_id: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true. Changing this forces a new resource to be created. Possible values are `true` and `false`. Defaults to `true`.
        pub auto_upgrade_minor_version_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Indicates whether the extension should be automatically upgraded by the platform if there is a newer version available. Possible values are `true` and `false`. Defaults to `true`.
        pub automatic_upgrade_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Azure Stack HCI Extension. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The json formatted protected settings for the extension.
        pub protected_settings: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the extension handler publisher, such as `Microsoft.Azure.Monitor`. Changing this forces a new resource to be created.
        pub publisher: pulumi_wasm_rust::Output<String>,
        /// The json formatted public settings for the extension.
        pub settings: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the type of the extension. For example `CustomScriptExtension` or `AzureMonitorLinuxAgent`. Changing this forces a new resource to be created.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of the script handler.
        ///
        /// > **NOTE:** `type_handler_version` cannot be set when `automatic_upgrade_enabled` is set to `true`.
        pub type_handler_version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HciExtensionArgs) -> HciExtensionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arc_setting_id_binding = args.arc_setting_id.get_inner();
        let auto_upgrade_minor_version_enabled_binding = args
            .auto_upgrade_minor_version_enabled
            .get_inner();
        let automatic_upgrade_enabled_binding = args
            .automatic_upgrade_enabled
            .get_inner();
        let name_binding = args.name.get_inner();
        let protected_settings_binding = args.protected_settings.get_inner();
        let publisher_binding = args.publisher.get_inner();
        let settings_binding = args.settings.get_inner();
        let type__binding = args.type_.get_inner();
        let type_handler_version_binding = args.type_handler_version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:stack/hciExtension:HciExtension".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arcSettingId".into(),
                    value: &arc_setting_id_binding,
                },
                register_interface::ObjectField {
                    name: "autoUpgradeMinorVersionEnabled".into(),
                    value: &auto_upgrade_minor_version_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "automaticUpgradeEnabled".into(),
                    value: &automatic_upgrade_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "protectedSettings".into(),
                    value: &protected_settings_binding,
                },
                register_interface::ObjectField {
                    name: "publisher".into(),
                    value: &publisher_binding,
                },
                register_interface::ObjectField {
                    name: "settings".into(),
                    value: &settings_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "typeHandlerVersion".into(),
                    value: &type_handler_version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arcSettingId".into(),
                },
                register_interface::ResultField {
                    name: "autoUpgradeMinorVersionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "automaticUpgradeEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "protectedSettings".into(),
                },
                register_interface::ResultField {
                    name: "publisher".into(),
                },
                register_interface::ResultField {
                    name: "settings".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "typeHandlerVersion".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HciExtensionResult {
            arc_setting_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arcSettingId").unwrap(),
            ),
            auto_upgrade_minor_version_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoUpgradeMinorVersionEnabled").unwrap(),
            ),
            automatic_upgrade_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticUpgradeEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            protected_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protectedSettings").unwrap(),
            ),
            publisher: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publisher").unwrap(),
            ),
            settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("settings").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            type_handler_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("typeHandlerVersion").unwrap(),
            ),
        }
    }
}
