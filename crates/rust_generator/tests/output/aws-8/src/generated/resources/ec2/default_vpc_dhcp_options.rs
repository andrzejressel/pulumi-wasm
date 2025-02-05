/// Provides a resource to manage the [default AWS DHCP Options Set](http://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/VPC_DHCP_Options.html#AmazonDNS)
/// in the current region.
///
/// Each AWS region comes with a default set of DHCP options.
/// **This is an advanced resource**, and has special caveats to be aware of when
/// using it. Please read this document in its entirety before using this resource.
///
/// The `aws.ec2.DefaultVpcDhcpOptions` behaves differently from normal resources, in that
/// this provider does not _create_ this resource, but instead "adopts" it
/// into management.
///
/// ## Example Usage
///
/// Basic usage with tags:
///
/// ```yaml
/// resources:
///   default:
///     type: aws:ec2:DefaultVpcDhcpOptions
///     properties:
///       tags:
///         Name: Default DHCP Option Set
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC DHCP Options using the DHCP Options `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/defaultVpcDhcpOptions:DefaultVpcDhcpOptions default_options dopt-d9070ebb
/// ```
pub mod default_vpc_dhcp_options {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultVpcDhcpOptionsArgs {
        /// The ID of the AWS account that owns the DHCP options set.
        #[builder(into, default)]
        pub owner_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DefaultVpcDhcpOptionsResult {
        /// The ARN of the DHCP Options Set.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub domain_name: pulumi_wasm_rust::Output<String>,
        pub domain_name_servers: pulumi_wasm_rust::Output<String>,
        pub ipv6_address_preferred_lease_time: pulumi_wasm_rust::Output<String>,
        /// List of NETBIOS name servers.
        pub netbios_name_servers: pulumi_wasm_rust::Output<String>,
        /// The NetBIOS node type (1, 2, 4, or 8). AWS recommends to specify 2 since broadcast and multicast are not supported in their network. For more information about these node types, see [RFC 2132](http://www.ietf.org/rfc/rfc2132.txt).
        pub netbios_node_type: pulumi_wasm_rust::Output<String>,
        pub ntp_servers: pulumi_wasm_rust::Output<String>,
        /// The ID of the AWS account that owns the DHCP options set.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
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
        args: DefaultVpcDhcpOptionsArgs,
    ) -> DefaultVpcDhcpOptionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let owner_id_binding = args.owner_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/defaultVpcDhcpOptions:DefaultVpcDhcpOptions".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ownerId".into(),
                    value: &owner_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DefaultVpcDhcpOptionsResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            domain_name_servers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainNameServers"),
            ),
            ipv6_address_preferred_lease_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipv6AddressPreferredLeaseTime"),
            ),
            netbios_name_servers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("netbiosNameServers"),
            ),
            netbios_node_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("netbiosNodeType"),
            ),
            ntp_servers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ntpServers"),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
