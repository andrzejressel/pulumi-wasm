/// Manages a Virtual Hub Route Table.
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
///     let exampleNetworkSecurityGroup = network_security_group::create(
///         "exampleNetworkSecurityGroup",
///         NetworkSecurityGroupArgs::builder()
///             .location("${example.location}")
///             .name("example-nsg")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.5.1.0/24",])
///             .name("examplesubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleSubnetNetworkSecurityGroupAssociation = subnet_network_security_group_association::create(
///         "exampleSubnetNetworkSecurityGroupAssociation",
///         SubnetNetworkSecurityGroupAssociationArgs::builder()
///             .network_security_group_id("${exampleNetworkSecurityGroup.id}")
///             .subnet_id("${exampleSubnet.id}")
///             .build_struct(),
///     );
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .address_prefix("10.0.2.0/24")
///             .location("${example.location}")
///             .name("example-vhub")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
///             .build_struct(),
///     );
///     let exampleVirtualHubConnection = virtual_hub_connection::create(
///         "exampleVirtualHubConnection",
///         VirtualHubConnectionArgs::builder()
///             .name("example-vhubconn")
///             .remote_virtual_network_id("${exampleVirtualNetwork.id}")
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
///     let exampleVirtualHubRouteTable = virtual_hub_route_table::create(
///         "exampleVirtualHubRouteTable",
///         VirtualHubRouteTableArgs::builder()
///             .labels(vec!["label1",])
///             .name("example-vhubroutetable")
///             .routes(
///                 vec![
///                     VirtualHubRouteTableRoute::builder()
///                     .destinations(vec!["10.0.0.0/16",]).destinationsType("CIDR")
///                     .name("example-route").nextHop("${exampleVirtualHubConnection.id}")
///                     .nextHopType("ResourceId").build_struct(),
///                 ],
///             )
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.5.0.0/16",])
///             .location("${example.location}")
///             .name("example-vnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVirtualWan = virtual_wan::create(
///         "exampleVirtualWan",
///         VirtualWanArgs::builder()
///             .location("${example.location}")
///             .name("example-vwan")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Hub Route Tables can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualHubRouteTable:VirtualHubRouteTable example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/virtualHubs/virtualHub1/hubRouteTables/routeTable1
/// ```
///
pub mod virtual_hub_route_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualHubRouteTableArgs {
        /// List of labels associated with this route table.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name which should be used for Virtual Hub Route Table. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `route` blocks as defined below.
        #[builder(into, default)]
        pub routes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::VirtualHubRouteTableRoute>>,
        >,
        /// The ID of the Virtual Hub within which this route table should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualHubRouteTableResult {
        /// List of labels associated with this route table.
        pub labels: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name which should be used for Virtual Hub Route Table. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `route` blocks as defined below.
        pub routes: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::VirtualHubRouteTableRoute>,
        >,
        /// The ID of the Virtual Hub within which this route table should be created. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VirtualHubRouteTableArgs,
    ) -> VirtualHubRouteTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let routes_binding = args.routes.get_inner();
        let virtual_hub_id_binding = args.virtual_hub_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/virtualHubRouteTable:VirtualHubRouteTable".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "routes".into(),
                    value: &routes_binding,
                },
                register_interface::ObjectField {
                    name: "virtualHubId".into(),
                    value: &virtual_hub_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "routes".into(),
                },
                register_interface::ResultField {
                    name: "virtualHubId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualHubRouteTableResult {
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routes").unwrap(),
            ),
            virtual_hub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualHubId").unwrap(),
            ),
        }
    }
}