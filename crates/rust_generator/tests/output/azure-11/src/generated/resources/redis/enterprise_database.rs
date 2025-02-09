/// Manages a Redis Enterprise Database.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod enterprise_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnterpriseDatabaseArgs {
        /// Specifies whether redis clients can connect using TLS-encrypted or plaintext redis protocols. Possible values are `Encrypted` and `Plaintext`. Defaults to `Encrypted`. Changing this forces a new Redis Enterprise Database to be created.
        #[builder(into, default)]
        pub client_protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource id of the Redis Enterprise Cluster to deploy this Redis Enterprise Database. Changing this forces a new Redis Enterprise Database to be created.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Clustering policy Specified at create time. Possible values are `EnterpriseCluster` and `OSSCluster`. Defaults to `OSSCluster`. Changing this forces a new Redis Enterprise Database to be created.
        #[builder(into, default)]
        pub clustering_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Redis eviction policy possible values are `AllKeysLFU`, `AllKeysLRU`, `AllKeysRandom`, `VolatileLRU`, `VolatileLFU`, `VolatileTTL`, `VolatileRandom` and `NoEviction`. Changing this forces a new Redis Enterprise Database to be created. Defaults to `VolatileLRU`.
        #[builder(into, default)]
        pub eviction_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Nickname of the group of linked databases. Changing this force a new Redis Enterprise Geo Database to be created.
        #[builder(into, default)]
        pub linked_database_group_nickname: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A list of database resources to link with this database with a maximum of 5.
        ///
        /// > **NOTE:** Only the newly created databases can be added to an existing geo-replication group. Existing regular databases or recreated databases cannot be added to the existing geo-replication group. Any linked database be removed from the list will be forcefully unlinked.The only recommended operation is to delete after force-unlink and the recommended scenario of force-unlink is region outrage. The database cannot be linked again after force-unlink.
        #[builder(into, default)]
        pub linked_database_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A `module` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Only `RediSearch` and `RedisJSON` modules are allowed with geo-replication
        #[builder(into, default)]
        pub modules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::redis::EnterpriseDatabaseModule>>,
        >,
        /// The name which should be used for this Redis Enterprise Database. Currently the acceptable value for this argument is `default`. Defaults to `default`. Changing this forces a new Redis Enterprise Database to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// TCP port of the database endpoint. Specified at create time. Defaults to an available port. Changing this forces a new Redis Enterprise Database to be created. Defaults to `10000`.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct EnterpriseDatabaseResult {
        /// Specifies whether redis clients can connect using TLS-encrypted or plaintext redis protocols. Possible values are `Encrypted` and `Plaintext`. Defaults to `Encrypted`. Changing this forces a new Redis Enterprise Database to be created.
        pub client_protocol: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource id of the Redis Enterprise Cluster to deploy this Redis Enterprise Database. Changing this forces a new Redis Enterprise Database to be created.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// Clustering policy Specified at create time. Possible values are `EnterpriseCluster` and `OSSCluster`. Defaults to `OSSCluster`. Changing this forces a new Redis Enterprise Database to be created.
        pub clustering_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Redis eviction policy possible values are `AllKeysLFU`, `AllKeysLRU`, `AllKeysRandom`, `VolatileLRU`, `VolatileLFU`, `VolatileTTL`, `VolatileRandom` and `NoEviction`. Changing this forces a new Redis Enterprise Database to be created. Defaults to `VolatileLRU`.
        pub eviction_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Nickname of the group of linked databases. Changing this force a new Redis Enterprise Geo Database to be created.
        pub linked_database_group_nickname: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of database resources to link with this database with a maximum of 5.
        ///
        /// > **NOTE:** Only the newly created databases can be added to an existing geo-replication group. Existing regular databases or recreated databases cannot be added to the existing geo-replication group. Any linked database be removed from the list will be forcefully unlinked.The only recommended operation is to delete after force-unlink and the recommended scenario of force-unlink is region outrage. The database cannot be linked again after force-unlink.
        pub linked_database_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A `module` block as defined below. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Only `RediSearch` and `RedisJSON` modules are allowed with geo-replication
        pub modules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::redis::EnterpriseDatabaseModule>>,
        >,
        /// The name which should be used for this Redis Enterprise Database. Currently the acceptable value for this argument is `default`. Defaults to `default`. Changing this forces a new Redis Enterprise Database to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// TCP port of the database endpoint. Specified at create time. Defaults to an available port. Changing this forces a new Redis Enterprise Database to be created. Defaults to `10000`.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The Primary Access Key for the Redis Enterprise Database Instance.
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Access Key for the Redis Enterprise Database Instance.
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EnterpriseDatabaseArgs,
    ) -> EnterpriseDatabaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let client_protocol_binding_1 = args.client_protocol.get_output(context);
        let client_protocol_binding = client_protocol_binding_1.get_inner();
        let cluster_id_binding_1 = args.cluster_id.get_output(context);
        let cluster_id_binding = cluster_id_binding_1.get_inner();
        let clustering_policy_binding_1 = args.clustering_policy.get_output(context);
        let clustering_policy_binding = clustering_policy_binding_1.get_inner();
        let eviction_policy_binding_1 = args.eviction_policy.get_output(context);
        let eviction_policy_binding = eviction_policy_binding_1.get_inner();
        let linked_database_group_nickname_binding_1 = args
            .linked_database_group_nickname
            .get_output(context);
        let linked_database_group_nickname_binding = linked_database_group_nickname_binding_1
            .get_inner();
        let linked_database_ids_binding_1 = args.linked_database_ids.get_output(context);
        let linked_database_ids_binding = linked_database_ids_binding_1.get_inner();
        let modules_binding_1 = args.modules.get_output(context);
        let modules_binding = modules_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let port_binding_1 = args.port.get_output(context);
        let port_binding = port_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnterpriseDatabaseResult {
            client_protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientProtocol"),
            ),
            cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterId"),
            ),
            clustering_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusteringPolicy"),
            ),
            eviction_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("evictionPolicy"),
            ),
            linked_database_group_nickname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linkedDatabaseGroupNickname"),
            ),
            linked_database_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linkedDatabaseIds"),
            ),
            modules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modules"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            primary_access_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryAccessKey"),
            ),
            secondary_access_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryAccessKey"),
            ),
        }
    }
}
