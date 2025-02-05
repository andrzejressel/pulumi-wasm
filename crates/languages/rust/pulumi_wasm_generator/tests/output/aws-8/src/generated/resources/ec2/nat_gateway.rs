/// Provides a resource to create a VPC NAT Gateway.
///
/// ## Example Usage
///
/// ### Public NAT
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:NatGateway
///     properties:
///       allocationId: ${exampleAwsEip.id}
///       subnetId: ${exampleAwsSubnet.id}
///       tags:
///         Name: gw NAT
///     options:
///       dependsOn:
///         - ${exampleAwsInternetGateway}
/// ```
///
/// ### Public NAT with Secondary Private IP Addresses
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = nat_gateway::create(
///         "example",
///         NatGatewayArgs::builder()
///             .allocation_id("${exampleAwsEip.id}")
///             .secondary_allocation_ids(vec!["${secondary.id}",])
///             .secondary_private_ip_addresses(vec!["10.0.1.5",])
///             .subnet_id("${exampleAwsSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Private NAT
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = nat_gateway::create(
///         "example",
///         NatGatewayArgs::builder()
///             .connectivity_type("private")
///             .subnet_id("${exampleAwsSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Private NAT with Secondary Private IP Addresses
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = nat_gateway::create(
///         "example",
///         NatGatewayArgs::builder()
///             .connectivity_type("private")
///             .secondary_private_ip_address_count(7)
///             .subnet_id("${exampleAwsSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import NAT Gateways using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/natGateway:NatGateway private_gw nat-05dba92075d71c408
/// ```
pub mod nat_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NatGatewayArgs {
        /// The Allocation ID of the Elastic IP address for the NAT Gateway. Required for `connectivity_type` of `public`.
        #[builder(into, default)]
        pub allocation_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Connectivity type for the NAT Gateway. Valid values are `private` and `public`. Defaults to `public`.
        #[builder(into, default)]
        pub connectivity_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The private IPv4 address to assign to the NAT Gateway. If you don't provide an address, a private IPv4 address will be automatically assigned.
        #[builder(into, default)]
        pub private_ip: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of secondary allocation EIP IDs for this NAT Gateway.
        #[builder(into, default)]
        pub secondary_allocation_ids: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// [Private NAT Gateway only] The number of secondary private IPv4 addresses you want to assign to the NAT Gateway.
        #[builder(into, default)]
        pub secondary_private_ip_address_count: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// A list of secondary private IPv4 addresses to assign to the NAT Gateway.
        #[builder(into, default)]
        pub secondary_private_ip_addresses: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The Subnet ID of the subnet in which to place the NAT Gateway.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NatGatewayResult {
        /// The Allocation ID of the Elastic IP address for the NAT Gateway. Required for `connectivity_type` of `public`.
        pub allocation_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The association ID of the Elastic IP address that's associated with the NAT Gateway. Only available when `connectivity_type` is `public`.
        pub association_id: pulumi_wasm_rust::Output<String>,
        /// Connectivity type for the NAT Gateway. Valid values are `private` and `public`. Defaults to `public`.
        pub connectivity_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the network interface associated with the NAT Gateway.
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// The private IPv4 address to assign to the NAT Gateway. If you don't provide an address, a private IPv4 address will be automatically assigned.
        pub private_ip: pulumi_wasm_rust::Output<String>,
        /// The Elastic IP address associated with the NAT Gateway.
        pub public_ip: pulumi_wasm_rust::Output<String>,
        /// A list of secondary allocation EIP IDs for this NAT Gateway.
        pub secondary_allocation_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// [Private NAT Gateway only] The number of secondary private IPv4 addresses you want to assign to the NAT Gateway.
        pub secondary_private_ip_address_count: pulumi_wasm_rust::Output<i32>,
        /// A list of secondary private IPv4 addresses to assign to the NAT Gateway.
        pub secondary_private_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Subnet ID of the subnet in which to place the NAT Gateway.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NatGatewayArgs,
    ) -> NatGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allocation_id_binding = args.allocation_id.get_output(context).get_inner();
        let connectivity_type_binding = args
            .connectivity_type
            .get_output(context)
            .get_inner();
        let private_ip_binding = args.private_ip.get_output(context).get_inner();
        let secondary_allocation_ids_binding = args
            .secondary_allocation_ids
            .get_output(context)
            .get_inner();
        let secondary_private_ip_address_count_binding = args
            .secondary_private_ip_address_count
            .get_output(context)
            .get_inner();
        let secondary_private_ip_addresses_binding = args
            .secondary_private_ip_addresses
            .get_output(context)
            .get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/natGateway:NatGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allocationId".into(),
                    value: &allocation_id_binding,
                },
                register_interface::ObjectField {
                    name: "connectivityType".into(),
                    value: &connectivity_type_binding,
                },
                register_interface::ObjectField {
                    name: "privateIp".into(),
                    value: &private_ip_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryAllocationIds".into(),
                    value: &secondary_allocation_ids_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryPrivateIpAddressCount".into(),
                    value: &secondary_private_ip_address_count_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryPrivateIpAddresses".into(),
                    value: &secondary_private_ip_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NatGatewayResult {
            allocation_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allocationId"),
            ),
            association_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("associationId"),
            ),
            connectivity_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectivityType"),
            ),
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkInterfaceId"),
            ),
            private_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateIp"),
            ),
            public_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicIp"),
            ),
            secondary_allocation_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryAllocationIds"),
            ),
            secondary_private_ip_address_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryPrivateIpAddressCount"),
            ),
            secondary_private_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryPrivateIpAddresses"),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
