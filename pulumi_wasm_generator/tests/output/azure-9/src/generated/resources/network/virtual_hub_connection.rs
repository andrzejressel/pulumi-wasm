/// Manages a Connection for a Virtual Hub.
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
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .address_prefix("10.0.1.0/24")
///             .location("${example.location}")
///             .name("example-hub")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
///             .build_struct(),
///     );
///     let exampleVirtualHubConnection = virtual_hub_connection::create(
///         "exampleVirtualHubConnection",
///         VirtualHubConnectionArgs::builder()
///             .name("example-vhub")
///             .remote_virtual_network_id("${exampleVirtualNetwork.id}")
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["172.16.0.0/12",])
///             .location("${example.location}")
///             .name("example-network")
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
/// Virtual Hub Connection's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualHubConnection:VirtualHubConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/virtualHubs/hub1/hubVirtualNetworkConnections/connection1
/// ```
///
pub mod virtual_hub_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualHubConnectionArgs {
        /// Should Internet Security be enabled to secure internet traffic? Defaults to `false`.
        #[builder(into, default)]
        pub internet_security_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Name which should be used for this Connection, which must be unique within the Virtual Hub. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Virtual Network which the Virtual Hub should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub remote_virtual_network_id: pulumi_wasm_rust::Output<String>,
        /// A `routing` block as defined below.
        #[builder(into, default)]
        pub routing: pulumi_wasm_rust::Output<
            Option<super::super::types::network::VirtualHubConnectionRouting>,
        >,
        /// The ID of the Virtual Hub within which this connection should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualHubConnectionResult {
        /// Should Internet Security be enabled to secure internet traffic? Defaults to `false`.
        pub internet_security_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Name which should be used for this Connection, which must be unique within the Virtual Hub. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Network which the Virtual Hub should be connected to. Changing this forces a new resource to be created.
        pub remote_virtual_network_id: pulumi_wasm_rust::Output<String>,
        /// A `routing` block as defined below.
        pub routing: pulumi_wasm_rust::Output<
            super::super::types::network::VirtualHubConnectionRouting,
        >,
        /// The ID of the Virtual Hub within which this connection should be created. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VirtualHubConnectionArgs,
    ) -> VirtualHubConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let internet_security_enabled_binding = args
            .internet_security_enabled
            .get_inner();
        let name_binding = args.name.get_inner();
        let remote_virtual_network_id_binding = args
            .remote_virtual_network_id
            .get_inner();
        let routing_binding = args.routing.get_inner();
        let virtual_hub_id_binding = args.virtual_hub_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/virtualHubConnection:VirtualHubConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "internetSecurityEnabled".into(),
                    value: &internet_security_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "remoteVirtualNetworkId".into(),
                    value: &remote_virtual_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "routing".into(),
                    value: &routing_binding,
                },
                register_interface::ObjectField {
                    name: "virtualHubId".into(),
                    value: &virtual_hub_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "internetSecurityEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "remoteVirtualNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "routing".into(),
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
        VirtualHubConnectionResult {
            internet_security_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internetSecurityEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            remote_virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteVirtualNetworkId").unwrap(),
            ),
            routing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routing").unwrap(),
            ),
            virtual_hub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualHubId").unwrap(),
            ),
        }
    }
}
