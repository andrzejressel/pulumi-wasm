/// Manages a Mongo Collection within a Cosmos DB Account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleMongoDatabase:
///     type: azure:cosmosdb:MongoDatabase
///     name: example
///     properties:
///       name: tfex-cosmos-mongo-db
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///   exampleMongoCollection:
///     type: azure:cosmosdb:MongoCollection
///     name: example
///     properties:
///       name: tfex-cosmos-mongo-db
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///       databaseName: ${exampleMongoDatabase.name}
///       defaultTtlSeconds: '777'
///       shardKey: uniqueKey
///       throughput: 400
///       indices:
///         - keys:
///             - _id
///           unique: true
/// variables:
///   example:
///     fn::invoke:
///       function: azure:cosmosdb:getAccount
///       arguments:
///         name: tfex-cosmosdb-account
///         resourceGroupName: tfex-cosmosdb-account-rg
/// ```
///
/// ## Import
///
/// CosmosDB Mongo Collection can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/mongoCollection:MongoCollection collection1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/account1/mongodbDatabases/db1/collections/collection1
/// ```
///
pub mod mongo_collection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MongoCollectionArgs {
        /// The name of the Cosmos DB Account in which the Cosmos DB Mongo Collection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The default time to live of Analytical Storage for this Mongo Collection. If present and the value is set to `-1`, it is equal to infinity, and items don’t expire by default. If present and the value is set to some number `n` – items will expire `n` seconds after their last modified time.
        #[builder(into, default)]
        pub analytical_storage_ttl: pulumi_wasm_rust::Output<Option<i32>>,
        #[builder(into, default)]
        pub autoscale_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::cosmosdb::MongoCollectionAutoscaleSettings>,
        >,
        /// The name of the Cosmos DB Mongo Database in which the Cosmos DB Mongo Collection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The default Time To Live in seconds. If the value is `-1`, items are not automatically expired.
        #[builder(into, default)]
        pub default_ttl_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// One or more `index` blocks as defined below.
        #[builder(into, default)]
        pub indices: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cosmosdb::MongoCollectionIndex>>,
        >,
        /// Specifies the name of the Cosmos DB Mongo Collection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the Cosmos DB Mongo Collection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the key to partition on for sharding. There must not be any other unique index keys. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub shard_key: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub throughput: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct MongoCollectionResult {
        /// The name of the Cosmos DB Account in which the Cosmos DB Mongo Collection is created. Changing this forces a new resource to be created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The default time to live of Analytical Storage for this Mongo Collection. If present and the value is set to `-1`, it is equal to infinity, and items don’t expire by default. If present and the value is set to some number `n` – items will expire `n` seconds after their last modified time.
        pub analytical_storage_ttl: pulumi_wasm_rust::Output<Option<i32>>,
        pub autoscale_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::cosmosdb::MongoCollectionAutoscaleSettings>,
        >,
        /// The name of the Cosmos DB Mongo Database in which the Cosmos DB Mongo Collection is created. Changing this forces a new resource to be created.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The default Time To Live in seconds. If the value is `-1`, items are not automatically expired.
        pub default_ttl_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// One or more `index` blocks as defined below.
        pub indices: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cosmosdb::MongoCollectionIndex>>,
        >,
        /// Specifies the name of the Cosmos DB Mongo Collection. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Cosmos DB Mongo Collection is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the key to partition on for sharding. There must not be any other unique index keys. Changing this forces a new resource to be created.
        pub shard_key: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `system_indexes` blocks as defined below.
        pub system_indexes: pulumi_wasm_rust::Output<
            Vec<super::super::types::cosmosdb::MongoCollectionSystemIndex>,
        >,
        pub throughput: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MongoCollectionArgs) -> MongoCollectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let analytical_storage_ttl_binding = args.analytical_storage_ttl.get_inner();
        let autoscale_settings_binding = args.autoscale_settings.get_inner();
        let database_name_binding = args.database_name.get_inner();
        let default_ttl_seconds_binding = args.default_ttl_seconds.get_inner();
        let indices_binding = args.indices.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let shard_key_binding = args.shard_key.get_inner();
        let throughput_binding = args.throughput.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/mongoCollection:MongoCollection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "analyticalStorageTtl".into(),
                    value: &analytical_storage_ttl_binding,
                },
                register_interface::ObjectField {
                    name: "autoscaleSettings".into(),
                    value: &autoscale_settings_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "defaultTtlSeconds".into(),
                    value: &default_ttl_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "indices".into(),
                    value: &indices_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "shardKey".into(),
                    value: &shard_key_binding,
                },
                register_interface::ObjectField {
                    name: "throughput".into(),
                    value: &throughput_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "analyticalStorageTtl".into(),
                },
                register_interface::ResultField {
                    name: "autoscaleSettings".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "defaultTtlSeconds".into(),
                },
                register_interface::ResultField {
                    name: "indices".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "shardKey".into(),
                },
                register_interface::ResultField {
                    name: "systemIndexes".into(),
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
        MongoCollectionResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            analytical_storage_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("analyticalStorageTtl").unwrap(),
            ),
            autoscale_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscaleSettings").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            default_ttl_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultTtlSeconds").unwrap(),
            ),
            indices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indices").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            shard_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shardKey").unwrap(),
            ),
            system_indexes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("systemIndexes").unwrap(),
            ),
            throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("throughput").unwrap(),
            ),
        }
    }
}
