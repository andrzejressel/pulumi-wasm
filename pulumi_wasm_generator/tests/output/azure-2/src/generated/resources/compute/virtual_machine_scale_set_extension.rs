/// Manages an Extension for a Virtual Machine Scale Set.
///
/// > **NOTE:** This resource is not intended to be used with the `azure.compute.ScaleSet` resource - instead it's intended for this to be used with the `azure.compute.LinuxVirtualMachineScaleSet` and `azure.compute.WindowsVirtualMachineScaleSet` resources.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: West Europe
///   exampleLinuxVirtualMachineScaleSet:
///     type: azure:compute:LinuxVirtualMachineScaleSet
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: Standard_F2
///       adminUsername: adminuser
///       instances: 1
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///       networkInterfaces:
///         - name: example
///           ipConfigurations:
///             - name: internal
///       osDisk:
///         storageAccountType: Standard_LRS
///         caching: ReadWrite
///   exampleVirtualMachineScaleSetExtension:
///     type: azure:compute:VirtualMachineScaleSetExtension
///     name: example
///     properties:
///       name: example
///       virtualMachineScaleSetId: ${exampleLinuxVirtualMachineScaleSet.id}
///       publisher: Microsoft.Azure.Extensions
///       type: CustomScript
///       typeHandlerVersion: '2.0'
///       settings:
///         fn::toJSON:
///           commandToExecute: echo $HOSTNAME
/// ```
///
/// ## Import
///
/// Virtual Machine Scale Set Extensions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/virtualMachineScaleSetExtension:VirtualMachineScaleSetExtension test /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Compute/virtualMachineScaleSets/scaleSet1/extensions/extension1
/// ```
///
pub mod virtual_machine_scale_set_extension {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineScaleSetExtensionArgs {
        /// Should the latest version of the Extension be used at Deployment Time, if one is available? This won't auto-update the extension on existing installation. Defaults to `true`.
        #[builder(into, default)]
        pub auto_upgrade_minor_version: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Extension be automatically updated whenever the Publisher releases a new version of this VM Extension?
        #[builder(into, default)]
        pub automatic_upgrade_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should failures from the extension be suppressed? Possible values are `true` or `false`. Defaults to `false`.
        ///
        /// > **NOTE:** Operational failures such as not connecting to the VM will not be suppressed regardless of the `failure_suppression_enabled` value.
        #[builder(into, default)]
        pub failure_suppression_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A value which, when different to the previous value can be used to force-run the Extension even if the Extension Configuration hasn't changed.
        #[builder(into, default)]
        pub force_update_tag: pulumi_wasm_rust::Output<Option<String>>,
        /// The name for the Virtual Machine Scale Set Extension. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A JSON String which specifies Sensitive Settings (such as Passwords) for the Extension.
        ///
        /// > **NOTE:** Keys within the `protected_settings` block are notoriously case-sensitive, where the casing required (e.g. TitleCase vs snakeCase) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
        #[builder(into, default)]
        pub protected_settings: pulumi_wasm_rust::Output<Option<String>>,
        /// A `protected_settings_from_key_vault` block as defined below.
        ///
        /// > **Note:** `protected_settings_from_key_vault` cannot be used with `protected_settings`
        #[builder(into, default)]
        pub protected_settings_from_key_vault: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::VirtualMachineScaleSetExtensionProtectedSettingsFromKeyVault,
            >,
        >,
        /// An ordered list of Extension names which this should be provisioned after.
        #[builder(into, default)]
        pub provision_after_extensions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the Publisher of the Extension. Changing this forces a new resource to be created.
        #[builder(into)]
        pub publisher: pulumi_wasm_rust::Output<String>,
        /// A JSON String which specifies Settings for the Extension.
        ///
        /// > **NOTE:** Keys within the `settings` block are notoriously case-sensitive, where the casing required (e.g. TitleCase vs snakeCase) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
        #[builder(into, default)]
        pub settings: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Type of the Extension. Changing this forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of the extension to use, available versions can be found using the Azure CLI.
        ///
        /// > **Note:** The `Publisher` and `Type` of Virtual Machine Scale Set Extensions can be found using the Azure CLI, via:
        ///
        /// ```shell
        /// az vmss extension image list --location westus -o table
        /// ```
        #[builder(into)]
        pub type_handler_version: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Machine Scale Set. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This should be the ID from the `azure.compute.LinuxVirtualMachineScaleSet` or `azure.compute.WindowsVirtualMachineScaleSet` resource - when using the older `azure.compute.ScaleSet` resource extensions should instead be defined inline.
        #[builder(into)]
        pub virtual_machine_scale_set_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineScaleSetExtensionResult {
        /// Should the latest version of the Extension be used at Deployment Time, if one is available? This won't auto-update the extension on existing installation. Defaults to `true`.
        pub auto_upgrade_minor_version: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Extension be automatically updated whenever the Publisher releases a new version of this VM Extension?
        pub automatic_upgrade_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should failures from the extension be suppressed? Possible values are `true` or `false`. Defaults to `false`.
        ///
        /// > **NOTE:** Operational failures such as not connecting to the VM will not be suppressed regardless of the `failure_suppression_enabled` value.
        pub failure_suppression_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A value which, when different to the previous value can be used to force-run the Extension even if the Extension Configuration hasn't changed.
        pub force_update_tag: pulumi_wasm_rust::Output<Option<String>>,
        /// The name for the Virtual Machine Scale Set Extension. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A JSON String which specifies Sensitive Settings (such as Passwords) for the Extension.
        ///
        /// > **NOTE:** Keys within the `protected_settings` block are notoriously case-sensitive, where the casing required (e.g. TitleCase vs snakeCase) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
        pub protected_settings: pulumi_wasm_rust::Output<Option<String>>,
        /// A `protected_settings_from_key_vault` block as defined below.
        ///
        /// > **Note:** `protected_settings_from_key_vault` cannot be used with `protected_settings`
        pub protected_settings_from_key_vault: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::VirtualMachineScaleSetExtensionProtectedSettingsFromKeyVault,
            >,
        >,
        /// An ordered list of Extension names which this should be provisioned after.
        pub provision_after_extensions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the Publisher of the Extension. Changing this forces a new resource to be created.
        pub publisher: pulumi_wasm_rust::Output<String>,
        /// A JSON String which specifies Settings for the Extension.
        ///
        /// > **NOTE:** Keys within the `settings` block are notoriously case-sensitive, where the casing required (e.g. TitleCase vs snakeCase) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
        pub settings: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Type of the Extension. Changing this forces a new resource to be created.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of the extension to use, available versions can be found using the Azure CLI.
        ///
        /// > **Note:** The `Publisher` and `Type` of Virtual Machine Scale Set Extensions can be found using the Azure CLI, via:
        ///
        /// ```shell
        /// az vmss extension image list --location westus -o table
        /// ```
        pub type_handler_version: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Machine Scale Set. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This should be the ID from the `azure.compute.LinuxVirtualMachineScaleSet` or `azure.compute.WindowsVirtualMachineScaleSet` resource - when using the older `azure.compute.ScaleSet` resource extensions should instead be defined inline.
        pub virtual_machine_scale_set_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VirtualMachineScaleSetExtensionArgs,
    ) -> VirtualMachineScaleSetExtensionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_upgrade_minor_version_binding = args
            .auto_upgrade_minor_version
            .get_inner();
        let automatic_upgrade_enabled_binding = args
            .automatic_upgrade_enabled
            .get_inner();
        let failure_suppression_enabled_binding = args
            .failure_suppression_enabled
            .get_inner();
        let force_update_tag_binding = args.force_update_tag.get_inner();
        let name_binding = args.name.get_inner();
        let protected_settings_binding = args.protected_settings.get_inner();
        let protected_settings_from_key_vault_binding = args
            .protected_settings_from_key_vault
            .get_inner();
        let provision_after_extensions_binding = args
            .provision_after_extensions
            .get_inner();
        let publisher_binding = args.publisher.get_inner();
        let settings_binding = args.settings.get_inner();
        let type__binding = args.type_.get_inner();
        let type_handler_version_binding = args.type_handler_version.get_inner();
        let virtual_machine_scale_set_id_binding = args
            .virtual_machine_scale_set_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/virtualMachineScaleSetExtension:VirtualMachineScaleSetExtension"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoUpgradeMinorVersion".into(),
                    value: &auto_upgrade_minor_version_binding,
                },
                register_interface::ObjectField {
                    name: "automaticUpgradeEnabled".into(),
                    value: &automatic_upgrade_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "failureSuppressionEnabled".into(),
                    value: &failure_suppression_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "forceUpdateTag".into(),
                    value: &force_update_tag_binding,
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
                    name: "protectedSettingsFromKeyVault".into(),
                    value: &protected_settings_from_key_vault_binding,
                },
                register_interface::ObjectField {
                    name: "provisionAfterExtensions".into(),
                    value: &provision_after_extensions_binding,
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
                register_interface::ObjectField {
                    name: "virtualMachineScaleSetId".into(),
                    value: &virtual_machine_scale_set_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoUpgradeMinorVersion".into(),
                },
                register_interface::ResultField {
                    name: "automaticUpgradeEnabled".into(),
                },
                register_interface::ResultField {
                    name: "failureSuppressionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "forceUpdateTag".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "protectedSettings".into(),
                },
                register_interface::ResultField {
                    name: "protectedSettingsFromKeyVault".into(),
                },
                register_interface::ResultField {
                    name: "provisionAfterExtensions".into(),
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
                register_interface::ResultField {
                    name: "virtualMachineScaleSetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualMachineScaleSetExtensionResult {
            auto_upgrade_minor_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoUpgradeMinorVersion").unwrap(),
            ),
            automatic_upgrade_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticUpgradeEnabled").unwrap(),
            ),
            failure_suppression_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failureSuppressionEnabled").unwrap(),
            ),
            force_update_tag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceUpdateTag").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            protected_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protectedSettings").unwrap(),
            ),
            protected_settings_from_key_vault: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protectedSettingsFromKeyVault").unwrap(),
            ),
            provision_after_extensions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisionAfterExtensions").unwrap(),
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
            virtual_machine_scale_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMachineScaleSetId").unwrap(),
            ),
        }
    }
}
