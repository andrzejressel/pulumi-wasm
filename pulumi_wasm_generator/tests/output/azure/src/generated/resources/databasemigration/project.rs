/// Manage a Azure Database Migration Project.
///
/// > **NOTE:** Destroying a Database Migration Project will leave any outstanding tasks untouched. This is to avoid unexpectedly deleting any tasks managed outside of this provider.
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
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleProject = project::create(
///         "exampleProject",
///         ProjectArgs::builder()
///             .location("${example.location}")
///             .name("example-dbms-project")
///             .resource_group_name("${example.name}")
///             .service_name("${exampleService.name}")
///             .source_platform("SQL")
///             .target_platform("SQLDB")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-dbms")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard_1vCores")
///             .subnet_id("${exampleSubnet.id}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.1.0/24",])
///             .name("example-subnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-vnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Database Migration Projects can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:databasemigration/project:Project example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.DataMigration/services/example-dms/projects/project1
/// ```
///
pub mod project {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify the name of the database migration project. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the resource group in which to create the database migration project. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Name of the database migration service where resource belongs to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// The platform type of the migration source. Possible values are `SQL`, `PostgreSQL`, `MySQL` and `MongoDb`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_platform: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The platform type of the migration target. Possible values are `SQLDB`, `AzureDbForPostgreSql`, `AzureDbForMySql` and `MongoDb`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_platform: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specify the name of the database migration project. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Name of the resource group in which to create the database migration project. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Name of the database migration service where resource belongs to. Changing this forces a new resource to be created.
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// The platform type of the migration source. Possible values are `SQL`, `PostgreSQL`, `MySQL` and `MongoDb`. Changing this forces a new resource to be created.
        pub source_platform: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The platform type of the migration target. Possible values are `SQLDB`, `AzureDbForPostgreSql`, `AzureDbForMySql` and `MongoDb`. Changing this forces a new resource to be created.
        pub target_platform: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProjectArgs) -> ProjectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let service_name_binding = args.service_name.get_inner();
        let source_platform_binding = args.source_platform.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_platform_binding = args.target_platform.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:databasemigration/project:Project".into(),
            name: name.to_string(),
            object: Vec::from([
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
                    name: "serviceName".into(),
                    value: &service_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourcePlatform".into(),
                    value: &source_platform_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetPlatform".into(),
                    value: &target_platform_binding,
                },
            ]),
            results: Vec::from([
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
                    name: "serviceName".into(),
                },
                register_interface::ResultField {
                    name: "sourcePlatform".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "targetPlatform".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProjectResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
            source_platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourcePlatform").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            target_platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetPlatform").unwrap(),
            ),
        }
    }
}
