/// Manages an Azure Cosmos DB for PostgreSQL Role.
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
///             .node_count(0)
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePostgresqlRole = postgresql_role::create(
///         "examplePostgresqlRole",
///         PostgresqlRoleArgs::builder()
///             .cluster_id("${examplePostgresqlCluster.id}")
///             .name("examplerole")
///             .password("H@Sh1CoR3!")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Cosmos DB for PostgreSQL Roles can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/postgresqlRole:PostgresqlRole example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.DBforPostgreSQL/serverGroupsv2/cluster1/roles/role1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod postgresql_role {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PostgresqlRoleArgs {
        /// The resource ID of the Azure Cosmos DB for PostgreSQL Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Azure Cosmos DB for PostgreSQL Role. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The password of the Azure Cosmos DB for PostgreSQL Role. Changing this forces a new resource to be created.
        #[builder(into)]
        pub password: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PostgresqlRoleResult {
        /// The resource ID of the Azure Cosmos DB for PostgreSQL Cluster. Changing this forces a new resource to be created.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Azure Cosmos DB for PostgreSQL Role. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The password of the Azure Cosmos DB for PostgreSQL Role. Changing this forces a new resource to be created.
        pub password: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PostgresqlRoleArgs,
    ) -> PostgresqlRoleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding_1 = args.cluster_id.get_output(context);
        let cluster_id_binding = cluster_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let password_binding_1 = args.password.get_output(context);
        let password_binding = password_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/postgresqlRole:PostgresqlRole".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "password".into(),
                    value: &password_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PostgresqlRoleResult {
            cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("password"),
            ),
        }
    }
}
