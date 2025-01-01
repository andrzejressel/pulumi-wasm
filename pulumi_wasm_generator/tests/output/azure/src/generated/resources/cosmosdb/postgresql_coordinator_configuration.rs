/// Sets a Coordinator Configuration value on Azure Cosmos DB for PostgreSQL Cluster.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = postgresql_cluster::create(
///         "example",
///         PostgresqlClusterArgs::builder()
///             .administrator_login_password("H@Sh1CoR3!")
///             .coordinator_storage_quota_in_mb(131072)
///             .coordinator_vcore_count(2)
///             .location("${exampleAzurermResourceGroup.location}")
///             .name("examplecluster")
///             .node_count(2)
///             .node_storage_quota_in_mb(131072)
///             .node_vcores(2)
///             .resource_group_name("${exampleAzurermResourceGroup.name}")
///             .build_struct(),
///     );
///     let examplePostgresqlCoordinatorConfiguration = postgresql_coordinator_configuration::create(
///         "examplePostgresqlCoordinatorConfiguration",
///         PostgresqlCoordinatorConfigurationArgs::builder()
///             .cluster_id("${example.id}")
///             .name("array_nulls")
///             .value("on")
///             .build_struct(),
///     );
///     let test = resource_group::create(
///         "test",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Coordinator Configurations on Azure Cosmos DB for PostgreSQL Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/postgresqlCoordinatorConfiguration:PostgresqlCoordinatorConfiguration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DBforPostgreSQL/serverGroupsv2/cluster1/coordinatorConfigurations/array_nulls
/// ```
///
pub mod postgresql_coordinator_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PostgresqlCoordinatorConfigurationArgs {
        /// The resource ID of the Azure Cosmos DB for PostgreSQL Cluster where we want to change configuration. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Coordinator Configuration on Azure Cosmos DB for PostgreSQL Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The value of the Coordinator Configuration on Azure Cosmos DB for PostgreSQL Cluster.
        #[builder(into)]
        pub value: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PostgresqlCoordinatorConfigurationResult {
        /// The resource ID of the Azure Cosmos DB for PostgreSQL Cluster where we want to change configuration. Changing this forces a new resource to be created.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Coordinator Configuration on Azure Cosmos DB for PostgreSQL Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The value of the Coordinator Configuration on Azure Cosmos DB for PostgreSQL Cluster.
        pub value: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: PostgresqlCoordinatorConfigurationArgs,
    ) -> PostgresqlCoordinatorConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding = args.cluster_id.get_inner();
        let name_binding = args.name.get_inner();
        let value_binding = args.value.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/postgresqlCoordinatorConfiguration:PostgresqlCoordinatorConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PostgresqlCoordinatorConfigurationResult {
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
        }
    }
}
