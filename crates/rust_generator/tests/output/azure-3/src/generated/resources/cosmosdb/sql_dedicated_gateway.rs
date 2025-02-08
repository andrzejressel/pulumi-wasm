/// Manages a SQL Dedicated Gateway within a Cosmos DB Account.
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
///             .name("example-resource-group")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .consistency_policy(
///                 AccountConsistencyPolicy::builder()
///                     .consistencyLevel("BoundedStaleness")
///                     .build_struct(),
///             )
///             .geo_locations(
///                 vec![
///                     AccountGeoLocation::builder().failoverPriority(0)
///                     .location("${example.location}").build_struct(),
///                 ],
///             )
///             .kind("GlobalDocumentDB")
///             .location("${example.location}")
///             .name("example-ca")
///             .offer_type("Standard")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSqlDedicatedGateway = sql_dedicated_gateway::create(
///         "exampleSqlDedicatedGateway",
///         SqlDedicatedGatewayArgs::builder()
///             .cosmosdb_account_id("${exampleAccount.id}")
///             .instance_count(1)
///             .instance_size("Cosmos.D4s")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// CosmosDB SQL Dedicated Gateways can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/sqlDedicatedGateway:SqlDedicatedGateway example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.DocumentDB/databaseAccounts/account1/services/SqlDedicatedGateway
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sql_dedicated_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlDedicatedGatewayArgs {
        /// The resource ID of the CosmosDB Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cosmosdb_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The instance count for the CosmosDB SQL Dedicated Gateway. Possible value is between `1` and `5`.
        #[builder(into)]
        pub instance_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The instance size for the CosmosDB SQL Dedicated Gateway. Changing this forces a new resource to be created. Possible values are `Cosmos.D4s`, `Cosmos.D8s` and `Cosmos.D16s`.
        #[builder(into)]
        pub instance_size: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SqlDedicatedGatewayResult {
        /// The resource ID of the CosmosDB Account. Changing this forces a new resource to be created.
        pub cosmosdb_account_id: pulumi_gestalt_rust::Output<String>,
        /// The instance count for the CosmosDB SQL Dedicated Gateway. Possible value is between `1` and `5`.
        pub instance_count: pulumi_gestalt_rust::Output<i32>,
        /// The instance size for the CosmosDB SQL Dedicated Gateway. Changing this forces a new resource to be created. Possible values are `Cosmos.D4s`, `Cosmos.D8s` and `Cosmos.D16s`.
        pub instance_size: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SqlDedicatedGatewayArgs,
    ) -> SqlDedicatedGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cosmosdb_account_id_binding = args
            .cosmosdb_account_id
            .get_output(context)
            .get_inner();
        let instance_count_binding = args.instance_count.get_output(context).get_inner();
        let instance_size_binding = args.instance_size.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/sqlDedicatedGateway:SqlDedicatedGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cosmosdbAccountId".into(),
                    value: &cosmosdb_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding,
                },
                register_interface::ObjectField {
                    name: "instanceSize".into(),
                    value: &instance_size_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SqlDedicatedGatewayResult {
            cosmosdb_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cosmosdbAccountId"),
            ),
            instance_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceCount"),
            ),
            instance_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceSize"),
            ),
        }
    }
}
