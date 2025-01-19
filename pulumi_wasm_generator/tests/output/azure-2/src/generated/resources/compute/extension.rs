/// Manages a Virtual Machine Extension to provide post deployment configuration
/// and run automated tasks.
///
/// > **NOTE:** Custom Script Extensions for Linux & Windows require that the `commandToExecute` returns a `0` exit code to be classified as successfully deployed. You can achieve this by appending `exit 0` to the end of your `commandToExecute`.
///
/// > **NOTE:** Custom Script Extensions require that the Azure Virtual Machine Guest Agent is running on the Virtual Machine.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: acctvn
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: acctsub
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleNetworkInterface:
///     type: azure:network:NetworkInterface
///     name: example
///     properties:
///       name: acctni
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       ipConfigurations:
///         - name: testconfiguration1
///           subnetId: ${exampleSubnet.id}
///           privateIpAddressAllocation: Dynamic
///   exampleLinuxVirtualMachine:
///     type: azure:compute:LinuxVirtualMachine
///     name: example
///     properties:
///       name: example-machine
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       size: Standard_F2
///       adminUsername: adminuser
///       networkInterfaceIds:
///         - ${exampleNetworkInterface.id}
///       adminSshKeys:
///         - username: adminuser
///           publicKey:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: ~/.ssh/id_rsa.pub
///               return: result
///       osDisk:
///         caching: ReadWrite
///         storageAccountType: Standard_LRS
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///   exampleExtension:
///     type: azure:compute:Extension
///     name: example
///     properties:
///       name: hostname
///       virtualMachineId: ${exampleLinuxVirtualMachine.id}
///       publisher: Microsoft.Azure.Extensions
///       type: CustomScript
///       typeHandlerVersion: '2.0'
///       settings: |2
///          {
///           "commandToExecute": "hostname && uptime"
///          }
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// Virtual Machine Extensions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/extension:Extension example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/virtualMachines/myVM/extensions/extensionName
/// ```
///
pub mod extension {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExtensionArgs {
        /// Specifies if the platform deploys the latest minor version update to the `type_handler_version` specified.
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
        /// The name of the virtual machine extension peering. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The protected_settings passed to the extension, like settings, these are specified as a JSON object in a string.
        ///
        /// > **Please Note:** Certain VM Extensions require that the keys in the `protected_settings` block are case sensitive. If you're seeing unhelpful errors, please ensure the keys are consistent with how Azure is expecting them (for instance, for the `JsonADDomainExtension` extension, the keys are expected to be in `TitleCase`.)
        #[builder(into, default)]
        pub protected_settings: pulumi_wasm_rust::Output<Option<String>>,
        /// A `protected_settings_from_key_vault` block as defined below.
        ///
        /// > **Note:** `protected_settings_from_key_vault` cannot be used with `protected_settings`
        #[builder(into, default)]
        pub protected_settings_from_key_vault: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::ExtensionProtectedSettingsFromKeyVault>,
        >,
        /// Specifies the collection of extension names after which this extension needs to be provisioned.
        #[builder(into, default)]
        pub provision_after_extensions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The publisher of the extension, available publishers can be found by using the Azure CLI. Changing this forces a new resource to be created.
        #[builder(into)]
        pub publisher: pulumi_wasm_rust::Output<String>,
        /// The settings passed to the extension, these are specified as a JSON object in a string.
        ///
        /// > **Please Note:** Certain VM Extensions require that the keys in the `settings` block are case sensitive. If you're seeing unhelpful errors, please ensure the keys are consistent with how Azure is expecting them (for instance, for the `JsonADDomainExtension` extension, the keys are expected to be in `TitleCase`.)
        #[builder(into, default)]
        pub settings: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of extension, available types for a publisher can be found using the Azure CLI.
        ///
        /// > **Note:** The `Publisher` and `Type` of Virtual Machine Extensions can be found using the Azure CLI, via:
        ///
        /// ```shell
        /// az vm extension image list --location westus -o table
        /// ```
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of the extension to use, available versions can be found using the Azure CLI.
        #[builder(into)]
        pub type_handler_version: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Machine. Changing this forces a new resource to be created
        #[builder(into)]
        pub virtual_machine_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ExtensionResult {
        /// Specifies if the platform deploys the latest minor version update to the `type_handler_version` specified.
        pub auto_upgrade_minor_version: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Extension be automatically updated whenever the Publisher releases a new version of this VM Extension?
        pub automatic_upgrade_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should failures from the extension be suppressed? Possible values are `true` or `false`. Defaults to `false`.
        ///
        /// > **NOTE:** Operational failures such as not connecting to the VM will not be suppressed regardless of the `failure_suppression_enabled` value.
        pub failure_suppression_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the virtual machine extension peering. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The protected_settings passed to the extension, like settings, these are specified as a JSON object in a string.
        ///
        /// > **Please Note:** Certain VM Extensions require that the keys in the `protected_settings` block are case sensitive. If you're seeing unhelpful errors, please ensure the keys are consistent with how Azure is expecting them (for instance, for the `JsonADDomainExtension` extension, the keys are expected to be in `TitleCase`.)
        pub protected_settings: pulumi_wasm_rust::Output<Option<String>>,
        /// A `protected_settings_from_key_vault` block as defined below.
        ///
        /// > **Note:** `protected_settings_from_key_vault` cannot be used with `protected_settings`
        pub protected_settings_from_key_vault: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::ExtensionProtectedSettingsFromKeyVault>,
        >,
        /// Specifies the collection of extension names after which this extension needs to be provisioned.
        pub provision_after_extensions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The publisher of the extension, available publishers can be found by using the Azure CLI. Changing this forces a new resource to be created.
        pub publisher: pulumi_wasm_rust::Output<String>,
        /// The settings passed to the extension, these are specified as a JSON object in a string.
        ///
        /// > **Please Note:** Certain VM Extensions require that the keys in the `settings` block are case sensitive. If you're seeing unhelpful errors, please ensure the keys are consistent with how Azure is expecting them (for instance, for the `JsonADDomainExtension` extension, the keys are expected to be in `TitleCase`.)
        pub settings: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of extension, available types for a publisher can be found using the Azure CLI.
        ///
        /// > **Note:** The `Publisher` and `Type` of Virtual Machine Extensions can be found using the Azure CLI, via:
        ///
        /// ```shell
        /// az vm extension image list --location westus -o table
        /// ```
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of the extension to use, available versions can be found using the Azure CLI.
        pub type_handler_version: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Machine. Changing this forces a new resource to be created
        pub virtual_machine_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ExtensionArgs) -> ExtensionResult {
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
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let type_handler_version_binding = args.type_handler_version.get_inner();
        let virtual_machine_id_binding = args.virtual_machine_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/extension:Extension".into(),
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
                register_interface::ObjectField {
                    name: "virtualMachineId".into(),
                    value: &virtual_machine_id_binding,
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
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "typeHandlerVersion".into(),
                },
                register_interface::ResultField {
                    name: "virtualMachineId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExtensionResult {
            auto_upgrade_minor_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoUpgradeMinorVersion").unwrap(),
            ),
            automatic_upgrade_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticUpgradeEnabled").unwrap(),
            ),
            failure_suppression_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failureSuppressionEnabled").unwrap(),
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
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            type_handler_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("typeHandlerVersion").unwrap(),
            ),
            virtual_machine_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMachineId").unwrap(),
            ),
        }
    }
}
