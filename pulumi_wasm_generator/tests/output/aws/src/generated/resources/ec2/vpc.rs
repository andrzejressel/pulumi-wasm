/// Provides a VPC resource.
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
///     let main = vpc::create(
///         "main",
///         VpcArgs::builder().cidr_block("10.0.0.0/16").build_struct(),
///     );
/// }
/// ```
///
/// Basic usage with tags:
///
/// ```yaml
/// resources:
///   main:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.0.0.0/16
///       instanceTenancy: default
///       tags:
///         Name: main
/// ```
///
/// VPC with CIDR from AWS IPAM:
///
/// ```yaml
/// resources:
///   test:
///     type: aws:ec2:VpcIpam
///     properties:
///       operatingRegions:
///         - regionName: ${current.name}
///   testVpcIpamPool:
///     type: aws:ec2:VpcIpamPool
///     name: test
///     properties:
///       addressFamily: ipv4
///       ipamScopeId: ${test.privateDefaultScopeId}
///       locale: ${current.name}
///   testVpcIpamPoolCidr:
///     type: aws:ec2:VpcIpamPoolCidr
///     name: test
///     properties:
///       ipamPoolId: ${testVpcIpamPool.id}
///       cidr: 172.20.0.0/16
///   testVpc:
///     type: aws:ec2:Vpc
///     name: test
///     properties:
///       ipv4IpamPoolId: ${testVpcIpamPool.id}
///       ipv4NetmaskLength: 28
///     options:
///       dependsOn:
///         - ${testVpcIpamPoolCidr}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPCs using the VPC `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpc:Vpc test_vpc vpc-a01106c2
/// ```
pub mod vpc {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcArgs {
        /// Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IP addresses, or the size of the CIDR block. Default is `false`. Conflicts with `ipv6_ipam_pool_id`
        #[builder(into, default)]
        pub assign_generated_ipv6_cidr_block: pulumi_wasm_rust::Output<Option<bool>>,
        /// The IPv4 CIDR block for the VPC. CIDR can be explicitly set or it can be derived from IPAM using `ipv4_netmask_length`.
        #[builder(into, default)]
        pub cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// A boolean flag to enable/disable DNS hostnames in the VPC. Defaults false.
        #[builder(into, default)]
        pub enable_dns_hostnames: pulumi_wasm_rust::Output<Option<bool>>,
        /// A boolean flag to enable/disable DNS support in the VPC. Defaults to true.
        #[builder(into, default)]
        pub enable_dns_support: pulumi_wasm_rust::Output<Option<bool>>,
        /// Indicates whether Network Address Usage metrics are enabled for your VPC. Defaults to false.
        #[builder(into, default)]
        pub enable_network_address_usage_metrics: pulumi_wasm_rust::Output<Option<bool>>,
        /// A tenancy option for instances launched into the VPC. Default is `default`, which ensures that EC2 instances launched in this VPC use the EC2 instance tenancy attribute specified when the EC2 instance is launched. The only other option is `dedicated`, which ensures that EC2 instances launched in this VPC are run on dedicated tenancy instances regardless of the tenancy attribute specified at launch. This has a dedicated per region fee of $2 per hour, plus an hourly per instance usage fee.
        #[builder(into, default)]
        pub instance_tenancy: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of an IPv4 IPAM pool you want to use for allocating this VPC's CIDR. IPAM is a VPC feature that you can use to automate your IP address management workflows including assigning, tracking, troubleshooting, and auditing IP addresses across AWS Regions and accounts. Using IPAM you can monitor IP address usage throughout your AWS Organization.
        #[builder(into, default)]
        pub ipv4_ipam_pool_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The netmask length of the IPv4 CIDR you want to allocate to this VPC. Requires specifying a `ipv4_ipam_pool_id`.
        #[builder(into, default)]
        pub ipv4_netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// IPv6 CIDR block to request from an IPAM Pool. Can be set explicitly or derived from IPAM using `ipv6_netmask_length`.
        #[builder(into, default)]
        pub ipv6_cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// By default when an IPv6 CIDR is assigned to a VPC a default ipv6_cidr_block_network_border_group will be set to the region of the VPC. This can be changed to restrict advertisement of public addresses to specific Network Border Groups such as LocalZones.
        #[builder(into, default)]
        pub ipv6_cidr_block_network_border_group: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// IPAM Pool ID for a IPv6 pool. Conflicts with `assign_generated_ipv6_cidr_block`.
        #[builder(into, default)]
        pub ipv6_ipam_pool_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Netmask length to request from IPAM Pool. Conflicts with `ipv6_cidr_block`. This can be omitted if IPAM pool as a `allocation_default_netmask_length` set. Valid values are from `44` to `60` in increments of 4.
        #[builder(into, default)]
        pub ipv6_netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcResult {
        /// Amazon Resource Name (ARN) of VPC
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IP addresses, or the size of the CIDR block. Default is `false`. Conflicts with `ipv6_ipam_pool_id`
        pub assign_generated_ipv6_cidr_block: pulumi_wasm_rust::Output<Option<bool>>,
        /// The IPv4 CIDR block for the VPC. CIDR can be explicitly set or it can be derived from IPAM using `ipv4_netmask_length`.
        pub cidr_block: pulumi_wasm_rust::Output<String>,
        /// The ID of the network ACL created by default on VPC creation
        pub default_network_acl_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the route table created by default on VPC creation
        pub default_route_table_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the security group created by default on VPC creation
        pub default_security_group_id: pulumi_wasm_rust::Output<String>,
        /// DHCP options id of the desired VPC.
        pub dhcp_options_id: pulumi_wasm_rust::Output<String>,
        /// A boolean flag to enable/disable DNS hostnames in the VPC. Defaults false.
        pub enable_dns_hostnames: pulumi_wasm_rust::Output<bool>,
        /// A boolean flag to enable/disable DNS support in the VPC. Defaults to true.
        pub enable_dns_support: pulumi_wasm_rust::Output<Option<bool>>,
        /// Indicates whether Network Address Usage metrics are enabled for your VPC. Defaults to false.
        pub enable_network_address_usage_metrics: pulumi_wasm_rust::Output<bool>,
        /// A tenancy option for instances launched into the VPC. Default is `default`, which ensures that EC2 instances launched in this VPC use the EC2 instance tenancy attribute specified when the EC2 instance is launched. The only other option is `dedicated`, which ensures that EC2 instances launched in this VPC are run on dedicated tenancy instances regardless of the tenancy attribute specified at launch. This has a dedicated per region fee of $2 per hour, plus an hourly per instance usage fee.
        pub instance_tenancy: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of an IPv4 IPAM pool you want to use for allocating this VPC's CIDR. IPAM is a VPC feature that you can use to automate your IP address management workflows including assigning, tracking, troubleshooting, and auditing IP addresses across AWS Regions and accounts. Using IPAM you can monitor IP address usage throughout your AWS Organization.
        pub ipv4_ipam_pool_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The netmask length of the IPv4 CIDR you want to allocate to this VPC. Requires specifying a `ipv4_ipam_pool_id`.
        pub ipv4_netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// The association ID for the IPv6 CIDR block.
        pub ipv6_association_id: pulumi_wasm_rust::Output<String>,
        /// IPv6 CIDR block to request from an IPAM Pool. Can be set explicitly or derived from IPAM using `ipv6_netmask_length`.
        pub ipv6_cidr_block: pulumi_wasm_rust::Output<String>,
        /// By default when an IPv6 CIDR is assigned to a VPC a default ipv6_cidr_block_network_border_group will be set to the region of the VPC. This can be changed to restrict advertisement of public addresses to specific Network Border Groups such as LocalZones.
        pub ipv6_cidr_block_network_border_group: pulumi_wasm_rust::Output<String>,
        /// IPAM Pool ID for a IPv6 pool. Conflicts with `assign_generated_ipv6_cidr_block`.
        pub ipv6_ipam_pool_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Netmask length to request from IPAM Pool. Conflicts with `ipv6_cidr_block`. This can be omitted if IPAM pool as a `allocation_default_netmask_length` set. Valid values are from `44` to `60` in increments of 4.
        pub ipv6_netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the main route table associated with
        /// this VPC. Note that you can change a VPC's main route table by using an
        /// `aws.ec2.MainRouteTableAssociation`.
        pub main_route_table_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the AWS account that owns the VPC.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: VpcArgs) -> VpcResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let assign_generated_ipv6_cidr_block_binding = args
            .assign_generated_ipv6_cidr_block
            .get_inner();
        let cidr_block_binding = args.cidr_block.get_inner();
        let enable_dns_hostnames_binding = args.enable_dns_hostnames.get_inner();
        let enable_dns_support_binding = args.enable_dns_support.get_inner();
        let enable_network_address_usage_metrics_binding = args
            .enable_network_address_usage_metrics
            .get_inner();
        let instance_tenancy_binding = args.instance_tenancy.get_inner();
        let ipv4_ipam_pool_id_binding = args.ipv4_ipam_pool_id.get_inner();
        let ipv4_netmask_length_binding = args.ipv4_netmask_length.get_inner();
        let ipv6_cidr_block_binding = args.ipv6_cidr_block.get_inner();
        let ipv6_cidr_block_network_border_group_binding = args
            .ipv6_cidr_block_network_border_group
            .get_inner();
        let ipv6_ipam_pool_id_binding = args.ipv6_ipam_pool_id.get_inner();
        let ipv6_netmask_length_binding = args.ipv6_netmask_length.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpc:Vpc".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assignGeneratedIpv6CidrBlock".into(),
                    value: &assign_generated_ipv6_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "enableDnsHostnames".into(),
                    value: &enable_dns_hostnames_binding,
                },
                register_interface::ObjectField {
                    name: "enableDnsSupport".into(),
                    value: &enable_dns_support_binding,
                },
                register_interface::ObjectField {
                    name: "enableNetworkAddressUsageMetrics".into(),
                    value: &enable_network_address_usage_metrics_binding,
                },
                register_interface::ObjectField {
                    name: "instanceTenancy".into(),
                    value: &instance_tenancy_binding,
                },
                register_interface::ObjectField {
                    name: "ipv4IpamPoolId".into(),
                    value: &ipv4_ipam_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "ipv4NetmaskLength".into(),
                    value: &ipv4_netmask_length_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6CidrBlock".into(),
                    value: &ipv6_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6CidrBlockNetworkBorderGroup".into(),
                    value: &ipv6_cidr_block_network_border_group_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6IpamPoolId".into(),
                    value: &ipv6_ipam_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6NetmaskLength".into(),
                    value: &ipv6_netmask_length_binding,
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
                    name: "assignGeneratedIpv6CidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "cidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "defaultNetworkAclId".into(),
                },
                register_interface::ResultField {
                    name: "defaultRouteTableId".into(),
                },
                register_interface::ResultField {
                    name: "defaultSecurityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "dhcpOptionsId".into(),
                },
                register_interface::ResultField {
                    name: "enableDnsHostnames".into(),
                },
                register_interface::ResultField {
                    name: "enableDnsSupport".into(),
                },
                register_interface::ResultField {
                    name: "enableNetworkAddressUsageMetrics".into(),
                },
                register_interface::ResultField {
                    name: "instanceTenancy".into(),
                },
                register_interface::ResultField {
                    name: "ipv4IpamPoolId".into(),
                },
                register_interface::ResultField {
                    name: "ipv4NetmaskLength".into(),
                },
                register_interface::ResultField {
                    name: "ipv6AssociationId".into(),
                },
                register_interface::ResultField {
                    name: "ipv6CidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "ipv6CidrBlockNetworkBorderGroup".into(),
                },
                register_interface::ResultField {
                    name: "ipv6IpamPoolId".into(),
                },
                register_interface::ResultField {
                    name: "ipv6NetmaskLength".into(),
                },
                register_interface::ResultField {
                    name: "mainRouteTableId".into(),
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
        VpcResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            assign_generated_ipv6_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignGeneratedIpv6CidrBlock").unwrap(),
            ),
            cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlock").unwrap(),
            ),
            default_network_acl_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultNetworkAclId").unwrap(),
            ),
            default_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultRouteTableId").unwrap(),
            ),
            default_security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecurityGroupId").unwrap(),
            ),
            dhcp_options_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dhcpOptionsId").unwrap(),
            ),
            enable_dns_hostnames: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableDnsHostnames").unwrap(),
            ),
            enable_dns_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableDnsSupport").unwrap(),
            ),
            enable_network_address_usage_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableNetworkAddressUsageMetrics").unwrap(),
            ),
            instance_tenancy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceTenancy").unwrap(),
            ),
            ipv4_ipam_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv4IpamPoolId").unwrap(),
            ),
            ipv4_netmask_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv4NetmaskLength").unwrap(),
            ),
            ipv6_association_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6AssociationId").unwrap(),
            ),
            ipv6_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6CidrBlock").unwrap(),
            ),
            ipv6_cidr_block_network_border_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6CidrBlockNetworkBorderGroup").unwrap(),
            ),
            ipv6_ipam_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6IpamPoolId").unwrap(),
            ),
            ipv6_netmask_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6NetmaskLength").unwrap(),
            ),
            main_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mainRouteTableId").unwrap(),
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
