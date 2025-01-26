/// Manages a Azure Site Recovery protection container. Protection containers serve as containers for replicated VMs and belong to a single region / recovery fabric. Protection containers can contain more than one replicated VM. To replicate a VM, a container must exist in both the source and target Azure regions.
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
///   fabric:
///     type: azure:siterecovery:Fabric
///     properties:
///       name: primary-fabric
///       resourceGroupName: ${secondary.name}
///       recoveryVaultName: ${vault.name}
///       location: ${primary.location}
///   protection-container:
///     type: azure:siterecovery:ProtectionContainer
///     properties:
///       name: protection-container
///       resourceGroupName: ${secondary.name}
///       recoveryVaultName: ${vault.name}
///       recoveryFabricName: ${fabric.name}
/// ```
///
/// ## Import
///
/// Site Recovery Protection Containers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/protectionContainer:ProtectionContainer mycontainer /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource-group-name/providers/Microsoft.RecoveryServices/vaults/recovery-vault-name/replicationFabrics/fabric-name/replicationProtectionContainers/protection-container-name
/// ```
///
pub mod protection_container {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProtectionContainerArgs {
        /// The name of the protection container. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of fabric that should contain this protection container. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_fabric_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProtectionContainerResult {
        /// The name of the protection container. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Name of fabric that should contain this protection container. Changing this forces a new resource to be created.
        pub recovery_fabric_name: pulumi_wasm_rust::Output<String>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_wasm_rust::Output<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProtectionContainerArgs,
    ) -> ProtectionContainerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let recovery_fabric_name_binding = args
            .recovery_fabric_name
            .get_output(context)
            .get_inner();
        let recovery_vault_name_binding = args
            .recovery_vault_name
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:siterecovery/protectionContainer:ProtectionContainer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryFabricName".into(),
                    value: &recovery_fabric_name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryVaultName".into(),
                    value: &recovery_vault_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProtectionContainerResult {
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            recovery_fabric_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("recoveryFabricName"),
            ),
            recovery_vault_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("recoveryVaultName"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
