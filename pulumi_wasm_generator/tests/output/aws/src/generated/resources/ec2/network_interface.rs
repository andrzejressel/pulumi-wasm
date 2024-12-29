/// Provides an Elastic network interface (ENI) resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = network_interface::create(
///         "test",
///         NetworkInterfaceArgs::builder()
///             .attachments(
///                 vec![
///                     NetworkInterfaceAttachment::builder().deviceIndex(1)
///                     .instance("${testAwsInstance.id}").build_struct(),
///                 ],
///             )
///             .private_ips(vec!["10.0.0.50",])
///             .security_groups(vec!["${web.id}",])
///             .subnet_id("${publicA.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example of Managing Multiple IPs on a Network Interface
///
/// By default, private IPs are managed through the `private_ips` and `private_ips_count` arguments which manage IPs as a set of IPs that are configured without regard to order. For a new network interface, the same primary IP address is consistently selected from a given set of addresses, regardless of the order provided. However, modifications of the set of addresses of an existing interface will not alter the current primary IP address unless it has been removed from the set.
///
/// In order to manage the private IPs as a sequentially ordered list, configure `private_ip_list_enabled` to `true` and use `private_ip_list` to manage the IPs. This will disable the `private_ips` and `private_ips_count` settings, which must be removed from the config file but are still exported. Note that changing the first address of `private_ip_list`, which is the primary, always requires a new interface.
///
/// If you are managing a specific set or list of IPs, instead of just using `private_ips_count`, this is a potential workflow for also leveraging `private_ips_count` to have AWS automatically assign additional IP addresses:
///
/// 1. Comment out `private_ips`, `private_ip_list`, `private_ip_list_enabled` in your configuration
/// 2. Set the desired `private_ips_count` (count of the number of secondaries, the primary is not included)
/// 3. Apply to assign the extra IPs
/// 4. Remove `private_ips_count` and restore your settings from the first step
/// 5. Add the new IPs to your current settings
/// 6. Apply again to update the stored state
///
/// This process can also be used to remove IP addresses in addition to the option of manually removing them. Adding IP addresses in a manually is more difficult because it requires knowledge of which addresses are available.
///
/// ## Import
///
/// Using `pulumi import`, import Network Interfaces using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/networkInterface:NetworkInterface test eni-e5aa89a3
/// ```
pub mod network_interface {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInterfaceArgs {
        /// Configuration block to define the attachment of the ENI. See Attachment below for more details!
        #[builder(into, default)]
        pub attachments: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::NetworkInterfaceAttachment>>,
        >,
        /// Description for the network interface.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of network interface to create. Set to `efa` for Elastic Fabric Adapter. Changing `interface_type` will cause the resource to be destroyed and re-created.
        #[builder(into, default)]
        pub interface_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of IPv4 prefixes that AWS automatically assigns to the network interface.
        #[builder(into, default)]
        pub ipv4_prefix_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// One or more IPv4 prefixes assigned to the network interface.
        #[builder(into, default)]
        pub ipv4_prefixes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Number of IPv6 addresses to assign to a network interface. You can't use this option if specifying specific `ipv6_addresses`. If your subnet has the AssignIpv6AddressOnCreation attribute set to `true`, you can specify `0` to override this setting.
        #[builder(into, default)]
        pub ipv6_address_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Whether `ipv6_address_list` is allowed and controls the IPs to assign to the ENI and `ipv6_addresses` and `ipv6_address_count` become read-only. Default false.
        #[builder(into, default)]
        pub ipv6_address_list_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of private IPs to assign to the ENI in sequential order.
        #[builder(into, default)]
        pub ipv6_address_lists: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. Addresses are assigned without regard to order. You can't use this option if you're specifying `ipv6_address_count`.
        #[builder(into, default)]
        pub ipv6_addresses: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Number of IPv6 prefixes that AWS automatically assigns to the network interface.
        #[builder(into, default)]
        pub ipv6_prefix_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// One or more IPv6 prefixes assigned to the network interface.
        #[builder(into, default)]
        pub ipv6_prefixes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        #[builder(into, default)]
        pub private_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether `private_ip_list` is allowed and controls the IPs to assign to the ENI and `private_ips` and `private_ips_count` become read-only. Default false.
        #[builder(into, default)]
        pub private_ip_list_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of private IPs to assign to the ENI in sequential order. Requires setting `private_ip_list_enabled` to `true`.
        #[builder(into, default)]
        pub private_ip_lists: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of private IPs to assign to the ENI without regard to order.
        #[builder(into, default)]
        pub private_ips: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Number of secondary private IPs to assign to the ENI. The total number of private IPs will be 1 + `private_ips_count`, as a primary private IP will be assiged to an ENI by default.
        #[builder(into, default)]
        pub private_ips_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of security group IDs to assign to the ENI.
        #[builder(into, default)]
        pub security_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether to enable source destination checking for the ENI. Default true.
        #[builder(into, default)]
        pub source_dest_check: pulumi_wasm_rust::Output<Option<bool>>,
        /// Subnet ID to create the ENI in.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkInterfaceResult {
        /// ARN of the network interface.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block to define the attachment of the ENI. See Attachment below for more details!
        pub attachments: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::NetworkInterfaceAttachment>,
        >,
        /// Description for the network interface.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of network interface to create. Set to `efa` for Elastic Fabric Adapter. Changing `interface_type` will cause the resource to be destroyed and re-created.
        pub interface_type: pulumi_wasm_rust::Output<String>,
        /// Number of IPv4 prefixes that AWS automatically assigns to the network interface.
        pub ipv4_prefix_count: pulumi_wasm_rust::Output<i32>,
        /// One or more IPv4 prefixes assigned to the network interface.
        pub ipv4_prefixes: pulumi_wasm_rust::Output<Vec<String>>,
        /// Number of IPv6 addresses to assign to a network interface. You can't use this option if specifying specific `ipv6_addresses`. If your subnet has the AssignIpv6AddressOnCreation attribute set to `true`, you can specify `0` to override this setting.
        pub ipv6_address_count: pulumi_wasm_rust::Output<i32>,
        /// Whether `ipv6_address_list` is allowed and controls the IPs to assign to the ENI and `ipv6_addresses` and `ipv6_address_count` become read-only. Default false.
        pub ipv6_address_list_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of private IPs to assign to the ENI in sequential order.
        pub ipv6_address_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. Addresses are assigned without regard to order. You can't use this option if you're specifying `ipv6_address_count`.
        pub ipv6_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Number of IPv6 prefixes that AWS automatically assigns to the network interface.
        pub ipv6_prefix_count: pulumi_wasm_rust::Output<i32>,
        /// One or more IPv6 prefixes assigned to the network interface.
        pub ipv6_prefixes: pulumi_wasm_rust::Output<Vec<String>>,
        /// MAC address of the network interface.
        pub mac_address: pulumi_wasm_rust::Output<String>,
        pub outpost_arn: pulumi_wasm_rust::Output<String>,
        /// AWS account ID of the owner of the network interface.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Private DNS name of the network interface (IPv4).
        pub private_dns_name: pulumi_wasm_rust::Output<String>,
        pub private_ip: pulumi_wasm_rust::Output<String>,
        /// Whether `private_ip_list` is allowed and controls the IPs to assign to the ENI and `private_ips` and `private_ips_count` become read-only. Default false.
        pub private_ip_list_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of private IPs to assign to the ENI in sequential order. Requires setting `private_ip_list_enabled` to `true`.
        pub private_ip_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of private IPs to assign to the ENI without regard to order.
        pub private_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// Number of secondary private IPs to assign to the ENI. The total number of private IPs will be 1 + `private_ips_count`, as a primary private IP will be assiged to an ENI by default.
        pub private_ips_count: pulumi_wasm_rust::Output<i32>,
        /// List of security group IDs to assign to the ENI.
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether to enable source destination checking for the ENI. Default true.
        pub source_dest_check: pulumi_wasm_rust::Output<Option<bool>>,
        /// Subnet ID to create the ENI in.
        ///
        /// The following arguments are optional:
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkInterfaceArgs) -> NetworkInterfaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attachments_binding = args.attachments.get_inner();
        let description_binding = args.description.get_inner();
        let interface_type_binding = args.interface_type.get_inner();
        let ipv4_prefix_count_binding = args.ipv4_prefix_count.get_inner();
        let ipv4_prefixes_binding = args.ipv4_prefixes.get_inner();
        let ipv6_address_count_binding = args.ipv6_address_count.get_inner();
        let ipv6_address_list_enabled_binding = args
            .ipv6_address_list_enabled
            .get_inner();
        let ipv6_address_lists_binding = args.ipv6_address_lists.get_inner();
        let ipv6_addresses_binding = args.ipv6_addresses.get_inner();
        let ipv6_prefix_count_binding = args.ipv6_prefix_count.get_inner();
        let ipv6_prefixes_binding = args.ipv6_prefixes.get_inner();
        let private_ip_binding = args.private_ip.get_inner();
        let private_ip_list_enabled_binding = args.private_ip_list_enabled.get_inner();
        let private_ip_lists_binding = args.private_ip_lists.get_inner();
        let private_ips_binding = args.private_ips.get_inner();
        let private_ips_count_binding = args.private_ips_count.get_inner();
        let security_groups_binding = args.security_groups.get_inner();
        let source_dest_check_binding = args.source_dest_check.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/networkInterface:NetworkInterface".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attachments".into(),
                    value: &attachments_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "interfaceType".into(),
                    value: &interface_type_binding,
                },
                register_interface::ObjectField {
                    name: "ipv4PrefixCount".into(),
                    value: &ipv4_prefix_count_binding,
                },
                register_interface::ObjectField {
                    name: "ipv4Prefixes".into(),
                    value: &ipv4_prefixes_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6AddressCount".into(),
                    value: &ipv6_address_count_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6AddressListEnabled".into(),
                    value: &ipv6_address_list_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6AddressLists".into(),
                    value: &ipv6_address_lists_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6Addresses".into(),
                    value: &ipv6_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6PrefixCount".into(),
                    value: &ipv6_prefix_count_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6Prefixes".into(),
                    value: &ipv6_prefixes_binding,
                },
                register_interface::ObjectField {
                    name: "privateIp".into(),
                    value: &private_ip_binding,
                },
                register_interface::ObjectField {
                    name: "privateIpListEnabled".into(),
                    value: &private_ip_list_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "privateIpLists".into(),
                    value: &private_ip_lists_binding,
                },
                register_interface::ObjectField {
                    name: "privateIps".into(),
                    value: &private_ips_binding,
                },
                register_interface::ObjectField {
                    name: "privateIpsCount".into(),
                    value: &private_ips_count_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroups".into(),
                    value: &security_groups_binding,
                },
                register_interface::ObjectField {
                    name: "sourceDestCheck".into(),
                    value: &source_dest_check_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "attachments".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "interfaceType".into(),
                },
                register_interface::ResultField {
                    name: "ipv4PrefixCount".into(),
                },
                register_interface::ResultField {
                    name: "ipv4Prefixes".into(),
                },
                register_interface::ResultField {
                    name: "ipv6AddressCount".into(),
                },
                register_interface::ResultField {
                    name: "ipv6AddressListEnabled".into(),
                },
                register_interface::ResultField {
                    name: "ipv6AddressLists".into(),
                },
                register_interface::ResultField {
                    name: "ipv6Addresses".into(),
                },
                register_interface::ResultField {
                    name: "ipv6PrefixCount".into(),
                },
                register_interface::ResultField {
                    name: "ipv6Prefixes".into(),
                },
                register_interface::ResultField {
                    name: "macAddress".into(),
                },
                register_interface::ResultField {
                    name: "outpostArn".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsName".into(),
                },
                register_interface::ResultField {
                    name: "privateIp".into(),
                },
                register_interface::ResultField {
                    name: "privateIpListEnabled".into(),
                },
                register_interface::ResultField {
                    name: "privateIpLists".into(),
                },
                register_interface::ResultField {
                    name: "privateIps".into(),
                },
                register_interface::ResultField {
                    name: "privateIpsCount".into(),
                },
                register_interface::ResultField {
                    name: "securityGroups".into(),
                },
                register_interface::ResultField {
                    name: "sourceDestCheck".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
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
        NetworkInterfaceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            attachments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachments").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            interface_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interfaceType").unwrap(),
            ),
            ipv4_prefix_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv4PrefixCount").unwrap(),
            ),
            ipv4_prefixes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv4Prefixes").unwrap(),
            ),
            ipv6_address_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6AddressCount").unwrap(),
            ),
            ipv6_address_list_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6AddressListEnabled").unwrap(),
            ),
            ipv6_address_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6AddressLists").unwrap(),
            ),
            ipv6_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6Addresses").unwrap(),
            ),
            ipv6_prefix_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6PrefixCount").unwrap(),
            ),
            ipv6_prefixes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6Prefixes").unwrap(),
            ),
            mac_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("macAddress").unwrap(),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outpostArn").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            private_dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsName").unwrap(),
            ),
            private_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIp").unwrap(),
            ),
            private_ip_list_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpListEnabled").unwrap(),
            ),
            private_ip_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpLists").unwrap(),
            ),
            private_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIps").unwrap(),
            ),
            private_ips_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpsCount").unwrap(),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroups").unwrap(),
            ),
            source_dest_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDestCheck").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
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
