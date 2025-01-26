/// Manages a Redis Enterprise Database.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-redisenterprise")
///             .build_struct(),
///     );
///     let example1 = enterprise_cluster::create(
///         "example1",
///         EnterpriseClusterArgs::builder()
///             .location("${example.location}")
///             .name("example-redisenterprise1")
///             .resource_group_name("${example.name}")
///             .sku_name("Enterprise_E20-4")
///             .build_struct(),
///     );
///     let exampleEnterpriseCluster = enterprise_cluster::create(
///         "exampleEnterpriseCluster",
///         EnterpriseClusterArgs::builder()
///             .location("${example.location}")
///             .name("example-redisenterprise")
///             .resource_group_name("${example.name}")
///             .sku_name("Enterprise_E20-4")
///             .build_struct(),
///     );
///     let exampleEnterpriseDatabase = enterprise_database::create(
///         "exampleEnterpriseDatabase",
///         EnterpriseDatabaseArgs::builder()
///             .client_protocol("Encrypted")
///             .cluster_id("${exampleEnterpriseCluster.id}")
///             .clustering_policy("EnterpriseCluster")
///             .eviction_policy("NoEviction")
///             .linked_database_group_nickname("tftestGeoGroup")
///             .linked_database_ids(
///                 vec![
///                     "${exampleEnterpriseCluster.id}/databases/default",
///                     "${example1.id}/databases/default",
///                 ],
///             )
///             .name("default")
///             .port(10000)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Redis Enterprise Databases can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:redis/enterpriseDatabase:EnterpriseDatabase example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Cache/redisEnterprise/cluster1/databases/database1
/// ```
///
pub mod enterprise_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnterpriseDatabaseArgs {
        /// Specifies whether redis clients can connect using TLS-encrypted or plaintext redis protocols. Possible values are `Encrypted` and `Plaintext`. Defaults to `Encrypted`. Changing this forces a new Redis Enterprise Database to be created.
        #[builder(into, default)]
        pub client_protocol: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource id of the Redis Enterprise Cluster to deploy this Redis Enterprise Database. Changing this forces a new Redis Enterprise Database to be created.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Clustering policy Specified at create time. Possible values are `EnterpriseCluster` and `OSSCluster`. Defaults to `OSSCluster`. Changing this forces a new Redis Enterprise Database to be created.
        #[builder(into, default)]
        pub clustering_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Redis eviction policy possible values are `AllKeysLFU`, `AllKeysLRU`, `AllKeysRandom`, `VolatileLRU`, `VolatileLFU`, `VolatileTTL`, `VolatileRandom` and `NoEviction`. Changing this forces a new Redis Enterprise Database to be created. Defaults to `VolatileLRU`.
        #[builder(into, default)]
        pub eviction_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Nickname of the group of linked databases. Changing this force a new Redis Enterprise Geo Database to be created.
        #[builder(into, default)]
        pub linked_database_group_nickname: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// A list of database resources to link with this database with a maximum of 5.
        ///
        /// > **NOTE:** Only the newly created databases can be added to an existing geo-replication group. Existing regular databases or recreated databases cannot be added to the existing geo-replication group. Any linked database be removed from the list will be forcefully unlinked.The only recommended operation is to delete after force-unlink and the recommended scenario of force-unlink is region outrage. The database cannot be linked again after force-unlink.
        #[builder(into, default)]
        pub linked_database_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A `module` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Only `RediSearch` and `RedisJSON` modules are allowed with geo-replication
        #[builder(into, default)]
        pub modules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::redis::EnterpriseDatabaseModule>>,
        >,
        /// The name which should be used for this Redis Enterprise Database. Currently the acceptable value for this argument is `default`. Defaults to `default`. Changing this forces a new Redis Enterprise Database to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// TCP port of the database endpoint. Specified at create time. Defaults to an available port. Changing this forces a new Redis Enterprise Database to be created. Defaults to `10000`.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct EnterpriseDatabaseResult {
        /// Specifies whether redis clients can connect using TLS-encrypted or plaintext redis protocols. Possible values are `Encrypted` and `Plaintext`. Defaults to `Encrypted`. Changing this forces a new Redis Enterprise Database to be created.
        pub client_protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource id of the Redis Enterprise Cluster to deploy this Redis Enterprise Database. Changing this forces a new Redis Enterprise Database to be created.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// Clustering policy Specified at create time. Possible values are `EnterpriseCluster` and `OSSCluster`. Defaults to `OSSCluster`. Changing this forces a new Redis Enterprise Database to be created.
        pub clustering_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Redis eviction policy possible values are `AllKeysLFU`, `AllKeysLRU`, `AllKeysRandom`, `VolatileLRU`, `VolatileLFU`, `VolatileTTL`, `VolatileRandom` and `NoEviction`. Changing this forces a new Redis Enterprise Database to be created. Defaults to `VolatileLRU`.
        pub eviction_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Nickname of the group of linked databases. Changing this force a new Redis Enterprise Geo Database to be created.
        pub linked_database_group_nickname: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of database resources to link with this database with a maximum of 5.
        ///
        /// > **NOTE:** Only the newly created databases can be added to an existing geo-replication group. Existing regular databases or recreated databases cannot be added to the existing geo-replication group. Any linked database be removed from the list will be forcefully unlinked.The only recommended operation is to delete after force-unlink and the recommended scenario of force-unlink is region outrage. The database cannot be linked again after force-unlink.
        pub linked_database_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `module` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Only `RediSearch` and `RedisJSON` modules are allowed with geo-replication
        pub modules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::redis::EnterpriseDatabaseModule>>,
        >,
        /// The name which should be used for this Redis Enterprise Database. Currently the acceptable value for this argument is `default`. Defaults to `default`. Changing this forces a new Redis Enterprise Database to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// TCP port of the database endpoint. Specified at create time. Defaults to an available port. Changing this forces a new Redis Enterprise Database to be created. Defaults to `10000`.
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Primary Access Key for the Redis Enterprise Database Instance.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// The Secondary Access Key for the Redis Enterprise Database Instance.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EnterpriseDatabaseArgs,
    ) -> EnterpriseDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_protocol_binding = args
            .client_protocol
            .get_output(context)
            .get_inner();
        let cluster_id_binding = args.cluster_id.get_output(context).get_inner();
        let clustering_policy_binding = args
            .clustering_policy
            .get_output(context)
            .get_inner();
        let eviction_policy_binding = args
            .eviction_policy
            .get_output(context)
            .get_inner();
        let linked_database_group_nickname_binding = args
            .linked_database_group_nickname
            .get_output(context)
            .get_inner();
        let linked_database_ids_binding = args
            .linked_database_ids
            .get_output(context)
            .get_inner();
        let modules_binding = args.modules.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:redis/enterpriseDatabase:EnterpriseDatabase".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientProtocol".into(),
                    value: &client_protocol_binding,
                },
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "clusteringPolicy".into(),
                    value: &clustering_policy_binding,
                },
                register_interface::ObjectField {
                    name: "evictionPolicy".into(),
                    value: &eviction_policy_binding,
                },
                register_interface::ObjectField {
                    name: "linkedDatabaseGroupNickname".into(),
                    value: &linked_database_group_nickname_binding,
                },
                register_interface::ObjectField {
                    name: "linkedDatabaseIds".into(),
                    value: &linked_database_ids_binding,
                },
                register_interface::ObjectField {
                    name: "modules".into(),
                    value: &modules_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clientProtocol".into(),
                },
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "clusteringPolicy".into(),
                },
                register_interface::ResultField {
                    name: "evictionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "linkedDatabaseGroupNickname".into(),
                },
                register_interface::ResultField {
                    name: "linkedDatabaseIds".into(),
                },
                register_interface::ResultField {
                    name: "modules".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "primaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAccessKey".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnterpriseDatabaseResult {
            client_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientProtocol").unwrap(),
            ),
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            clustering_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusteringPolicy").unwrap(),
            ),
            eviction_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("evictionPolicy").unwrap(),
            ),
            linked_database_group_nickname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedDatabaseGroupNickname").unwrap(),
            ),
            linked_database_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedDatabaseIds").unwrap(),
            ),
            modules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modules").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAccessKey").unwrap(),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAccessKey").unwrap(),
            ),
        }
    }
}
