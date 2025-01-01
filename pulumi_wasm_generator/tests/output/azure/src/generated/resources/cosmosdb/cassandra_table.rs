/// Manages a Cassandra Table within a Cosmos DB Cassandra Keyspace.
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
///             .name("tflex-cosmosdb-account-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .capabilities(
///                 vec![
///                     AccountCapability::builder().name("EnableCassandra").build_struct(),
///                 ],
///             )
///             .consistency_policy(
///                 AccountConsistencyPolicy::builder()
///                     .consistencyLevel("Strong")
///                     .build_struct(),
///             )
///             .geo_locations(
///                 vec![
///                     AccountGeoLocation::builder().failoverPriority(0)
///                     .location("${example.location}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("tfex-cosmosdb-account")
///             .offer_type("Standard")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleCassandraKeyspace = cassandra_keyspace::create(
///         "exampleCassandraKeyspace",
///         CassandraKeyspaceArgs::builder()
///             .account_name("${exampleAccount.name}")
///             .name("tfex-cosmos-cassandra-keyspace")
///             .resource_group_name("${exampleAccount.resourceGroupName}")
///             .throughput(400)
///             .build_struct(),
///     );
///     let exampleCassandraTable = cassandra_table::create(
///         "exampleCassandraTable",
///         CassandraTableArgs::builder()
///             .cassandra_keyspace_id("${exampleCassandraKeyspace.id}")
///             .name("testtable")
///             .schema(
///                 CassandraTableSchema::builder()
///                     .columns(
///                         vec![
///                             CassandraTableSchemaColumn::builder().name("test1"). type
///                             ("ascii").build_struct(),
///                             CassandraTableSchemaColumn::builder().name("test2"). type
///                             ("int").build_struct(),
///                         ],
///                     )
///                     .partitionKeys(
///                         vec![
///                             CassandraTableSchemaPartitionKey::builder().name("test1")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Cosmos Cassandra Table can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/cassandraTable:CassandraTable ks1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/account1/cassandraKeyspaces/ks1/tables/table1
/// ```
///
pub mod cassandra_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CassandraTableArgs {
        /// Time to live of the Analytical Storage. Possible values are between `-1` and `2147483647` except `0`. `-1` means the Analytical Storage never expires. Changing this forces a new resource to be created.
        ///
        /// > **Note:** throughput has a maximum value of `1000000` unless a higher limit is requested via Azure Support
        #[builder(into, default)]
        pub analytical_storage_ttl: pulumi_wasm_rust::Output<Option<i32>>,
        #[builder(into, default)]
        pub autoscale_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::cosmosdb::CassandraTableAutoscaleSettings>,
        >,
        /// The ID of the Cosmos DB Cassandra Keyspace to create the table within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cassandra_keyspace_id: pulumi_wasm_rust::Output<String>,
        /// Time to live of the Cosmos DB Cassandra table. Possible values are at least `-1`. `-1` means the Cassandra table never expires.
        #[builder(into, default)]
        pub default_ttl: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Cosmos DB Cassandra Table. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `schema` block as defined below.
        #[builder(into)]
        pub schema: pulumi_wasm_rust::Output<
            super::super::types::cosmosdb::CassandraTableSchema,
        >,
        #[builder(into, default)]
        pub throughput: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct CassandraTableResult {
        /// Time to live of the Analytical Storage. Possible values are between `-1` and `2147483647` except `0`. `-1` means the Analytical Storage never expires. Changing this forces a new resource to be created.
        ///
        /// > **Note:** throughput has a maximum value of `1000000` unless a higher limit is requested via Azure Support
        pub analytical_storage_ttl: pulumi_wasm_rust::Output<Option<i32>>,
        pub autoscale_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::cosmosdb::CassandraTableAutoscaleSettings>,
        >,
        /// The ID of the Cosmos DB Cassandra Keyspace to create the table within. Changing this forces a new resource to be created.
        pub cassandra_keyspace_id: pulumi_wasm_rust::Output<String>,
        /// Time to live of the Cosmos DB Cassandra table. Possible values are at least `-1`. `-1` means the Cassandra table never expires.
        pub default_ttl: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Cosmos DB Cassandra Table. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `schema` block as defined below.
        pub schema: pulumi_wasm_rust::Output<
            super::super::types::cosmosdb::CassandraTableSchema,
        >,
        pub throughput: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CassandraTableArgs) -> CassandraTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let analytical_storage_ttl_binding = args.analytical_storage_ttl.get_inner();
        let autoscale_settings_binding = args.autoscale_settings.get_inner();
        let cassandra_keyspace_id_binding = args.cassandra_keyspace_id.get_inner();
        let default_ttl_binding = args.default_ttl.get_inner();
        let name_binding = args.name.get_inner();
        let schema_binding = args.schema.get_inner();
        let throughput_binding = args.throughput.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/cassandraTable:CassandraTable".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "analyticalStorageTtl".into(),
                    value: &analytical_storage_ttl_binding,
                },
                register_interface::ObjectField {
                    name: "autoscaleSettings".into(),
                    value: &autoscale_settings_binding,
                },
                register_interface::ObjectField {
                    name: "cassandraKeyspaceId".into(),
                    value: &cassandra_keyspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "defaultTtl".into(),
                    value: &default_ttl_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "schema".into(),
                    value: &schema_binding,
                },
                register_interface::ObjectField {
                    name: "throughput".into(),
                    value: &throughput_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "analyticalStorageTtl".into(),
                },
                register_interface::ResultField {
                    name: "autoscaleSettings".into(),
                },
                register_interface::ResultField {
                    name: "cassandraKeyspaceId".into(),
                },
                register_interface::ResultField {
                    name: "defaultTtl".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "schema".into(),
                },
                register_interface::ResultField {
                    name: "throughput".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CassandraTableResult {
            analytical_storage_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("analyticalStorageTtl").unwrap(),
            ),
            autoscale_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscaleSettings").unwrap(),
            ),
            cassandra_keyspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cassandraKeyspaceId").unwrap(),
            ),
            default_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultTtl").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schema").unwrap(),
            ),
            throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("throughput").unwrap(),
            ),
        }
    }
}
