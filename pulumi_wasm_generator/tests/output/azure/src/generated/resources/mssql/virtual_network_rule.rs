/// Allows you to manage rules for allowing traffic between an Azure SQL server and a subnet of a virtual network.
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
///             .name("example-sql-server-vnet-rule")
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("4dm1n157r470r")
///             .administrator_login_password("4-v3ry-53cr37-p455w0rd")
///             .location("${example.location}")
///             .name("uniqueazuresqlserver")
///             .resource_group_name("${example.name}")
///             .version("12.0")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.7.29.0/29",])
///             .name("example-subnet")
///             .resource_group_name("${example.name}")
///             .service_endpoints(vec!["Microsoft.Sql",])
///             .virtual_network_name("${exampleVirtualNetwork.name}")
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
///             .name("sql-vnet-rule")
///             .server_id("${exampleServer.id}")
///             .subnet_id("${exampleSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// SQL Virtual Network Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/virtualNetworkRule:VirtualNetworkRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Sql/servers/myserver/virtualNetworkRules/vnetrulename
/// ```
///
pub mod virtual_network_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkRuleArgs {
        /// Create the virtual network rule before the subnet has the virtual network service endpoint enabled. Defaults to `false`.
        ///
        /// > **NOTE:** If `ignore_missing_vnet_service_endpoint` is false, and the target subnet does not contain the `Microsoft.SQL` endpoint in the `service_endpoints` array, the deployment will fail when it tries to create the SQL virtual network rule.
        #[builder(into, default)]
        pub ignore_missing_vnet_service_endpoint: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the SQL virtual network rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the SQL Server to which this SQL virtual network rule will be applied. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the subnet from which the SQL server will accept communications.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkRuleResult {
        /// Create the virtual network rule before the subnet has the virtual network service endpoint enabled. Defaults to `false`.
        ///
        /// > **NOTE:** If `ignore_missing_vnet_service_endpoint` is false, and the target subnet does not contain the `Microsoft.SQL` endpoint in the `service_endpoints` array, the deployment will fail when it tries to create the SQL virtual network rule.
        pub ignore_missing_vnet_service_endpoint: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the SQL virtual network rule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the SQL Server to which this SQL virtual network rule will be applied. Changing this forces a new resource to be created.
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the subnet from which the SQL server will accept communications.
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
        let server_id_binding = args.server_id.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/virtualNetworkRule:VirtualNetworkRule".into(),
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
                    name: "serverId".into(),
                    value: &server_id_binding,
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
                    name: "serverId".into(),
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
            server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverId").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
        }
    }
}