/// Manages a Hybrid Compute Machine Extension.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example
///       location: West Europe
///   exampleExtension:
///     type: azure:arcmachine:Extension
///     name: example
///     properties:
///       name: example
///       location: West Europe
///       arcMachineId: ${example.id}
///       publisher: Microsoft.Azure.Monitor
///       type: AzureMonitorLinuxAgent
/// variables:
///   example:
///     fn::invoke:
///       function: azure:arcmachine:get
///       arguments:
///         name: existing-hcmachine
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Hybrid Compute Machine Extensions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:arcmachine/extension:Extension example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.HybridCompute/machines/hcmachine1/extensions/ext1
/// ```
///
pub mod extension {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExtensionArgs {
        /// The ID of the Hybrid Compute Machine Extension. Changing this forces a new Hybrid Compute Machine Extension to be created.
        #[builder(into)]
        pub arc_machine_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Indicates whether the extension should be automatically upgraded by the platform if there is a newer version available. Supported values are `true` and `false`. Defaults to `true`.
        ///
        /// > **NOTE:** When `automatic_upgrade_enabled` can only be set during creation. Any later change will be ignored.
        ///
        /// > **NOTE:** When `automatic_upgrade_enabled` is set to `true`, the `type_handler_version` is automatically updated by the Azure platform when a new version is available and any change in `type_handler_version` will be automatically ignored.
        #[builder(into, default)]
        pub automatic_upgrade_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// How the extension handler should be forced to update even if the extension configuration has not changed.
        #[builder(into, default)]
        pub force_update_tag: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Hybrid Compute Machine Extension should exist. Changing this forces a new Hybrid Compute Machine Extension to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Hybrid Compute Machine Extension. Changing this forces a new Hybrid Compute Machine Extension to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Json formatted protected settings for the extension.
        #[builder(into, default)]
        pub protected_settings: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the extension handler publisher, such as `Microsoft.Azure.Monitor`. Changing this forces a new Hybrid Compute Machine Extension to be created.
        #[builder(into)]
        pub publisher: pulumi_wasm_rust::InputOrOutput<String>,
        /// Json formatted public settings for the extension.
        #[builder(into, default)]
        pub settings: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Hybrid Compute Machine Extension.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the type of the extension. For example `CustomScriptExtension` or `AzureMonitorLinuxAgent`. Changing this forces a new Hybrid Compute Machine Extension to be created.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the version of the script handler.
        ///
        /// > **NOTE:** 1. When `automatic_upgrade_enabled` is set to `false` and no `type_handler_version` is specified, the `type_handler_version` change should be manually ignored by `ignore_changes` lifecycle block. This is because the `type_handler_version` is set by the Azure platform when the extension is created. 2. When `automatic_upgrade_enabled` is set to `false` and `type_handler_version` is specified, the provider will check whether the version prefix is aligned with user input. For example, if user specifies `1.24` in `type_handler_version`, `1.24.1` will be considered as no diff.
        #[builder(into, default)]
        pub type_handler_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ExtensionResult {
        /// The ID of the Hybrid Compute Machine Extension. Changing this forces a new Hybrid Compute Machine Extension to be created.
        pub arc_machine_id: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the extension should be automatically upgraded by the platform if there is a newer version available. Supported values are `true` and `false`. Defaults to `true`.
        ///
        /// > **NOTE:** When `automatic_upgrade_enabled` can only be set during creation. Any later change will be ignored.
        ///
        /// > **NOTE:** When `automatic_upgrade_enabled` is set to `true`, the `type_handler_version` is automatically updated by the Azure platform when a new version is available and any change in `type_handler_version` will be automatically ignored.
        pub automatic_upgrade_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// How the extension handler should be forced to update even if the extension configuration has not changed.
        pub force_update_tag: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure Region where the Hybrid Compute Machine Extension should exist. Changing this forces a new Hybrid Compute Machine Extension to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Hybrid Compute Machine Extension. Changing this forces a new Hybrid Compute Machine Extension to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Json formatted protected settings for the extension.
        pub protected_settings: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the extension handler publisher, such as `Microsoft.Azure.Monitor`. Changing this forces a new Hybrid Compute Machine Extension to be created.
        pub publisher: pulumi_wasm_rust::Output<String>,
        /// Json formatted public settings for the extension.
        pub settings: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Hybrid Compute Machine Extension.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the type of the extension. For example `CustomScriptExtension` or `AzureMonitorLinuxAgent`. Changing this forces a new Hybrid Compute Machine Extension to be created.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of the script handler.
        ///
        /// > **NOTE:** 1. When `automatic_upgrade_enabled` is set to `false` and no `type_handler_version` is specified, the `type_handler_version` change should be manually ignored by `ignore_changes` lifecycle block. This is because the `type_handler_version` is set by the Azure platform when the extension is created. 2. When `automatic_upgrade_enabled` is set to `false` and `type_handler_version` is specified, the provider will check whether the version prefix is aligned with user input. For example, if user specifies `1.24` in `type_handler_version`, `1.24.1` will be considered as no diff.
        pub type_handler_version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ExtensionArgs,
    ) -> ExtensionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arc_machine_id_binding = args.arc_machine_id.get_output(context).get_inner();
        let automatic_upgrade_enabled_binding = args
            .automatic_upgrade_enabled
            .get_output(context)
            .get_inner();
        let force_update_tag_binding = args
            .force_update_tag
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let protected_settings_binding = args
            .protected_settings
            .get_output(context)
            .get_inner();
        let publisher_binding = args.publisher.get_output(context).get_inner();
        let settings_binding = args.settings.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let type_handler_version_binding = args
            .type_handler_version
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:arcmachine/extension:Extension".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arcMachineId".into(),
                    value: &arc_machine_id_binding,
                },
                register_interface::ObjectField {
                    name: "automaticUpgradeEnabled".into(),
                    value: &automatic_upgrade_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "forceUpdateTag".into(),
                    value: &force_update_tag_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "tags".into(),
                    value: &tags_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ExtensionResult {
            arc_machine_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("arcMachineId"),
            ),
            automatic_upgrade_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("automaticUpgradeEnabled"),
            ),
            force_update_tag: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forceUpdateTag"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            protected_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protectedSettings"),
            ),
            publisher: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publisher"),
            ),
            settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("settings"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            type_handler_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("typeHandlerVersion"),
            ),
        }
    }
}
