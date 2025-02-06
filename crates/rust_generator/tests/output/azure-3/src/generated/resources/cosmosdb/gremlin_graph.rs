/// Manages a Gremlin Graph within a Cosmos DB Account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleGremlinDatabase:
///     type: azure:cosmosdb:GremlinDatabase
///     name: example
///     properties:
///       name: tfex-cosmos-gremlin-db
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///   exampleGremlinGraph:
///     type: azure:cosmosdb:GremlinGraph
///     name: example
///     properties:
///       name: tfex-cosmos-gremlin-graph
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///       databaseName: ${exampleGremlinDatabase.name}
///       partitionKeyPath: /Example
///       throughput: 400
///       indexPolicy:
///         automatic: true
///         indexingMode: consistent
///         includedPaths:
///           - /*
///         excludedPaths:
///           - /"_etag"/?
///       conflictResolutionPolicy:
///         mode: LastWriterWins
///         conflictResolutionPath: /_ts
///       uniqueKeys:
///         - paths:
///             - /definition/id1
///             - /definition/id2
/// variables:
///   example:
///     fn::invoke:
///       function: azure:cosmosdb:getAccount
///       arguments:
///         name: tfex-cosmosdb-account
///         resourceGroupName: tfex-cosmosdb-account-rg
/// ```
///
/// > **NOTE:** The CosmosDB Account needs to have the `EnableGremlin` capability enabled to use this resource - which can be done by adding this to the `capabilities` list within the `azure.cosmosdb.Account` resource.
///
/// ## Import
///
/// Cosmos Gremlin Graphs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/gremlinGraph:GremlinGraph example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/account1/gremlinDatabases/db1/graphs/graphs1
/// ```
///
pub mod gremlin_graph {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GremlinGraphArgs {
        /// The name of the CosmosDB Account to create the Gremlin Graph within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The time to live of Analytical Storage for this Cosmos DB Gremlin Graph. Possible values are between `-1` to `2147483647` not including `0`. If present and the value is set to `-1`, it means never expire.
        ///
        /// > **Note:** Disabling `analytical_storage_ttl` will force a new resource to be created since it can't be disabled once it's enabled.
        #[builder(into, default)]
        pub analytical_storage_ttl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub autoscale_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::GremlinGraphAutoscaleSettings>,
        >,
        /// A `conflict_resolution_policy` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub conflict_resolution_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::GremlinGraphConflictResolutionPolicy>,
        >,
        /// The name of the Cosmos DB Graph Database in which the Cosmos DB Gremlin Graph is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The default time to live (TTL) of the Gremlin graph. If the value is missing or set to "-1", items don’t expire.
        #[builder(into, default)]
        pub default_ttl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The configuration of the indexing policy. One or more `index_policy` blocks as defined below.
        #[builder(into, default)]
        pub index_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::GremlinGraphIndexPolicy>,
        >,
        /// Specifies the name of the Cosmos DB Gremlin Graph. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Define a partition key. Changing this forces a new resource to be created.
        #[builder(into)]
        pub partition_key_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Define a partition key version. Changing this forces a new resource to be created. Possible values are `1`and `2`. This should be set to `2` in order to use large partition keys.
        #[builder(into, default)]
        pub partition_key_version: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the resource group in which the Cosmos DB Gremlin Graph is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The throughput of the Gremlin graph (RU/s). Must be set in increments of `100`. The minimum value is `400`. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        #[builder(into, default)]
        pub throughput: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// One or more `unique_key` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub unique_keys: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cosmosdb::GremlinGraphUniqueKey>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GremlinGraphResult {
        /// The name of the CosmosDB Account to create the Gremlin Graph within. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// The time to live of Analytical Storage for this Cosmos DB Gremlin Graph. Possible values are between `-1` to `2147483647` not including `0`. If present and the value is set to `-1`, it means never expire.
        ///
        /// > **Note:** Disabling `analytical_storage_ttl` will force a new resource to be created since it can't be disabled once it's enabled.
        pub analytical_storage_ttl: pulumi_gestalt_rust::Output<Option<i32>>,
        pub autoscale_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::cosmosdb::GremlinGraphAutoscaleSettings>,
        >,
        /// A `conflict_resolution_policy` blocks as defined below. Changing this forces a new resource to be created.
        pub conflict_resolution_policy: pulumi_gestalt_rust::Output<
            super::super::types::cosmosdb::GremlinGraphConflictResolutionPolicy,
        >,
        /// The name of the Cosmos DB Graph Database in which the Cosmos DB Gremlin Graph is created. Changing this forces a new resource to be created.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The default time to live (TTL) of the Gremlin graph. If the value is missing or set to "-1", items don’t expire.
        pub default_ttl: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The configuration of the indexing policy. One or more `index_policy` blocks as defined below.
        pub index_policy: pulumi_gestalt_rust::Output<
            super::super::types::cosmosdb::GremlinGraphIndexPolicy,
        >,
        /// Specifies the name of the Cosmos DB Gremlin Graph. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Define a partition key. Changing this forces a new resource to be created.
        pub partition_key_path: pulumi_gestalt_rust::Output<String>,
        /// Define a partition key version. Changing this forces a new resource to be created. Possible values are `1`and `2`. This should be set to `2` in order to use large partition keys.
        pub partition_key_version: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the resource group in which the Cosmos DB Gremlin Graph is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The throughput of the Gremlin graph (RU/s). Must be set in increments of `100`. The minimum value is `400`. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        pub throughput: pulumi_gestalt_rust::Output<i32>,
        /// One or more `unique_key` blocks as defined below. Changing this forces a new resource to be created.
        pub unique_keys: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cosmosdb::GremlinGraphUniqueKey>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GremlinGraphArgs,
    ) -> GremlinGraphResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_output(context).get_inner();
        let analytical_storage_ttl_binding = args
            .analytical_storage_ttl
            .get_output(context)
            .get_inner();
        let autoscale_settings_binding = args
            .autoscale_settings
            .get_output(context)
            .get_inner();
        let conflict_resolution_policy_binding = args
            .conflict_resolution_policy
            .get_output(context)
            .get_inner();
        let database_name_binding = args.database_name.get_output(context).get_inner();
        let default_ttl_binding = args.default_ttl.get_output(context).get_inner();
        let index_policy_binding = args.index_policy.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let partition_key_path_binding = args
            .partition_key_path
            .get_output(context)
            .get_inner();
        let partition_key_version_binding = args
            .partition_key_version
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let throughput_binding = args.throughput.get_output(context).get_inner();
        let unique_keys_binding = args.unique_keys.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/gremlinGraph:GremlinGraph".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "conflictResolutionPolicy".into(),
                    value: &conflict_resolution_policy_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "defaultTtl".into(),
                    value: &default_ttl_binding,
                },
                register_interface::ObjectField {
                    name: "indexPolicy".into(),
                    value: &index_policy_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "partitionKeyPath".into(),
                    value: &partition_key_path_binding,
                },
                register_interface::ObjectField {
                    name: "partitionKeyVersion".into(),
                    value: &partition_key_version_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "throughput".into(),
                    value: &throughput_binding,
                },
                register_interface::ObjectField {
                    name: "uniqueKeys".into(),
                    value: &unique_keys_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GremlinGraphResult {
            account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            analytical_storage_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("analyticalStorageTtl"),
            ),
            autoscale_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoscaleSettings"),
            ),
            conflict_resolution_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("conflictResolutionPolicy"),
            ),
            database_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseName"),
            ),
            default_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultTtl"),
            ),
            index_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("indexPolicy"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            partition_key_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partitionKeyPath"),
            ),
            partition_key_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partitionKeyVersion"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            throughput: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("throughput"),
            ),
            unique_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("uniqueKeys"),
            ),
        }
    }
}
