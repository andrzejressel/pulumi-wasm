/// Provides a VPC DHCP Options resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod vpc_dhcp_options {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcDhcpOptionsArgs {
        /// the suffix domain name to use by default when resolving non Fully Qualified Domain Names. In other words, this is what ends up being the `search` value in the `/etc/resolv.conf` file.
        #[builder(into, default)]
        pub domain_name: pulumi_wasm_rust::Output<Option<String>>,
        /// List of name servers to configure in `/etc/resolv.conf`. If you want to use the default AWS nameservers you should set this to `AmazonProvidedDNS`.
        #[builder(into, default)]
        pub domain_name_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// How frequently, in seconds, a running instance with an IPv6 assigned to it goes through DHCPv6 lease renewal. Acceptable values are between 140 and 2147483647 (approximately 68 years). If no value is entered, the default lease time is 140 seconds. If you use long-term addressing for EC2 instances, you can increase the lease time and avoid frequent lease renewal requests. Lease renewal typically occurs when half of the lease time has elapsed.
        #[builder(into, default)]
        pub ipv6_address_preferred_lease_time: pulumi_wasm_rust::Output<Option<String>>,
        /// List of NETBIOS name servers.
        #[builder(into, default)]
        pub netbios_name_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The NetBIOS node type (1, 2, 4, or 8). AWS recommends to specify 2 since broadcast and multicast are not supported in their network. For more information about these node types, see [RFC 2132](http://www.ietf.org/rfc/rfc2132.txt).
        #[builder(into, default)]
        pub netbios_node_type: pulumi_wasm_rust::Output<Option<String>>,
        /// List of NTP servers to configure.
        #[builder(into, default)]
        pub ntp_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcDhcpOptionsResult {
        /// The ARN of the DHCP Options Set.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// the suffix domain name to use by default when resolving non Fully Qualified Domain Names. In other words, this is what ends up being the `search` value in the `/etc/resolv.conf` file.
        pub domain_name: pulumi_wasm_rust::Output<Option<String>>,
        /// List of name servers to configure in `/etc/resolv.conf`. If you want to use the default AWS nameservers you should set this to `AmazonProvidedDNS`.
        pub domain_name_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// How frequently, in seconds, a running instance with an IPv6 assigned to it goes through DHCPv6 lease renewal. Acceptable values are between 140 and 2147483647 (approximately 68 years). If no value is entered, the default lease time is 140 seconds. If you use long-term addressing for EC2 instances, you can increase the lease time and avoid frequent lease renewal requests. Lease renewal typically occurs when half of the lease time has elapsed.
        pub ipv6_address_preferred_lease_time: pulumi_wasm_rust::Output<Option<String>>,
        /// List of NETBIOS name servers.
        pub netbios_name_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The NetBIOS node type (1, 2, 4, or 8). AWS recommends to specify 2 since broadcast and multicast are not supported in their network. For more information about these node types, see [RFC 2132](http://www.ietf.org/rfc/rfc2132.txt).
        pub netbios_node_type: pulumi_wasm_rust::Output<Option<String>>,
        /// List of NTP servers to configure.
        pub ntp_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the AWS account that owns the DHCP options set.
        pub owner_id: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: VpcDhcpOptionsArgs) -> VpcDhcpOptionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_inner();
        let domain_name_servers_binding = args.domain_name_servers.get_inner();
        let ipv6_address_preferred_lease_time_binding = args
            .ipv6_address_preferred_lease_time
            .get_inner();
        let netbios_name_servers_binding = args.netbios_name_servers.get_inner();
        let netbios_node_type_binding = args.netbios_node_type.get_inner();
        let ntp_servers_binding = args.ntp_servers.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcDhcpOptions:VpcDhcpOptions".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "domainNameServers".into(),
                    value: &domain_name_servers_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6AddressPreferredLeaseTime".into(),
                    value: &ipv6_address_preferred_lease_time_binding,
                },
                register_interface::ObjectField {
                    name: "netbiosNameServers".into(),
                    value: &netbios_name_servers_binding,
                },
                register_interface::ObjectField {
                    name: "netbiosNodeType".into(),
                    value: &netbios_node_type_binding,
                },
                register_interface::ObjectField {
                    name: "ntpServers".into(),
                    value: &ntp_servers_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "domainNameServers".into(),
                },
                register_interface::ResultField {
                    name: "ipv6AddressPreferredLeaseTime".into(),
                },
                register_interface::ResultField {
                    name: "netbiosNameServers".into(),
                },
                register_interface::ResultField {
                    name: "netbiosNodeType".into(),
                },
                register_interface::ResultField {
                    name: "ntpServers".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcDhcpOptionsResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            domain_name_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainNameServers").unwrap(),
            ),
            ipv6_address_preferred_lease_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6AddressPreferredLeaseTime").unwrap(),
            ),
            netbios_name_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("netbiosNameServers").unwrap(),
            ),
            netbios_node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("netbiosNodeType").unwrap(),
            ),
            ntp_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ntpServers").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}