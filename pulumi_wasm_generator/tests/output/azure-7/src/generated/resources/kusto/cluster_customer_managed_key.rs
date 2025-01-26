/// Manages a Customer Managed Key for a Kusto Cluster.
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
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekv
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       purgeProtectionEnabled: true
///   cluster:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${exampleCluster.identity.principalId}
///       keyPermissions:
///         - Get
///         - UnwrapKey
///         - WrapKey
///   client:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       keyPermissions:
///         - Get
///         - List
///         - Create
///         - Delete
///         - Recover
///         - GetRotationPolicy
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: tfex-key
///       keyVaultId: ${exampleKeyVault.id}
///       keyType: RSA
///       keySize: 2048
///       keyOpts:
///         - decrypt
///         - encrypt
///         - sign
///         - unwrapKey
///         - verify
///         - wrapKey
///     options:
///       dependsOn:
///         - ${client}
///         - ${cluster}
///   exampleCluster:
///     type: azure:kusto:Cluster
///     name: example
///     properties:
///       name: kustocluster
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         name: Standard_D13_v2
///         capacity: 2
///       identity:
///         type: SystemAssigned
///   exampleClusterCustomerManagedKey:
///     type: azure:kusto:ClusterCustomerManagedKey
///     name: example
///     properties:
///       clusterId: ${exampleCluster.id}
///       keyVaultId: ${exampleKeyVault.id}
///       keyName: ${exampleKey.name}
///       keyVersion: ${exampleKey.version}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Customer Managed Keys for a Kusto Cluster can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/clusterCustomerManagedKey:ClusterCustomerManagedKey example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Kusto/clusters/cluster1
/// ```
///
pub mod cluster_customer_managed_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterCustomerManagedKeyArgs {
        /// The ID of the Kusto Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of Key Vault Key.
        #[builder(into)]
        pub key_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Key Vault.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The version of Key Vault Key.
        #[builder(into, default)]
        pub key_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The user assigned identity that has access to the Key Vault Key. If not specified, system assigned identity will be used.
        #[builder(into, default)]
        pub user_identity: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ClusterCustomerManagedKeyResult {
        /// The ID of the Kusto Cluster. Changing this forces a new resource to be created.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The name of Key Vault Key.
        pub key_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Key Vault.
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// The version of Key Vault Key.
        pub key_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The user assigned identity that has access to the Key Vault Key. If not specified, system assigned identity will be used.
        pub user_identity: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ClusterCustomerManagedKeyArgs,
    ) -> ClusterCustomerManagedKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding = args.cluster_id.get_output(context).get_inner();
        let key_name_binding = args.key_name.get_output(context).get_inner();
        let key_vault_id_binding = args.key_vault_id.get_output(context).get_inner();
        let key_version_binding = args.key_version.get_output(context).get_inner();
        let user_identity_binding = args.user_identity.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:kusto/clusterCustomerManagedKey:ClusterCustomerManagedKey"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyName".into(),
                    value: &key_name_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyVersion".into(),
                    value: &key_version_binding,
                },
                register_interface::ObjectField {
                    name: "userIdentity".into(),
                    value: &user_identity_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "keyName".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
                register_interface::ResultField {
                    name: "keyVersion".into(),
                },
                register_interface::ResultField {
                    name: "userIdentity".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterCustomerManagedKeyResult {
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyName").unwrap(),
            ),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            key_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVersion").unwrap(),
            ),
            user_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userIdentity").unwrap(),
            ),
        }
    }
}
