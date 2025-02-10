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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_machine_scale_set_extension {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineScaleSetExtensionArgs {
        /// Should the latest version of the Extension be used at Deployment Time, if one is available? This won't auto-update the extension on existing installation. Defaults to `true`.
        #[builder(into, default)]
        pub auto_upgrade_minor_version: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should the Extension be automatically updated whenever the Publisher releases a new version of this VM Extension?
        #[builder(into, default)]
        pub automatic_upgrade_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should failures from the extension be suppressed? Possible values are `true` or `false`. Defaults to `false`.
        ///
        /// > **NOTE:** Operational failures such as not connecting to the VM will not be suppressed regardless of the `failure_suppression_enabled` value.
        #[builder(into, default)]
        pub failure_suppression_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A value which, when different to the previous value can be used to force-run the Extension even if the Extension Configuration hasn't changed.
        #[builder(into, default)]
        pub force_update_tag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name for the Virtual Machine Scale Set Extension. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A JSON String which specifies Sensitive Settings (such as Passwords) for the Extension.
        ///
        /// > **NOTE:** Keys within the `protected_settings` block are notoriously case-sensitive, where the casing required (e.g. TitleCase vs snakeCase) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
        #[builder(into, default)]
        pub protected_settings: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `protected_settings_from_key_vault` block as defined below.
        ///
        /// > **Note:** `protected_settings_from_key_vault` cannot be used with `protected_settings`
        #[builder(into, default)]
        pub protected_settings_from_key_vault: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::VirtualMachineScaleSetExtensionProtectedSettingsFromKeyVault,
            >,
        >,
        /// An ordered list of Extension names which this should be provisioned after.
        #[builder(into, default)]
        pub provision_after_extensions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies the Publisher of the Extension. Changing this forces a new resource to be created.
        #[builder(into)]
        pub publisher: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A JSON String which specifies Settings for the Extension.
        ///
        /// > **NOTE:** Keys within the `settings` block are notoriously case-sensitive, where the casing required (e.g. TitleCase vs snakeCase) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
        #[builder(into, default)]
        pub settings: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Type of the Extension. Changing this forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the version of the extension to use, available versions can be found using the Azure CLI.
        ///
        /// > **Note:** The `Publisher` and `Type` of Virtual Machine Scale Set Extensions can be found using the Azure CLI, via:
        ///
        /// ```shell
        /// az vmss extension image list --location westus -o table
        /// ```
        #[builder(into)]
        pub type_handler_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Virtual Machine Scale Set. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This should be the ID from the `azure.compute.LinuxVirtualMachineScaleSet` or `azure.compute.WindowsVirtualMachineScaleSet` resource - when using the older `azure.compute.ScaleSet` resource extensions should instead be defined inline.
        #[builder(into)]
        pub virtual_machine_scale_set_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineScaleSetExtensionResult {
        /// Should the latest version of the Extension be used at Deployment Time, if one is available? This won't auto-update the extension on existing installation. Defaults to `true`.
        pub auto_upgrade_minor_version: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the Extension be automatically updated whenever the Publisher releases a new version of this VM Extension?
        pub automatic_upgrade_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should failures from the extension be suppressed? Possible values are `true` or `false`. Defaults to `false`.
        ///
        /// > **NOTE:** Operational failures such as not connecting to the VM will not be suppressed regardless of the `failure_suppression_enabled` value.
        pub failure_suppression_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A value which, when different to the previous value can be used to force-run the Extension even if the Extension Configuration hasn't changed.
        pub force_update_tag: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name for the Virtual Machine Scale Set Extension. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A JSON String which specifies Sensitive Settings (such as Passwords) for the Extension.
        ///
        /// > **NOTE:** Keys within the `protected_settings` block are notoriously case-sensitive, where the casing required (e.g. TitleCase vs snakeCase) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
        pub protected_settings: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `protected_settings_from_key_vault` block as defined below.
        ///
        /// > **Note:** `protected_settings_from_key_vault` cannot be used with `protected_settings`
        pub protected_settings_from_key_vault: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::VirtualMachineScaleSetExtensionProtectedSettingsFromKeyVault,
            >,
        >,
        /// An ordered list of Extension names which this should be provisioned after.
        pub provision_after_extensions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the Publisher of the Extension. Changing this forces a new resource to be created.
        pub publisher: pulumi_gestalt_rust::Output<String>,
        /// A JSON String which specifies Settings for the Extension.
        ///
        /// > **NOTE:** Keys within the `settings` block are notoriously case-sensitive, where the casing required (e.g. TitleCase vs snakeCase) depends on the Extension being used. Please refer to the documentation for the specific Virtual Machine Extension you're looking to use for more information.
        pub settings: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Type of the Extension. Changing this forces a new resource to be created.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Specifies the version of the extension to use, available versions can be found using the Azure CLI.
        ///
        /// > **Note:** The `Publisher` and `Type` of Virtual Machine Scale Set Extensions can be found using the Azure CLI, via:
        ///
        /// ```shell
        /// az vmss extension image list --location westus -o table
        /// ```
        pub type_handler_version: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Virtual Machine Scale Set. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This should be the ID from the `azure.compute.LinuxVirtualMachineScaleSet` or `azure.compute.WindowsVirtualMachineScaleSet` resource - when using the older `azure.compute.ScaleSet` resource extensions should instead be defined inline.
        pub virtual_machine_scale_set_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualMachineScaleSetExtensionArgs,
    ) -> VirtualMachineScaleSetExtensionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_upgrade_minor_version_binding = args
            .auto_upgrade_minor_version
            .get_output(context);
        let automatic_upgrade_enabled_binding = args
            .automatic_upgrade_enabled
            .get_output(context);
        let failure_suppression_enabled_binding = args
            .failure_suppression_enabled
            .get_output(context);
        let force_update_tag_binding = args.force_update_tag.get_output(context);
        let name_binding = args.name.get_output(context);
        let protected_settings_binding = args.protected_settings.get_output(context);
        let protected_settings_from_key_vault_binding = args
            .protected_settings_from_key_vault
            .get_output(context);
        let provision_after_extensions_binding = args
            .provision_after_extensions
            .get_output(context);
        let publisher_binding = args.publisher.get_output(context);
        let settings_binding = args.settings.get_output(context);
        let type__binding = args.type_.get_output(context);
        let type_handler_version_binding = args.type_handler_version.get_output(context);
        let virtual_machine_scale_set_id_binding = args
            .virtual_machine_scale_set_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/virtualMachineScaleSetExtension:VirtualMachineScaleSetExtension"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoUpgradeMinorVersion".into(),
                    value: auto_upgrade_minor_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticUpgradeEnabled".into(),
                    value: automatic_upgrade_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "failureSuppressionEnabled".into(),
                    value: failure_suppression_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceUpdateTag".into(),
                    value: force_update_tag_binding.get_id(),
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
                    name: "protectedSettingsFromKeyVault".into(),
                    value: protected_settings_from_key_vault_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "provisionAfterExtensions".into(),
                    value: provision_after_extensions_binding.get_id(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachineScaleSetId".into(),
                    value: virtual_machine_scale_set_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualMachineScaleSetExtensionResult {
            auto_upgrade_minor_version: o.get_field("autoUpgradeMinorVersion"),
            automatic_upgrade_enabled: o.get_field("automaticUpgradeEnabled"),
            failure_suppression_enabled: o.get_field("failureSuppressionEnabled"),
            force_update_tag: o.get_field("forceUpdateTag"),
            name: o.get_field("name"),
            protected_settings: o.get_field("protectedSettings"),
            protected_settings_from_key_vault: o
                .get_field("protectedSettingsFromKeyVault"),
            provision_after_extensions: o.get_field("provisionAfterExtensions"),
            publisher: o.get_field("publisher"),
            settings: o.get_field("settings"),
            type_: o.get_field("type"),
            type_handler_version: o.get_field("typeHandlerVersion"),
            virtual_machine_scale_set_id: o.get_field("virtualMachineScaleSetId"),
        }
    }
}
