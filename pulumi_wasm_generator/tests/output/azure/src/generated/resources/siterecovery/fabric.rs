/// Manages a Azure Site Recovery Replication Fabric within a Recovery Services vault. Only Azure fabrics are supported at this time. Replication Fabrics serve as a container within an Azure region for other Site Recovery resources such as protection containers, protected items, network mappings.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let fabric = fabric::create(
///         "fabric",
///         FabricArgs::builder()
///             .location("${primary.location}")
///             .name("primary-fabric")
///             .recovery_vault_name("${vault.name}")
///             .resource_group_name("${secondary.name}")
///             .build_struct(),
///     );
///     let primary = resource_group::create(
///         "primary",
///         ResourceGroupArgs::builder()
///             .location("West US")
///             .name("tfex-network-mapping-primary")
///             .build_struct(),
///     );
///     let secondary = resource_group::create(
///         "secondary",
///         ResourceGroupArgs::builder()
///             .location("East US")
///             .name("tfex-network-mapping-secondary")
///             .build_struct(),
///     );
///     let vault = vault::create(
///         "vault",
///         VaultArgs::builder()
///             .location("${secondary.location}")
///             .name("example-recovery-vault")
///             .resource_group_name("${secondary.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Site Recovery Fabric can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/fabric:Fabric myfabric /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource-group-name/providers/Microsoft.RecoveryServices/vaults/recovery-vault-name/replicationFabrics/fabric-name
/// ```
///
pub mod fabric {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FabricArgs {
        /// In what region should the fabric be located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the network mapping. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_wasm_rust::Output<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FabricResult {
        /// In what region should the fabric be located. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the network mapping. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_wasm_rust::Output<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FabricArgs) -> FabricResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let recovery_vault_name_binding = args.recovery_vault_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:siterecovery/fabric:Fabric".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recoveryVaultName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FabricResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recovery_vault_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryVaultName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}