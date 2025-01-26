/// Manages a Cassandra Cluster.
///
/// > **NOTE:** In order for the `Azure Managed Instances for Apache Cassandra` to work properly the product requires the `Azure Cosmos DB` Application ID to be present and working in your tenant. If the `Azure Cosmos DB` Application ID is missing in your environment you will need to have an administrator of your tenant run the following command to add the `Azure Cosmos DB` Application ID to your tenant:
///
/// ```powershell
/// New-AzADServicePrincipal -ApplicationId a232010e-820c-4083-83bb-3ace5fc29d0b
/// ```
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: accexample-rg
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       addressSpaces:
///         - 10.0.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleVirtualNetwork.id}
///       roleDefinitionName: Network Contributor
///       principalId: ${example.objectId}
///   exampleCassandraCluster:
///     type: azure:cosmosdb:CassandraCluster
///     name: example
///     properties:
///       name: example-cluster
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       delegatedManagementSubnetId: ${exampleSubnet.id}
///       defaultAdminPassword: Password1234
///     options:
///       dependsOn:
///         - ${exampleAssignment}
/// variables:
///   example:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         displayName: Azure Cosmos DB
/// ```
///
/// ## Import
///
/// Cassandra Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/cassandraCluster:CassandraCluster example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.DocumentDB/cassandraClusters/cluster1
/// ```
///
pub mod cassandra_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CassandraClusterArgs {
        /// The authentication method that is used to authenticate clients. Possible values are `None` and `Cassandra`. Defaults to `Cassandra`.
        #[builder(into, default)]
        pub authentication_method: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of TLS certificates that is used to authorize client connecting to the Cassandra Cluster.
        #[builder(into, default)]
        pub client_certificate_pems: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The initial admin password for this Cassandra Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub default_admin_password: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the delegated management subnet for this Cassandra Cluster. Changing this forces a new Cassandra Cluster to be created.
        #[builder(into)]
        pub delegated_management_subnet_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of TLS certificates that is used to authorize gossip from unmanaged Cassandra Data Center.
        #[builder(into, default)]
        pub external_gossip_certificate_pems: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A list of IP Addresses of the seed nodes in unmanaged the Cassandra Data Center which will be added to the seed node lists of all managed nodes.
        #[builder(into, default)]
        pub external_seed_node_ip_addresses: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The number of hours to wait between taking a backup of the Cassandra Cluster. Defaults to `24`.
        ///
        /// > **Note:** To disable this feature, set this property to `0`.
        #[builder(into, default)]
        pub hours_between_backups: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::CassandraClusterIdentity>,
        >,
        /// The Azure Region where the Cassandra Cluster should exist. Changing this forces a new Cassandra Cluster to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Cassandra Cluster. Changing this forces a new Cassandra Cluster to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Is the automatic repair enabled on the Cassandra Cluster? Defaults to `true`.
        #[builder(into, default)]
        pub repair_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the Resource Group where the Cassandra Cluster should exist. Changing this forces a new Cassandra Cluster to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of Cassandra what the Cluster converges to run. Possible values are `3.11` and `4.0`. Defaults to `3.11`. Changing this forces a new Cassandra Cluster to be created.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CassandraClusterResult {
        /// The authentication method that is used to authenticate clients. Possible values are `None` and `Cassandra`. Defaults to `Cassandra`.
        pub authentication_method: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of TLS certificates that is used to authorize client connecting to the Cassandra Cluster.
        pub client_certificate_pems: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The initial admin password for this Cassandra Cluster. Changing this forces a new resource to be created.
        pub default_admin_password: pulumi_wasm_rust::Output<String>,
        /// The ID of the delegated management subnet for this Cassandra Cluster. Changing this forces a new Cassandra Cluster to be created.
        pub delegated_management_subnet_id: pulumi_wasm_rust::Output<String>,
        /// A list of TLS certificates that is used to authorize gossip from unmanaged Cassandra Data Center.
        pub external_gossip_certificate_pems: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// A list of IP Addresses of the seed nodes in unmanaged the Cassandra Data Center which will be added to the seed node lists of all managed nodes.
        pub external_seed_node_ip_addresses: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The number of hours to wait between taking a backup of the Cassandra Cluster. Defaults to `24`.
        ///
        /// > **Note:** To disable this feature, set this property to `0`.
        pub hours_between_backups: pulumi_wasm_rust::Output<Option<i32>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::cosmosdb::CassandraClusterIdentity>,
        >,
        /// The Azure Region where the Cassandra Cluster should exist. Changing this forces a new Cassandra Cluster to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Cassandra Cluster. Changing this forces a new Cassandra Cluster to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Is the automatic repair enabled on the Cassandra Cluster? Defaults to `true`.
        pub repair_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the Cassandra Cluster should exist. Changing this forces a new Cassandra Cluster to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of Cassandra what the Cluster converges to run. Possible values are `3.11` and `4.0`. Defaults to `3.11`. Changing this forces a new Cassandra Cluster to be created.
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CassandraClusterArgs,
    ) -> CassandraClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_method_binding = args
            .authentication_method
            .get_output(context)
            .get_inner();
        let client_certificate_pems_binding = args
            .client_certificate_pems
            .get_output(context)
            .get_inner();
        let default_admin_password_binding = args
            .default_admin_password
            .get_output(context)
            .get_inner();
        let delegated_management_subnet_id_binding = args
            .delegated_management_subnet_id
            .get_output(context)
            .get_inner();
        let external_gossip_certificate_pems_binding = args
            .external_gossip_certificate_pems
            .get_output(context)
            .get_inner();
        let external_seed_node_ip_addresses_binding = args
            .external_seed_node_ip_addresses
            .get_output(context)
            .get_inner();
        let hours_between_backups_binding = args
            .hours_between_backups
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let repair_enabled_binding = args.repair_enabled.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/cassandraCluster:CassandraCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationMethod".into(),
                    value: &authentication_method_binding,
                },
                register_interface::ObjectField {
                    name: "clientCertificatePems".into(),
                    value: &client_certificate_pems_binding,
                },
                register_interface::ObjectField {
                    name: "defaultAdminPassword".into(),
                    value: &default_admin_password_binding,
                },
                register_interface::ObjectField {
                    name: "delegatedManagementSubnetId".into(),
                    value: &delegated_management_subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "externalGossipCertificatePems".into(),
                    value: &external_gossip_certificate_pems_binding,
                },
                register_interface::ObjectField {
                    name: "externalSeedNodeIpAddresses".into(),
                    value: &external_seed_node_ip_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "hoursBetweenBackups".into(),
                    value: &hours_between_backups_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
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
                    name: "repairEnabled".into(),
                    value: &repair_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authenticationMethod".into(),
                },
                register_interface::ResultField {
                    name: "clientCertificatePems".into(),
                },
                register_interface::ResultField {
                    name: "defaultAdminPassword".into(),
                },
                register_interface::ResultField {
                    name: "delegatedManagementSubnetId".into(),
                },
                register_interface::ResultField {
                    name: "externalGossipCertificatePems".into(),
                },
                register_interface::ResultField {
                    name: "externalSeedNodeIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "hoursBetweenBackups".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "repairEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CassandraClusterResult {
            authentication_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationMethod").unwrap(),
            ),
            client_certificate_pems: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientCertificatePems").unwrap(),
            ),
            default_admin_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultAdminPassword").unwrap(),
            ),
            delegated_management_subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegatedManagementSubnetId").unwrap(),
            ),
            external_gossip_certificate_pems: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalGossipCertificatePems").unwrap(),
            ),
            external_seed_node_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalSeedNodeIpAddresses").unwrap(),
            ),
            hours_between_backups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hoursBetweenBackups").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            repair_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repairEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
