/// Manages a Kusto (also known as Azure Data Explorer) Attached Database Configuration
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
///             .name("my-kusto-rg")
///             .build_struct(),
///     );
///     let exampleAttachedDatabaseConfiguration = attached_database_configuration::create(
///         "exampleAttachedDatabaseConfiguration",
///         AttachedDatabaseConfigurationArgs::builder()
///             .cluster_name("${followerCluster.name}")
///             .cluster_resource_id("${followedCluster.id}")
///             .database_name("${exampleDatabase.name}")
///             .location("${example.location}")
///             .name("configuration1")
///             .resource_group_name("${example.name}")
///             .sharing(
///                 AttachedDatabaseConfigurationSharing::builder()
///                     .externalTablesToExcludes(vec!["ExternalTable2",])
///                     .externalTablesToIncludes(vec!["ExternalTable1",])
///                     .materializedViewsToExcludes(vec!["MaterializedViewTable2",])
///                     .materializedViewsToIncludes(vec!["MaterializedViewTable1",])
///                     .tablesToExcludes(vec!["Table2",])
///                     .tablesToIncludes(vec!["Table1",])
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleDatabase = database::create(
///         "exampleDatabase",
///         DatabaseArgs::builder()
///             .cluster_name("${followerCluster.name}")
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let followedCluster = cluster::create(
///         "followedCluster",
///         ClusterArgs::builder()
///             .location("${example.location}")
///             .name("cluster2")
///             .resource_group_name("${example.name}")
///             .sku(
///                 ClusterSku::builder()
///                     .capacity(1)
///                     .name("Dev(No SLA)_Standard_D11_v2")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let followedDatabase = database::create(
///         "followedDatabase",
///         DatabaseArgs::builder()
///             .cluster_name("${followerCluster.name}")
///             .location("${example.location}")
///             .name("my-followed-database")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let followerCluster = cluster::create(
///         "followerCluster",
///         ClusterArgs::builder()
///             .location("${example.location}")
///             .name("cluster1")
///             .resource_group_name("${example.name}")
///             .sku(
///                 ClusterSku::builder()
///                     .capacity(1)
///                     .name("Dev(No SLA)_Standard_D11_v2")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Kusto Attached Database Configurations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/attachedDatabaseConfiguration:AttachedDatabaseConfiguration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Kusto/clusters/cluster1/attachedDatabaseConfigurations/configuration1
/// ```
///
pub mod attached_database_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AttachedDatabaseConfigurationArgs {
        /// Specifies the name of the Kusto Cluster for which the configuration will be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource id of the cluster where the databases you would like to attach reside. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_resource_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the database which you would like to attach, use * if you want to follow all current and future databases. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The default principals modification kind. Valid values are: `None` (default), `Replace` and `Union`. Defaults to `None`.
        #[builder(into, default)]
        pub default_principal_modification_kind: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the location of the Kusto Cluster for which the configuration will be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Kusto Attached Database Configuration to create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the resource group of the Kusto Cluster for which the configuration will be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `sharing` block as defined below.
        #[builder(into, default)]
        pub sharing: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::kusto::AttachedDatabaseConfigurationSharing>,
        >,
    }
    #[allow(dead_code)]
    pub struct AttachedDatabaseConfigurationResult {
        /// The list of databases from the `cluster_resource_id` which are currently attached to the cluster.
        pub attached_database_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the name of the Kusto Cluster for which the configuration will be created. Changing this forces a new resource to be created.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The resource id of the cluster where the databases you would like to attach reside. Changing this forces a new resource to be created.
        pub cluster_resource_id: pulumi_wasm_rust::Output<String>,
        /// The name of the database which you would like to attach, use * if you want to follow all current and future databases. Changing this forces a new resource to be created.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The default principals modification kind. Valid values are: `None` (default), `Replace` and `Union`. Defaults to `None`.
        pub default_principal_modification_kind: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Specifies the location of the Kusto Cluster for which the configuration will be created. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Kusto Attached Database Configuration to create. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the resource group of the Kusto Cluster for which the configuration will be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `sharing` block as defined below.
        pub sharing: pulumi_wasm_rust::Output<
            Option<super::super::types::kusto::AttachedDatabaseConfigurationSharing>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AttachedDatabaseConfigurationArgs,
    ) -> AttachedDatabaseConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let cluster_resource_id_binding = args
            .cluster_resource_id
            .get_output(context)
            .get_inner();
        let database_name_binding = args.database_name.get_output(context).get_inner();
        let default_principal_modification_kind_binding = args
            .default_principal_modification_kind
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sharing_binding = args.sharing.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:kusto/attachedDatabaseConfiguration:AttachedDatabaseConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "clusterResourceId".into(),
                    value: &cluster_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "defaultPrincipalModificationKind".into(),
                    value: &default_principal_modification_kind_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "sharing".into(),
                    value: &sharing_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attachedDatabaseNames".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "clusterResourceId".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "defaultPrincipalModificationKind".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sharing".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AttachedDatabaseConfigurationResult {
            attached_database_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachedDatabaseNames").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            cluster_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterResourceId").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            default_principal_modification_kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrincipalModificationKind").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sharing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharing").unwrap(),
            ),
        }
    }
}
