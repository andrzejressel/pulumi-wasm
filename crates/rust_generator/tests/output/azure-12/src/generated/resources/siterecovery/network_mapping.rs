/// Manages a site recovery network mapping on Azure. A network mapping decides how to translate connected networks when a VM is migrated from one region to another.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   primary:
///     type: azure:core:ResourceGroup
///     properties:
///       name: tfex-network-mapping-primary
///       location: West US
///   secondary:
///     type: azure:core:ResourceGroup
///     properties:
///       name: tfex-network-mapping-secondary
///       location: East US
///   vault:
///     type: azure:recoveryservices:Vault
///     properties:
///       name: example-recovery-vault
///       location: ${secondary.location}
///       resourceGroupName: ${secondary.name}
///       sku: Standard
///   primaryFabric:
///     type: azure:siterecovery:Fabric
///     name: primary
///     properties:
///       name: primary-fabric
///       resourceGroupName: ${secondary.name}
///       recoveryVaultName: ${vault.name}
///       location: ${primary.location}
///   secondaryFabric:
///     type: azure:siterecovery:Fabric
///     name: secondary
///     properties:
///       name: secondary-fabric
///       resourceGroupName: ${secondary.name}
///       recoveryVaultName: ${vault.name}
///       location: ${secondary.location}
///     options:
///       dependsOn:
///         - ${primaryFabric}
///   primaryVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: primary
///     properties:
///       name: network1
///       resourceGroupName: ${primary.name}
///       addressSpaces:
///         - 192.168.1.0/24
///       location: ${primary.location}
///   secondaryVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: secondary
///     properties:
///       name: network2
///       resourceGroupName: ${secondary.name}
///       addressSpaces:
///         - 192.168.2.0/24
///       location: ${secondary.location}
///   recovery-mapping:
///     type: azure:siterecovery:NetworkMapping
///     properties:
///       name: recovery-network-mapping-1
///       resourceGroupName: ${secondary.name}
///       recoveryVaultName: ${vault.name}
///       sourceRecoveryFabricName: primary-fabric
///       targetRecoveryFabricName: secondary-fabric
///       sourceNetworkId: ${primaryVirtualNetwork.id}
///       targetNetworkId: ${secondaryVirtualNetwork.id}
/// ```
///
/// ## Import
///
/// Site Recovery Network Mapping can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/networkMapping:NetworkMapping mymapping /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource-group-name/providers/Microsoft.RecoveryServices/vaults/recovery-vault-name/replicationFabrics/primary-fabric-name/replicationNetworks/azureNetwork/replicationNetworkMappings/mapping-name
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_mapping {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkMappingArgs {
        /// The name of the network mapping. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the primary network. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the ASR fabric where mapping should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_recovery_fabric_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the recovery network. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Site Recovery fabric object corresponding to the recovery Azure region. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_recovery_fabric_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkMappingResult {
        /// The name of the network mapping. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The id of the primary network. Changing this forces a new resource to be created.
        pub source_network_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ASR fabric where mapping should be created. Changing this forces a new resource to be created.
        pub source_recovery_fabric_name: pulumi_gestalt_rust::Output<String>,
        /// The id of the recovery network. Changing this forces a new resource to be created.
        pub target_network_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Site Recovery fabric object corresponding to the recovery Azure region. Changing this forces a new resource to be created.
        pub target_recovery_fabric_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkMappingArgs,
    ) -> NetworkMappingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let recovery_vault_name_binding = args.recovery_vault_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let source_network_id_binding = args.source_network_id.get_output(context);
        let source_recovery_fabric_name_binding = args
            .source_recovery_fabric_name
            .get_output(context);
        let target_network_id_binding = args.target_network_id.get_output(context);
        let target_recovery_fabric_name_binding = args
            .target_recovery_fabric_name
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:siterecovery/networkMapping:NetworkMapping".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryVaultName".into(),
                    value: recovery_vault_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceNetworkId".into(),
                    value: source_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceRecoveryFabricName".into(),
                    value: source_recovery_fabric_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetNetworkId".into(),
                    value: target_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetRecoveryFabricName".into(),
                    value: target_recovery_fabric_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkMappingResult {
            name: o.get_field("name"),
            recovery_vault_name: o.get_field("recoveryVaultName"),
            resource_group_name: o.get_field("resourceGroupName"),
            source_network_id: o.get_field("sourceNetworkId"),
            source_recovery_fabric_name: o.get_field("sourceRecoveryFabricName"),
            target_network_id: o.get_field("targetNetworkId"),
            target_recovery_fabric_name: o.get_field("targetRecoveryFabricName"),
        }
    }
}
