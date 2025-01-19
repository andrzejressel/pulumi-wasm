/// Manages an Azure Cosmos DB for PostgreSQL Role.
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
pub mod postgresql_role {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PostgresqlRoleArgs {
        /// The resource ID of the Azure Cosmos DB for PostgreSQL Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Azure Cosmos DB for PostgreSQL Role. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The password of the Azure Cosmos DB for PostgreSQL Role. Changing this forces a new resource to be created.
        #[builder(into)]
        pub password: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PostgresqlRoleResult {
        /// The resource ID of the Azure Cosmos DB for PostgreSQL Cluster. Changing this forces a new resource to be created.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Azure Cosmos DB for PostgreSQL Role. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The password of the Azure Cosmos DB for PostgreSQL Role. Changing this forces a new resource to be created.
        pub password: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PostgresqlRoleArgs) -> PostgresqlRoleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding = args.cluster_id.get_inner();
        let name_binding = args.name.get_inner();
        let password_binding = args.password.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PostgresqlRoleResult {
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
        }
    }
}
