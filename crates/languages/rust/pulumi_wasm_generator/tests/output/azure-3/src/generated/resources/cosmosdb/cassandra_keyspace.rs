/// Manages a Cassandra KeySpace within a Cosmos DB Account.
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
/// }
/// ```
///
/// ## Import
///
/// Cosmos Cassandra KeySpace can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/cassandraKeyspace:CassandraKeyspace ks1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/account1/cassandraKeyspaces/ks1
/// ```
///
pub mod cassandra_keyspace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CassandraKeyspaceArgs {
        /// The name of the Cosmos DB Cassandra KeySpace to create the table within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// An `autoscale_settings` block as defined below. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        ///
        /// > **Note:** Switching between autoscale and manual throughput is not supported via this provider and must be completed via the Azure Portal and refreshed.
        #[builder(into, default)]
        pub autoscale_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::CassandraKeyspaceAutoscaleSettings>,
        >,
        /// Specifies the name of the Cosmos DB Cassandra KeySpace. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Cosmos DB Cassandra KeySpace is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The throughput of Cassandra KeySpace (RU/s). Must be set in increments of `100`. The minimum value is `400`. This must be set upon database creation otherwise it cannot be updated without a manual resource destroy-apply.
        #[builder(into, default)]
        pub throughput: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct CassandraKeyspaceResult {
        /// The name of the Cosmos DB Cassandra KeySpace to create the table within. Changing this forces a new resource to be created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// An `autoscale_settings` block as defined below. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        ///
        /// > **Note:** Switching between autoscale and manual throughput is not supported via this provider and must be completed via the Azure Portal and refreshed.
        pub autoscale_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::cosmosdb::CassandraKeyspaceAutoscaleSettings>,
        >,
        /// Specifies the name of the Cosmos DB Cassandra KeySpace. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Cosmos DB Cassandra KeySpace is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The throughput of Cassandra KeySpace (RU/s). Must be set in increments of `100`. The minimum value is `400`. This must be set upon database creation otherwise it cannot be updated without a manual resource destroy-apply.
        pub throughput: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CassandraKeyspaceArgs,
    ) -> CassandraKeyspaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_output(context).get_inner();
        let autoscale_settings_binding = args
            .autoscale_settings
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let throughput_binding = args.throughput.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/cassandraKeyspace:CassandraKeyspace".into(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        CassandraKeyspaceResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            autoscale_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoscaleSettings"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            throughput: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("throughput"),
            ),
        }
    }
}
