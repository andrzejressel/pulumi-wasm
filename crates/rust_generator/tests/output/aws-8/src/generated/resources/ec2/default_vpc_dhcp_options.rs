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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod default_vpc_dhcp_options {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultVpcDhcpOptionsArgs {
        /// The ID of the AWS account that owns the DHCP options set.
        #[builder(into, default)]
        pub owner_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DefaultVpcDhcpOptionsResult {
        /// The ARN of the DHCP Options Set.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        pub domain_name_servers: pulumi_gestalt_rust::Output<String>,
        pub ipv6_address_preferred_lease_time: pulumi_gestalt_rust::Output<String>,
        /// List of NETBIOS name servers.
        pub netbios_name_servers: pulumi_gestalt_rust::Output<String>,
        /// The NetBIOS node type (1, 2, 4, or 8). AWS recommends to specify 2 since broadcast and multicast are not supported in their network. For more information about these node types, see [RFC 2132](http://www.ietf.org/rfc/rfc2132.txt).
        pub netbios_node_type: pulumi_gestalt_rust::Output<String>,
        pub ntp_servers: pulumi_gestalt_rust::Output<String>,
        /// The ID of the AWS account that owns the DHCP options set.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultVpcDhcpOptionsArgs,
    ) -> DefaultVpcDhcpOptionsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let owner_id_binding = args.owner_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/defaultVpcDhcpOptions:DefaultVpcDhcpOptions".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownerId".into(),
                    value: owner_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefaultVpcDhcpOptionsResult {
            arn: o.get_field("arn"),
            domain_name: o.get_field("domainName"),
            domain_name_servers: o.get_field("domainNameServers"),
            ipv6_address_preferred_lease_time: o
                .get_field("ipv6AddressPreferredLeaseTime"),
            netbios_name_servers: o.get_field("netbiosNameServers"),
            netbios_node_type: o.get_field("netbiosNodeType"),
            ntp_servers: o.get_field("ntpServers"),
            owner_id: o.get_field("ownerId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
