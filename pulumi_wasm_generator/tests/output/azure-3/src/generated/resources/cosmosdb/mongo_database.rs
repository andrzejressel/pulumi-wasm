/// Manages a Mongo Database within a Cosmos DB Account.
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
///       throughput: 400
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
/// Cosmos Mongo Database can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/mongoDatabase:MongoDatabase db1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/account1/mongodbDatabases/db1
/// ```
///
pub mod mongo_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MongoDatabaseArgs {
        /// The name of the Cosmos DB Mongo Database to create the table within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// An `autoscale_settings` block as defined below. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        ///
        /// > **Note:** Switching between autoscale and manual throughput is not supported via this provider and must be completed via the Azure Portal and refreshed.
        #[builder(into, default)]
        pub autoscale_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::cosmosdb::MongoDatabaseAutoscaleSettings>,
        >,
        /// Specifies the name of the Cosmos DB Mongo Database. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the Cosmos DB Mongo Database is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The throughput of the MongoDB collection (RU/s). Must be set in increments of `100`. The minimum value is `400`. This must be set upon database creation otherwise it cannot be updated without a manual resource destroy-apply.
        ///
        /// > **Note:** throughput has a maximum value of `1000000` unless a higher limit is requested via Azure Support.
        #[builder(into, default)]
        pub throughput: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct MongoDatabaseResult {
        /// The name of the Cosmos DB Mongo Database to create the table within. Changing this forces a new resource to be created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// An `autoscale_settings` block as defined below. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        ///
        /// > **Note:** Switching between autoscale and manual throughput is not supported via this provider and must be completed via the Azure Portal and refreshed.
        pub autoscale_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::cosmosdb::MongoDatabaseAutoscaleSettings>,
        >,
        /// Specifies the name of the Cosmos DB Mongo Database. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Cosmos DB Mongo Database is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The throughput of the MongoDB collection (RU/s). Must be set in increments of `100`. The minimum value is `400`. This must be set upon database creation otherwise it cannot be updated without a manual resource destroy-apply.
        ///
        /// > **Note:** throughput has a maximum value of `1000000` unless a higher limit is requested via Azure Support.
        pub throughput: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MongoDatabaseArgs) -> MongoDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let autoscale_settings_binding = args.autoscale_settings.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let throughput_binding = args.throughput.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/mongoDatabase:MongoDatabase".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "autoscaleSettings".into(),
                    value: &autoscale_settings_binding,
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
                    name: "throughput".into(),
                    value: &throughput_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "autoscaleSettings".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
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
        MongoDatabaseResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            autoscale_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscaleSettings").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("throughput").unwrap(),
            ),
        }
    }
}
