/// Manages a SQL Container within a Cosmos DB Account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleSqlDatabase:
///     type: azure:cosmosdb:SqlDatabase
///     name: example
///     properties:
///       name: example-acsd
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///   exampleSqlContainer:
///     type: azure:cosmosdb:SqlContainer
///     name: example
///     properties:
///       name: example-container
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///       databaseName: ${exampleSqlDatabase.name}
///       partitionKeyPaths:
///         - /definition/id
///       partitionKeyVersion: 1
///       throughput: 400
///       indexingPolicy:
///         indexingMode: consistent
///         includedPaths:
///           - path: /*
///           - path: /included/?
///         excludedPaths:
///           - path: /excluded/?
///       uniqueKeys:
///         - paths:
///             - /definition/idlong
///             - /definition/idshort
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
/// Cosmos SQL Containers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/sqlContainer:SqlContainer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DocumentDB/databaseAccounts/account1/sqlDatabases/database1/containers/container1
/// ```
///
pub mod sql_container {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlContainerArgs {
        /// The name of the Cosmos DB Account to create the container within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The default time to live of Analytical Storage for this SQL container. If present and the value is set to `-1`, it is equal to infinity, and items don’t expire by default. If present and the value is set to some number `n` – items will expire `n` seconds after their last modified time.
        #[builder(into, default)]
        pub analytical_storage_ttl: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// An `autoscale_settings` block as defined below. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        ///
        /// > **Note:** Switching between autoscale and manual throughput is not supported via this provider and must be completed via the Azure Portal and refreshed.
        #[builder(into, default)]
        pub autoscale_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::SqlContainerAutoscaleSettings>,
        >,
        /// A `conflict_resolution_policy` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub conflict_resolution_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::SqlContainerConflictResolutionPolicy>,
        >,
        /// The name of the Cosmos DB SQL Database to create the container within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The default time to live of SQL container. If missing, items are not expired automatically. If present and the value is set to `-1`, it is equal to infinity, and items don’t expire by default. If present and the value is set to some number `n` – items will expire `n` seconds after their last modified time.
        #[builder(into, default)]
        pub default_ttl: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// An `indexing_policy` block as defined below.
        #[builder(into, default)]
        pub indexing_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::SqlContainerIndexingPolicy>,
        >,
        /// Specifies the name of the Cosmos DB SQL Container. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Define a partition key kind. Possible values are `Hash` and `MultiHash`. Defaults to `Hash`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub partition_key_kind: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of partition key paths. Changing this forces a new resource to be created.
        #[builder(into)]
        pub partition_key_paths: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Define a partition key version. Possible values are `1`and `2`. This should be set to `2` in order to use large partition keys.
        ///
        /// > **Note:** If `partition_key_version` is not specified when creating a new resource, you can update `partition_key_version` to `1`, updating to `2` forces a new resource to be created.
        #[builder(into, default)]
        pub partition_key_version: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The name of the resource group in which the Cosmos DB SQL Container is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The throughput of SQL container (RU/s). Must be set in increments of `100`. The minimum value is `400`. This must be set upon container creation otherwise it cannot be updated without a manual resource destroy-apply.
        #[builder(into, default)]
        pub throughput: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// One or more `unique_key` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub unique_keys: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::cosmosdb::SqlContainerUniqueKey>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SqlContainerResult {
        /// The name of the Cosmos DB Account to create the container within. Changing this forces a new resource to be created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The default time to live of Analytical Storage for this SQL container. If present and the value is set to `-1`, it is equal to infinity, and items don’t expire by default. If present and the value is set to some number `n` – items will expire `n` seconds after their last modified time.
        pub analytical_storage_ttl: pulumi_wasm_rust::Output<Option<i32>>,
        /// An `autoscale_settings` block as defined below. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        ///
        /// > **Note:** Switching between autoscale and manual throughput is not supported via this provider and must be completed via the Azure Portal and refreshed.
        pub autoscale_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::cosmosdb::SqlContainerAutoscaleSettings>,
        >,
        /// A `conflict_resolution_policy` blocks as defined below. Changing this forces a new resource to be created.
        pub conflict_resolution_policy: pulumi_wasm_rust::Output<
            super::super::types::cosmosdb::SqlContainerConflictResolutionPolicy,
        >,
        /// The name of the Cosmos DB SQL Database to create the container within. Changing this forces a new resource to be created.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The default time to live of SQL container. If missing, items are not expired automatically. If present and the value is set to `-1`, it is equal to infinity, and items don’t expire by default. If present and the value is set to some number `n` – items will expire `n` seconds after their last modified time.
        pub default_ttl: pulumi_wasm_rust::Output<Option<i32>>,
        /// An `indexing_policy` block as defined below.
        pub indexing_policy: pulumi_wasm_rust::Output<
            super::super::types::cosmosdb::SqlContainerIndexingPolicy,
        >,
        /// Specifies the name of the Cosmos DB SQL Container. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Define a partition key kind. Possible values are `Hash` and `MultiHash`. Defaults to `Hash`. Changing this forces a new resource to be created.
        pub partition_key_kind: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of partition key paths. Changing this forces a new resource to be created.
        pub partition_key_paths: pulumi_wasm_rust::Output<Vec<String>>,
        /// Define a partition key version. Possible values are `1`and `2`. This should be set to `2` in order to use large partition keys.
        ///
        /// > **Note:** If `partition_key_version` is not specified when creating a new resource, you can update `partition_key_version` to `1`, updating to `2` forces a new resource to be created.
        pub partition_key_version: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the resource group in which the Cosmos DB SQL Container is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The throughput of SQL container (RU/s). Must be set in increments of `100`. The minimum value is `400`. This must be set upon container creation otherwise it cannot be updated without a manual resource destroy-apply.
        pub throughput: pulumi_wasm_rust::Output<i32>,
        /// One or more `unique_key` blocks as defined below. Changing this forces a new resource to be created.
        pub unique_keys: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cosmosdb::SqlContainerUniqueKey>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SqlContainerArgs,
    ) -> SqlContainerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
        let indexing_policy_binding = args
            .indexing_policy
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let partition_key_kind_binding = args
            .partition_key_kind
            .get_output(context)
            .get_inner();
        let partition_key_paths_binding = args
            .partition_key_paths
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
            type_: "azure:cosmosdb/sqlContainer:SqlContainer".into(),
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
                    name: "indexingPolicy".into(),
                    value: &indexing_policy_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "partitionKeyKind".into(),
                    value: &partition_key_kind_binding,
                },
                register_interface::ObjectField {
                    name: "partitionKeyPaths".into(),
                    value: &partition_key_paths_binding,
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
                    name: "conflictResolutionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "defaultTtl".into(),
                },
                register_interface::ResultField {
                    name: "indexingPolicy".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "partitionKeyKind".into(),
                },
                register_interface::ResultField {
                    name: "partitionKeyPaths".into(),
                },
                register_interface::ResultField {
                    name: "partitionKeyVersion".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "throughput".into(),
                },
                register_interface::ResultField {
                    name: "uniqueKeys".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SqlContainerResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            analytical_storage_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("analyticalStorageTtl").unwrap(),
            ),
            autoscale_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscaleSettings").unwrap(),
            ),
            conflict_resolution_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conflictResolutionPolicy").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            default_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultTtl").unwrap(),
            ),
            indexing_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexingPolicy").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            partition_key_kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionKeyKind").unwrap(),
            ),
            partition_key_paths: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionKeyPaths").unwrap(),
            ),
            partition_key_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionKeyVersion").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("throughput").unwrap(),
            ),
            unique_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniqueKeys").unwrap(),
            ),
        }
    }
}
