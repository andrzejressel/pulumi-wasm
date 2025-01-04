/// Manages a PostgreSQL Virtual Network Rule.
///
/// > **NOTE:** PostgreSQL Virtual Network Rules [can only be used with SKU Tiers of `GeneralPurpose` or `MemoryOptimized`](https://docs.microsoft.com/azure/postgresql/concepts-data-access-and-security-vnet)
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
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("psqladmin")
///             .administrator_login_password("H@Sh1CoR3!")
///             .backup_retention_days(7)
///             .location("${example.location}")
///             .name("postgresql-server-1")
///             .resource_group_name("${example.name}")
///             .sku_name("GP_Gen5_2")
///             .ssl_enforcement_enabled(true)
///             .storage_mb(5120)
///             .version("9.5")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.7.29.0/29",])
///             .location("${example.location}")
///             .name("example-vnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetworkRule = virtual_network_rule::create(
///         "exampleVirtualNetworkRule",
///         VirtualNetworkRuleArgs::builder()
///             .ignore_missing_vnet_service_endpoint(true)
///             .name("postgresql-vnet-rule")
///             .resource_group_name("${example.name}")
///             .server_name("${exampleServer.name}")
///             .subnet_id("${internal.id}")
///             .build_struct(),
///     );
///     let internal = subnet::create(
///         "internal",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.7.29.0/29",])
///             .name("internal")
///             .resource_group_name("${example.name}")
///             .service_endpoints(vec!["Microsoft.Sql",])
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PostgreSQL Virtual Network Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/virtualNetworkRule:VirtualNetworkRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.DBforPostgreSQL/servers/myserver/virtualNetworkRules/vnetrulename
/// ```
///
pub mod virtual_network_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkRuleArgs {
        /// Should the Virtual Network Rule be created before the Subnet has the Virtual Network Service Endpoint enabled?
        #[builder(into, default)]
        pub ignore_missing_vnet_service_endpoint: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the PostgreSQL virtual network rule. Cannot be empty and must only contain alphanumeric characters and hyphens. Cannot start with a number, and cannot start or end with a hyphen. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `name` must be between 1-128 characters long and must satisfy all of the requirements below:
        ///
        /// 1. Contains only alphanumeric and hyphen characters
        /// 2. Cannot start with a number or hyphen
        /// 3. Cannot end with a hyphen
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group where the PostgreSQL server resides. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the SQL Server to which this PostgreSQL virtual network rule will be applied to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the subnet that the PostgreSQL server will be connected to.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkRuleResult {
        /// Should the Virtual Network Rule be created before the Subnet has the Virtual Network Service Endpoint enabled?
        pub ignore_missing_vnet_service_endpoint: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the PostgreSQL virtual network rule. Cannot be empty and must only contain alphanumeric characters and hyphens. Cannot start with a number, and cannot start or end with a hyphen. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `name` must be between 1-128 characters long and must satisfy all of the requirements below:
        ///
        /// 1. Contains only alphanumeric and hyphen characters
        /// 2. Cannot start with a number or hyphen
        /// 3. Cannot end with a hyphen
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group where the PostgreSQL server resides. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the SQL Server to which this PostgreSQL virtual network rule will be applied to. Changing this forces a new resource to be created.
        pub server_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the subnet that the PostgreSQL server will be connected to.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VirtualNetworkRuleArgs) -> VirtualNetworkRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ignore_missing_vnet_service_endpoint_binding = args
            .ignore_missing_vnet_service_endpoint
            .get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let server_name_binding = args.server_name.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:postgresql/virtualNetworkRule:VirtualNetworkRule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ignoreMissingVnetServiceEndpoint".into(),
                    value: &ignore_missing_vnet_service_endpoint_binding,
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
                    name: "serverName".into(),
                    value: &server_name_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "ignoreMissingVnetServiceEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serverName".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualNetworkRuleResult {
            ignore_missing_vnet_service_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ignoreMissingVnetServiceEndpoint").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            server_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverName").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
        }
    }
}
