/// Manages a HyperV site recovery network mapping on Azure. A HyperV network mapping decides how to translate connected networks when a VM is migrated from HyperV VMM Center to Azure.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   target:
///     type: azure:core:ResourceGroup
///     properties:
///       name: tfex-network-mapping
///       location: East US
///   vault:
///     type: azure:recoveryservices:Vault
///     properties:
///       name: example-recovery-vault
///       location: ${target.location}
///       resourceGroupName: ${target.name}
///       sku: Standard
///   targetVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: target
///     properties:
///       name: network
///       resourceGroupName: ${target.name}
///       addressSpaces:
///         - 192.168.2.0/24
///       location: ${target.location}
///   recovery-mapping:
///     type: azure:siterecovery:HypervNetworkMapping
///     properties:
///       name: recovery-network-mapping
///       recoveryVaultId: ${vault.id}
///       sourceSystemCenterVirtualMachineManagerName: my-vmm-server
///       sourceNetworkName: my-vmm-network
///       targetNetworkId: ${targetVirtualNetwork.id}
/// ```
///
/// ## Import
///
/// Site Recovery Network Mapping can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/hypervNetworkMapping:HypervNetworkMapping azurerm_site_recovery_hyperv_network_mapping.mymapping /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource-group-name/providers/Microsoft.RecoveryServices/vaults/recovery-vault-name/replicationFabrics/primary-fabric-name/replicationNetworks/azureNetwork/replicationNetworkMappings/mapping-name
/// ```
///
pub mod hyperv_network_mapping {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HypervNetworkMappingArgs {
        /// The name of the HyperV network mapping. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Recovery Services Vault where the HyperV network mapping should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the primary network. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_network_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of source System Center Virtual Machine Manager where the source network exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_system_center_virtual_machine_manager_name: pulumi_wasm_rust::InputOrOutput<
            String,
        >,
        /// The id of the recovery network. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_network_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HypervNetworkMappingResult {
        /// The name of the HyperV network mapping. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Recovery Services Vault where the HyperV network mapping should be created. Changing this forces a new resource to be created.
        pub recovery_vault_id: pulumi_wasm_rust::Output<String>,
        /// The Name of the primary network. Changing this forces a new resource to be created.
        pub source_network_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of source System Center Virtual Machine Manager where the source network exists. Changing this forces a new resource to be created.
        pub source_system_center_virtual_machine_manager_name: pulumi_wasm_rust::Output<
            String,
        >,
        /// The id of the recovery network. Changing this forces a new resource to be created.
        pub target_network_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: HypervNetworkMappingArgs,
    ) -> HypervNetworkMappingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let recovery_vault_id_binding = args
            .recovery_vault_id
            .get_output(context)
            .get_inner();
        let source_network_name_binding = args
            .source_network_name
            .get_output(context)
            .get_inner();
        let source_system_center_virtual_machine_manager_name_binding = args
            .source_system_center_virtual_machine_manager_name
            .get_output(context)
            .get_inner();
        let target_network_id_binding = args
            .target_network_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:siterecovery/hypervNetworkMapping:HypervNetworkMapping".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryVaultId".into(),
                    value: &recovery_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "sourceNetworkName".into(),
                    value: &source_network_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceSystemCenterVirtualMachineManagerName".into(),
                    value: &source_system_center_virtual_machine_manager_name_binding,
                },
                register_interface::ObjectField {
                    name: "targetNetworkId".into(),
                    value: &target_network_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recoveryVaultId".into(),
                },
                register_interface::ResultField {
                    name: "sourceNetworkName".into(),
                },
                register_interface::ResultField {
                    name: "sourceSystemCenterVirtualMachineManagerName".into(),
                },
                register_interface::ResultField {
                    name: "targetNetworkId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HypervNetworkMappingResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recovery_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryVaultId").unwrap(),
            ),
            source_network_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceNetworkName").unwrap(),
            ),
            source_system_center_virtual_machine_manager_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceSystemCenterVirtualMachineManagerName").unwrap(),
            ),
            target_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetNetworkId").unwrap(),
            ),
        }
    }
}
