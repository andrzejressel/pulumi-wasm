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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mongo_collection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MongoCollectionArgs {
        /// The name of the Cosmos DB Account in which the Cosmos DB Mongo Collection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The default time to live of Analytical Storage for this Mongo Collection. If present and the value is set to `-1`, it is equal to infinity, and items don’t expire by default. If present and the value is set to some number `n` – items will expire `n` seconds after their last modified time.
        #[builder(into, default)]
        pub analytical_storage_ttl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub autoscale_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::MongoCollectionAutoscaleSettings>,
        >,
        /// The name of the Cosmos DB Mongo Database in which the Cosmos DB Mongo Collection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The default Time To Live in seconds. If the value is `-1`, items are not automatically expired.
        #[builder(into, default)]
        pub default_ttl_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// One or more `index` blocks as defined below.
        #[builder(into, default)]
        pub indices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cosmosdb::MongoCollectionIndex>>,
        >,
        /// Specifies the name of the Cosmos DB Mongo Collection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Cosmos DB Mongo Collection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the key to partition on for sharding. There must not be any other unique index keys. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub shard_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub throughput: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct MongoCollectionResult {
        /// The name of the Cosmos DB Account in which the Cosmos DB Mongo Collection is created. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// The default time to live of Analytical Storage for this Mongo Collection. If present and the value is set to `-1`, it is equal to infinity, and items don’t expire by default. If present and the value is set to some number `n` – items will expire `n` seconds after their last modified time.
        pub analytical_storage_ttl: pulumi_gestalt_rust::Output<Option<i32>>,
        pub autoscale_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::cosmosdb::MongoCollectionAutoscaleSettings>,
        >,
        /// The name of the Cosmos DB Mongo Database in which the Cosmos DB Mongo Collection is created. Changing this forces a new resource to be created.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The default Time To Live in seconds. If the value is `-1`, items are not automatically expired.
        pub default_ttl_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// One or more `index` blocks as defined below.
        pub indices: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cosmosdb::MongoCollectionIndex>>,
        >,
        /// Specifies the name of the Cosmos DB Mongo Collection. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Cosmos DB Mongo Collection is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the key to partition on for sharding. There must not be any other unique index keys. Changing this forces a new resource to be created.
        pub shard_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `system_indexes` blocks as defined below.
        pub system_indexes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cosmosdb::MongoCollectionSystemIndex>,
        >,
        pub throughput: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MongoCollectionArgs,
    ) -> MongoCollectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let analytical_storage_ttl_binding = args
            .analytical_storage_ttl
            .get_output(context);
        let autoscale_settings_binding = args.autoscale_settings.get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let default_ttl_seconds_binding = args.default_ttl_seconds.get_output(context);
        let indices_binding = args.indices.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let shard_key_binding = args.shard_key.get_output(context);
        let throughput_binding = args.throughput.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cosmosdb/mongoCollection:MongoCollection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "analyticalStorageTtl".into(),
                    value: analytical_storage_ttl_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscaleSettings".into(),
                    value: autoscale_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: database_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultTtlSeconds".into(),
                    value: default_ttl_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indices".into(),
                    value: indices_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shardKey".into(),
                    value: shard_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "throughput".into(),
                    value: throughput_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MongoCollectionResult {
            account_name: o.get_field("accountName"),
            analytical_storage_ttl: o.get_field("analyticalStorageTtl"),
            autoscale_settings: o.get_field("autoscaleSettings"),
            database_name: o.get_field("databaseName"),
            default_ttl_seconds: o.get_field("defaultTtlSeconds"),
            indices: o.get_field("indices"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            shard_key: o.get_field("shardKey"),
            system_indexes: o.get_field("systemIndexes"),
            throughput: o.get_field("throughput"),
        }
    }
}
