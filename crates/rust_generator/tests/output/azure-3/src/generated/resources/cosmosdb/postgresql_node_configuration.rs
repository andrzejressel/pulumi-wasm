/// Sets a Node Configuration value on Azure Cosmos DB for PostgreSQL Cluster.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let examplePostgresqlCluster = postgresql_cluster::create(
///         "examplePostgresqlCluster",
///         PostgresqlClusterArgs::builder()
///             .administrator_login_password("H@Sh1CoR3!")
///             .coordinator_storage_quota_in_mb(131072)
///             .coordinator_vcore_count(2)
///             .location("${example.location}")
///             .name("examplecluster")
///             .node_count(2)
///             .node_storage_quota_in_mb(131072)
///             .node_vcores(2)
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePostgresqlNodeConfiguration = postgresql_node_configuration::create(
///         "examplePostgresqlNodeConfiguration",
///         PostgresqlNodeConfigurationArgs::builder()
///             .cluster_id("${examplePostgresqlCluster.id}")
///             .name("array_nulls")
///             .value("on")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Node Configurations on Azure Cosmos DB for PostgreSQL Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/postgresqlNodeConfiguration:PostgresqlNodeConfiguration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DBforPostgreSQL/serverGroupsv2/cluster1/nodeConfigurations/array_nulls
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod postgresql_node_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PostgresqlNodeConfigurationArgs {
        /// The resource ID of the Azure Cosmos DB for PostgreSQL Cluster where we want to change configuration. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Node Configuration on Azure Cosmos DB for PostgreSQL Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The value of the Node Configuration on Azure Cosmos DB for PostgreSQL Cluster.
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PostgresqlNodeConfigurationResult {
        /// The resource ID of the Azure Cosmos DB for PostgreSQL Cluster where we want to change configuration. Changing this forces a new resource to be created.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Node Configuration on Azure Cosmos DB for PostgreSQL Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The value of the Node Configuration on Azure Cosmos DB for PostgreSQL Cluster.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PostgresqlNodeConfigurationArgs,
    ) -> PostgresqlNodeConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_id_binding = args.cluster_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cosmosdb/postgresqlNodeConfiguration:PostgresqlNodeConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterId".into(),
                    value: cluster_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: value_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PostgresqlNodeConfigurationResult {
            cluster_id: o.get_field("clusterId"),
            name: o.get_field("name"),
            value: o.get_field("value"),
        }
    }
}
