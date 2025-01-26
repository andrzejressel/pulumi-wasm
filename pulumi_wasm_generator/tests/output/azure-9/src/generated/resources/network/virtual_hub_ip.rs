/// Manages a Virtual Hub IP. This resource is also known as a Route Server.
///
/// > **NOTE** Virtual Hub IP only supports Standard Virtual Hub without Virtual Wan.
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
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("example-pip")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.5.1.0/24",])
///             .name("RouteServerSubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .location("${example.location}")
///             .name("example-vhub")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleVirtualHubIp = virtual_hub_ip::create(
///         "exampleVirtualHubIp",
///         VirtualHubIpArgs::builder()
///             .name("example-vhubipconfig")
///             .private_ip_address("10.5.1.18")
///             .private_ip_allocation_method("Static")
///             .public_ip_address_id("${examplePublicIp.id}")
///             .subnet_id("${exampleSubnet.id}")
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
/// }
/// ```
///
/// ## Import
///
/// Virtual Hub IPs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualHubIp:VirtualHubIp example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/virtualHubs/virtualHub1/ipConfigurations/ipConfig1
/// ```
///
pub mod virtual_hub_ip {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualHubIpArgs {
        /// The name which should be used for this Virtual Hub IP. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The private IP address of the IP configuration.
        #[builder(into, default)]
        pub private_ip_address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The private IP address allocation method. Possible values are `Static` and `Dynamic` is allowed. Defaults to `Dynamic`.
        #[builder(into, default)]
        pub private_ip_allocation_method: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the Public IP Address. This option is required since September 1st 2021. Changing this forces a new resource to be created.
        #[builder(into)]
        pub public_ip_address_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Subnet that the IP will reside. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Virtual Hub within which this IP configuration should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualHubIpResult {
        /// The name which should be used for this Virtual Hub IP. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The private IP address of the IP configuration.
        pub private_ip_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The private IP address allocation method. Possible values are `Static` and `Dynamic` is allowed. Defaults to `Dynamic`.
        pub private_ip_allocation_method: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Public IP Address. This option is required since September 1st 2021. Changing this forces a new resource to be created.
        pub public_ip_address_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet that the IP will reside. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Hub within which this IP configuration should be created. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VirtualHubIpArgs,
    ) -> VirtualHubIpResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let private_ip_address_binding = args
            .private_ip_address
            .get_output(context)
            .get_inner();
        let private_ip_allocation_method_binding = args
            .private_ip_allocation_method
            .get_output(context)
            .get_inner();
        let public_ip_address_id_binding = args
            .public_ip_address_id
            .get_output(context)
            .get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let virtual_hub_id_binding = args.virtual_hub_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/virtualHubIp:VirtualHubIp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateIpAddress".into(),
                    value: &private_ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "privateIpAllocationMethod".into(),
                    value: &private_ip_allocation_method_binding,
                },
                register_interface::ObjectField {
                    name: "publicIpAddressId".into(),
                    value: &public_ip_address_id_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "virtualHubId".into(),
                    value: &virtual_hub_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VirtualHubIpResult {
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            private_ip_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateIpAddress"),
            ),
            private_ip_allocation_method: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateIpAllocationMethod"),
            ),
            public_ip_address_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicIpAddressId"),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            virtual_hub_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualHubId"),
            ),
        }
    }
}
