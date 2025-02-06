/// Provides an Elastic network interface (ENI) resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInterfaceArgs {
        /// Configuration block to define the attachment of the ENI. See Attachment below for more details!
        #[builder(into, default)]
        pub attachments: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::NetworkInterfaceAttachment>>,
        >,
        /// Description for the network interface.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables assigning a primary IPv6 Global Unicast Address (GUA) to the network interface (ENI) in dual-stack or IPv6-only subnets. This ensures the instance attached to the ENI retains a consistent IPv6 address. Once enabled, the first IPv6 GUA becomes the primary IPv6 address and cannot be disabled. The primary IPv6 address remains assigned until the instance is terminated or the ENI is detached. Enabling and subsequent disabling forces recreation of the ENI.
        #[builder(into, default)]
        pub enable_primary_ipv6: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Type of network interface to create. Set to `efa` for Elastic Fabric Adapter. Changing `interface_type` will cause the resource to be destroyed and re-created.
        #[builder(into, default)]
        pub interface_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Number of IPv4 prefixes that AWS automatically assigns to the network interface.
        #[builder(into, default)]
        pub ipv4_prefix_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// One or more IPv4 prefixes assigned to the network interface.
        #[builder(into, default)]
        pub ipv4_prefixes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Number of IPv6 addresses to assign to a network interface. You can't use this option if specifying specific `ipv6_addresses`. If your subnet has the AssignIpv6AddressOnCreation attribute set to `true`, you can specify `0` to override this setting.
        #[builder(into, default)]
        pub ipv6_address_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Whether `ipv6_address_list` is allowed and controls the IPs to assign to the ENI and `ipv6_addresses` and `ipv6_address_count` become read-only. Default is `false`.
        #[builder(into, default)]
        pub ipv6_address_list_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of private IPs to assign to the ENI in sequential order.
        #[builder(into, default)]
        pub ipv6_address_lists: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. Addresses are assigned without regard to order. You can't use this option if you're specifying `ipv6_address_count`.
        #[builder(into, default)]
        pub ipv6_addresses: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Number of IPv6 prefixes that AWS automatically assigns to the network interface.
        #[builder(into, default)]
        pub ipv6_prefix_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// One or more IPv6 prefixes assigned to the network interface.
        #[builder(into, default)]
        pub ipv6_prefixes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        #[builder(into, default)]
        pub private_ip: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether `private_ip_list` is allowed and controls the IPs to assign to the ENI and `private_ips` and `private_ips_count` become read-only. Default is `false`.
        #[builder(into, default)]
        pub private_ip_list_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of private IPs to assign to the ENI in sequential order. Requires setting `private_ip_list_enabled` to `true`.
        #[builder(into, default)]
        pub private_ip_lists: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of private IPs to assign to the ENI without regard to order.
        #[builder(into, default)]
        pub private_ips: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Number of secondary private IPs to assign to the ENI. The total number of private IPs will be 1 + `private_ips_count`, as a primary private IP will be assiged to an ENI by default.
        #[builder(into, default)]
        pub private_ips_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// List of security group IDs to assign to the ENI.
        #[builder(into, default)]
        pub security_groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Whether to enable source destination checking for the ENI. Default true.
        #[builder(into, default)]
        pub source_dest_check: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Subnet ID to create the ENI in.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkInterfaceResult {
        /// ARN of the network interface.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block to define the attachment of the ENI. See Attachment below for more details!
        pub attachments: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::NetworkInterfaceAttachment>,
        >,
        /// Description for the network interface.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Enables assigning a primary IPv6 Global Unicast Address (GUA) to the network interface (ENI) in dual-stack or IPv6-only subnets. This ensures the instance attached to the ENI retains a consistent IPv6 address. Once enabled, the first IPv6 GUA becomes the primary IPv6 address and cannot be disabled. The primary IPv6 address remains assigned until the instance is terminated or the ENI is detached. Enabling and subsequent disabling forces recreation of the ENI.
        pub enable_primary_ipv6: pulumi_gestalt_rust::Output<bool>,
        /// Type of network interface to create. Set to `efa` for Elastic Fabric Adapter. Changing `interface_type` will cause the resource to be destroyed and re-created.
        pub interface_type: pulumi_gestalt_rust::Output<String>,
        /// Number of IPv4 prefixes that AWS automatically assigns to the network interface.
        pub ipv4_prefix_count: pulumi_gestalt_rust::Output<i32>,
        /// One or more IPv4 prefixes assigned to the network interface.
        pub ipv4_prefixes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Number of IPv6 addresses to assign to a network interface. You can't use this option if specifying specific `ipv6_addresses`. If your subnet has the AssignIpv6AddressOnCreation attribute set to `true`, you can specify `0` to override this setting.
        pub ipv6_address_count: pulumi_gestalt_rust::Output<i32>,
        /// Whether `ipv6_address_list` is allowed and controls the IPs to assign to the ENI and `ipv6_addresses` and `ipv6_address_count` become read-only. Default is `false`.
        pub ipv6_address_list_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of private IPs to assign to the ENI in sequential order.
        pub ipv6_address_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet. Addresses are assigned without regard to order. You can't use this option if you're specifying `ipv6_address_count`.
        pub ipv6_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Number of IPv6 prefixes that AWS automatically assigns to the network interface.
        pub ipv6_prefix_count: pulumi_gestalt_rust::Output<i32>,
        /// One or more IPv6 prefixes assigned to the network interface.
        pub ipv6_prefixes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// MAC address of the network interface.
        pub mac_address: pulumi_gestalt_rust::Output<String>,
        pub outpost_arn: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID of the owner of the network interface.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Private DNS name of the network interface (IPv4).
        pub private_dns_name: pulumi_gestalt_rust::Output<String>,
        pub private_ip: pulumi_gestalt_rust::Output<String>,
        /// Whether `private_ip_list` is allowed and controls the IPs to assign to the ENI and `private_ips` and `private_ips_count` become read-only. Default is `false`.
        pub private_ip_list_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of private IPs to assign to the ENI in sequential order. Requires setting `private_ip_list_enabled` to `true`.
        pub private_ip_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// List of private IPs to assign to the ENI without regard to order.
        pub private_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Number of secondary private IPs to assign to the ENI. The total number of private IPs will be 1 + `private_ips_count`, as a primary private IP will be assiged to an ENI by default.
        pub private_ips_count: pulumi_gestalt_rust::Output<i32>,
        /// List of security group IDs to assign to the ENI.
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether to enable source destination checking for the ENI. Default true.
        pub source_dest_check: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Subnet ID to create the ENI in.
        ///
        /// The following arguments are optional:
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NetworkInterfaceArgs,
    ) -> NetworkInterfaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let attachments_binding = args.attachments.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enable_primary_ipv6_binding = args
            .enable_primary_ipv6
            .get_output(context)
            .get_inner();
        let interface_type_binding = args.interface_type.get_output(context).get_inner();
        let ipv4_prefix_count_binding = args
            .ipv4_prefix_count
            .get_output(context)
            .get_inner();
        let ipv4_prefixes_binding = args.ipv4_prefixes.get_output(context).get_inner();
        let ipv6_address_count_binding = args
            .ipv6_address_count
            .get_output(context)
            .get_inner();
        let ipv6_address_list_enabled_binding = args
            .ipv6_address_list_enabled
            .get_output(context)
            .get_inner();
        let ipv6_address_lists_binding = args
            .ipv6_address_lists
            .get_output(context)
            .get_inner();
        let ipv6_addresses_binding = args.ipv6_addresses.get_output(context).get_inner();
        let ipv6_prefix_count_binding = args
            .ipv6_prefix_count
            .get_output(context)
            .get_inner();
        let ipv6_prefixes_binding = args.ipv6_prefixes.get_output(context).get_inner();
        let private_ip_binding = args.private_ip.get_output(context).get_inner();
        let private_ip_list_enabled_binding = args
            .private_ip_list_enabled
            .get_output(context)
            .get_inner();
        let private_ip_lists_binding = args
            .private_ip_lists
            .get_output(context)
            .get_inner();
        let private_ips_binding = args.private_ips.get_output(context).get_inner();
        let private_ips_count_binding = args
            .private_ips_count
            .get_output(context)
            .get_inner();
        let security_groups_binding = args
            .security_groups
            .get_output(context)
            .get_inner();
        let source_dest_check_binding = args
            .source_dest_check
            .get_output(context)
            .get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/networkInterface:NetworkInterface".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "enablePrimaryIpv6".into(),
                    value: &enable_primary_ipv6_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkInterfaceResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            attachments: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachments"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enable_primary_ipv6: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enablePrimaryIpv6"),
            ),
            interface_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("interfaceType"),
            ),
            ipv4_prefix_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv4PrefixCount"),
            ),
            ipv4_prefixes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv4Prefixes"),
            ),
            ipv6_address_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6AddressCount"),
            ),
            ipv6_address_list_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6AddressListEnabled"),
            ),
            ipv6_address_lists: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6AddressLists"),
            ),
            ipv6_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6Addresses"),
            ),
            ipv6_prefix_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6PrefixCount"),
            ),
            ipv6_prefixes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6Prefixes"),
            ),
            mac_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("macAddress"),
            ),
            outpost_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outpostArn"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            private_dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateDnsName"),
            ),
            private_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIp"),
            ),
            private_ip_list_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpListEnabled"),
            ),
            private_ip_lists: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpLists"),
            ),
            private_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIps"),
            ),
            private_ips_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpsCount"),
            ),
            security_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroups"),
            ),
            source_dest_check: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceDestCheck"),
            ),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
