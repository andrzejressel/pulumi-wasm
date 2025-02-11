/// Provides a VPC DHCP Options resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dnsResolver = vpc_dhcp_options::create(
///         "dnsResolver",
///         VpcDhcpOptionsArgs::builder()
///             .domain_name_servers(vec!["8.8.8.8", "8.8.4.4",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// Full usage:
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:ec2:VpcDhcpOptions
///     properties:
///       domainName: service.consul
///       domainNameServers:
///         - 127.0.0.1
///         - 10.0.0.2
///       ipv6AddressPreferredLeaseTime: 1440
///       ntpServers:
///         - 127.0.0.1
///       netbiosNameServers:
///         - 127.0.0.1
///       netbiosNodeType: 2
///       tags:
///         Name: foo-name
/// ```
///
/// ## Remarks
///
/// * Notice that all arguments are optional but you have to specify at least one argument.
/// * `domain_name_servers`, `netbios_name_servers`, `ntp_servers` are limited by AWS to maximum four servers only.
/// * To actually use the DHCP Options Set you need to associate it to a VPC using `aws.ec2.VpcDhcpOptionsAssociation`.
/// * If you delete a DHCP Options Set, all VPCs using it will be associated to AWS's `default` DHCP Option Set.
/// * In most cases unless you're configuring your own DNS you'll want to set `domain_name_servers` to `AmazonProvidedDNS`.
///
/// ## Import
///
/// Using `pulumi import`, import VPC DHCP Options using the DHCP Options `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcDhcpOptions:VpcDhcpOptions my_options dopt-d9070ebb
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_dhcp_options {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcDhcpOptionsArgs {
        /// the suffix domain name to use by default when resolving non Fully Qualified Domain Names. In other words, this is what ends up being the `search` value in the `/etc/resolv.conf` file.
        #[builder(into, default)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of name servers to configure in `/etc/resolv.conf`. If you want to use the default AWS nameservers you should set this to `AmazonProvidedDNS`.
        #[builder(into, default)]
        pub domain_name_servers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// How frequently, in seconds, a running instance with an IPv6 assigned to it goes through DHCPv6 lease renewal. Acceptable values are between 140 and 2147483647 (approximately 68 years). If no value is entered, the default lease time is 140 seconds. If you use long-term addressing for EC2 instances, you can increase the lease time and avoid frequent lease renewal requests. Lease renewal typically occurs when half of the lease time has elapsed.
        #[builder(into, default)]
        pub ipv6_address_preferred_lease_time: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// List of NETBIOS name servers.
        #[builder(into, default)]
        pub netbios_name_servers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The NetBIOS node type (1, 2, 4, or 8). AWS recommends to specify 2 since broadcast and multicast are not supported in their network. For more information about these node types, see [RFC 2132](http://www.ietf.org/rfc/rfc2132.txt).
        #[builder(into, default)]
        pub netbios_node_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of NTP servers to configure.
        #[builder(into, default)]
        pub ntp_servers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcDhcpOptionsResult {
        /// The ARN of the DHCP Options Set.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// the suffix domain name to use by default when resolving non Fully Qualified Domain Names. In other words, this is what ends up being the `search` value in the `/etc/resolv.conf` file.
        pub domain_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of name servers to configure in `/etc/resolv.conf`. If you want to use the default AWS nameservers you should set this to `AmazonProvidedDNS`.
        pub domain_name_servers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// How frequently, in seconds, a running instance with an IPv6 assigned to it goes through DHCPv6 lease renewal. Acceptable values are between 140 and 2147483647 (approximately 68 years). If no value is entered, the default lease time is 140 seconds. If you use long-term addressing for EC2 instances, you can increase the lease time and avoid frequent lease renewal requests. Lease renewal typically occurs when half of the lease time has elapsed.
        pub ipv6_address_preferred_lease_time: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// List of NETBIOS name servers.
        pub netbios_name_servers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The NetBIOS node type (1, 2, 4, or 8). AWS recommends to specify 2 since broadcast and multicast are not supported in their network. For more information about these node types, see [RFC 2132](http://www.ietf.org/rfc/rfc2132.txt).
        pub netbios_node_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of NTP servers to configure.
        pub ntp_servers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the AWS account that owns the DHCP options set.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: VpcDhcpOptionsArgs,
    ) -> VpcDhcpOptionsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let domain_name_servers_binding = args.domain_name_servers.get_output(context);
        let ipv6_address_preferred_lease_time_binding = args
            .ipv6_address_preferred_lease_time
            .get_output(context);
        let netbios_name_servers_binding = args.netbios_name_servers.get_output(context);
        let netbios_node_type_binding = args.netbios_node_type.get_output(context);
        let ntp_servers_binding = args.ntp_servers.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcDhcpOptions:VpcDhcpOptions".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainNameServers".into(),
                    value: &domain_name_servers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6AddressPreferredLeaseTime".into(),
                    value: &ipv6_address_preferred_lease_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "netbiosNameServers".into(),
                    value: &netbios_name_servers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "netbiosNodeType".into(),
                    value: &netbios_node_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ntpServers".into(),
                    value: &ntp_servers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcDhcpOptionsResult {
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
