/// Provides an Elastic IP resource.
///
/// > **Note:** EIP may require IGW to exist prior to association. Use `depends_on` to set an explicit dependency on the IGW.
///
/// > **Note:** Do not use `network_interface` to associate the EIP to `aws.lb.LoadBalancer` or `aws.ec2.NatGateway` resources. Instead use the `allocation_id` available in those resources to allow AWS to manage the association, otherwise you will see `AuthFailure` errors.
///
/// ## Example Usage
///
/// ### Single EIP associated with an instance
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let lb = eip::create(
///         "lb",
///         EipArgs::builder().domain("vpc").instance("${web.id}").build_struct(),
///     );
/// }
/// ```
///
/// ### Multiple EIPs associated with a single network interface
///
/// ```yaml
/// resources:
///   multi-ip:
///     type: aws:ec2:NetworkInterface
///     properties:
///       subnetId: ${main.id}
///       privateIps:
///         - 10.0.0.10
///         - 10.0.0.11
///   one:
///     type: aws:ec2:Eip
///     properties:
///       domain: vpc
///       networkInterface: ${["multi-ip"].id}
///       associateWithPrivateIp: 10.0.0.10
///   two:
///     type: aws:ec2:Eip
///     properties:
///       domain: vpc
///       networkInterface: ${["multi-ip"].id}
///       associateWithPrivateIp: 10.0.0.11
/// ```
///
/// ### Attaching an EIP to an Instance with a pre-assigned private ip (VPC Only)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bar = eip::create(
///         "bar",
///         EipArgs::builder()
///             .associate_with_private_ip("10.0.0.12")
///             .domain("vpc")
///             .instance("${foo.id}")
///             .build_struct(),
///     );
///     let default = vpc::create(
///         "default",
///         VpcArgs::builder()
///             .cidr_block("10.0.0.0/16")
///             .enable_dns_hostnames(true)
///             .build_struct(),
///     );
///     let foo = instance::create(
///         "foo",
///         InstanceArgs::builder()
///             .ami("ami-5189a661")
///             .instance_type("t2.micro")
///             .private_ip("10.0.0.12")
///             .subnet_id("${myTestSubnet.id}")
///             .build_struct(),
///     );
///     let gw = internet_gateway::create(
///         "gw",
///         InternetGatewayArgs::builder().vpc_id("${default.id}").build_struct(),
///     );
///     let myTestSubnet = subnet::create(
///         "myTestSubnet",
///         SubnetArgs::builder()
///             .cidr_block("10.0.0.0/24")
///             .map_public_ip_on_launch(true)
///             .vpc_id("${default.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Allocating EIP from the BYOIP pool
///
/// ```yaml
/// resources:
///   byoip-ip:
///     type: aws:ec2:Eip
///     properties:
///       domain: vpc
///       publicIpv4Pool: ipv4pool-ec2-012345
/// ```
///
/// ### Allocating EIP from the IPAM Pool
///
/// ```yaml
/// resources:
///   ipam-ip:
///     type: aws:ec2:Eip
///     properties:
///       domain: vpc
///       ipamPoolId: ipam-pool-07ccc86aa41bef7ce
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EIPs in a VPC using their Allocation ID. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/eip:Eip bar eipalloc-00a10e96
/// ```
pub mod eip {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EipArgs {
        /// IP address from an EC2 BYOIP pool. This option is only available for VPC EIPs.
        #[builder(into, default)]
        pub address: pulumi_wasm_rust::Output<Option<String>>,
        /// User-specified primary or secondary private IP address to associate with the Elastic IP address. If no private IP address is specified, the Elastic IP address is associated with the primary private IP address.
        #[builder(into, default)]
        pub associate_with_private_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// ID  of a customer-owned address pool. For more on customer owned IP addressed check out [Customer-owned IP addresses guide](https://docs.aws.amazon.com/outposts/latest/userguide/outposts-networking-components.html#ip-addressing).
        #[builder(into, default)]
        pub customer_owned_ipv4_pool: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates if this EIP is for use in VPC (`vpc`).
        #[builder(into, default)]
        pub domain: pulumi_wasm_rust::Output<Option<String>>,
        /// EC2 instance ID.
        #[builder(into, default)]
        pub instance: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of an IPAM pool which has an Amazon-provided or BYOIP public IPv4 CIDR provisioned to it.
        #[builder(into, default)]
        pub ipam_pool_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Location from which the IP address is advertised. Use this parameter to limit the address to this location.
        #[builder(into, default)]
        pub network_border_group: pulumi_wasm_rust::Output<Option<String>>,
        /// Network interface ID to associate with.
        #[builder(into, default)]
        pub network_interface: pulumi_wasm_rust::Output<Option<String>>,
        /// EC2 IPv4 address pool identifier or `amazon`.
        /// This option is only available for VPC EIPs.
        #[builder(into, default)]
        pub public_ipv4_pool: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. Tags can only be applied to EIPs in a VPC. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Boolean if the EIP is in a VPC or not. Use `domain` instead.
        /// Defaults to `true` unless the region supports EC2-Classic.
        ///
        /// > **NOTE:** You can specify either the `instance` ID or the `network_interface` ID, but not both. Including both will **not** return an error from the AWS API, but will have undefined behavior. See the relevant [AssociateAddress API Call][1] for more information.
        ///
        /// > **NOTE:** Specifying both `public_ipv4_pool` and `address` won't cause an error but `address` will be used in the
        /// case both options are defined as the api only requires one or the other.
        #[builder(into, default)]
        pub vpc: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EipResult {
        /// IP address from an EC2 BYOIP pool. This option is only available for VPC EIPs.
        pub address: pulumi_wasm_rust::Output<Option<String>>,
        /// ID that AWS assigns to represent the allocation of the Elastic IP address for use with instances in a VPC.
        pub allocation_id: pulumi_wasm_rust::Output<String>,
        pub arn: pulumi_wasm_rust::Output<String>,
        /// User-specified primary or secondary private IP address to associate with the Elastic IP address. If no private IP address is specified, the Elastic IP address is associated with the primary private IP address.
        pub associate_with_private_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// ID representing the association of the address with an instance in a VPC.
        pub association_id: pulumi_wasm_rust::Output<String>,
        /// Carrier IP address.
        pub carrier_ip: pulumi_wasm_rust::Output<String>,
        /// Customer owned IP.
        pub customer_owned_ip: pulumi_wasm_rust::Output<String>,
        /// ID  of a customer-owned address pool. For more on customer owned IP addressed check out [Customer-owned IP addresses guide](https://docs.aws.amazon.com/outposts/latest/userguide/outposts-networking-components.html#ip-addressing).
        pub customer_owned_ipv4_pool: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates if this EIP is for use in VPC (`vpc`).
        pub domain: pulumi_wasm_rust::Output<String>,
        /// EC2 instance ID.
        pub instance: pulumi_wasm_rust::Output<String>,
        /// The ID of an IPAM pool which has an Amazon-provided or BYOIP public IPv4 CIDR provisioned to it.
        pub ipam_pool_id: pulumi_wasm_rust::Output<String>,
        /// Location from which the IP address is advertised. Use this parameter to limit the address to this location.
        pub network_border_group: pulumi_wasm_rust::Output<String>,
        /// Network interface ID to associate with.
        pub network_interface: pulumi_wasm_rust::Output<String>,
        /// The Private DNS associated with the Elastic IP address (if in VPC).
        pub private_dns: pulumi_wasm_rust::Output<String>,
        /// Contains the private IP address (if in VPC).
        pub private_ip: pulumi_wasm_rust::Output<String>,
        /// The DNS pointer (PTR) record for the IP address.
        pub ptr_record: pulumi_wasm_rust::Output<String>,
        /// Public DNS associated with the Elastic IP address.
        pub public_dns: pulumi_wasm_rust::Output<String>,
        /// Contains the public IP address.
        pub public_ip: pulumi_wasm_rust::Output<String>,
        /// EC2 IPv4 address pool identifier or `amazon`.
        /// This option is only available for VPC EIPs.
        pub public_ipv4_pool: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. Tags can only be applied to EIPs in a VPC. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Boolean if the EIP is in a VPC or not. Use `domain` instead.
        /// Defaults to `true` unless the region supports EC2-Classic.
        ///
        /// > **NOTE:** You can specify either the `instance` ID or the `network_interface` ID, but not both. Including both will **not** return an error from the AWS API, but will have undefined behavior. See the relevant [AssociateAddress API Call][1] for more information.
        ///
        /// > **NOTE:** Specifying both `public_ipv4_pool` and `address` won't cause an error but `address` will be used in the
        /// case both options are defined as the api only requires one or the other.
        pub vpc: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EipArgs) -> EipResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let address_binding = args.address.get_inner();
        let associate_with_private_ip_binding = args
            .associate_with_private_ip
            .get_inner();
        let customer_owned_ipv4_pool_binding = args.customer_owned_ipv4_pool.get_inner();
        let domain_binding = args.domain.get_inner();
        let instance_binding = args.instance.get_inner();
        let ipam_pool_id_binding = args.ipam_pool_id.get_inner();
        let network_border_group_binding = args.network_border_group.get_inner();
        let network_interface_binding = args.network_interface.get_inner();
        let public_ipv4_pool_binding = args.public_ipv4_pool.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_binding = args.vpc.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/eip:Eip".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "address".into(),
                    value: &address_binding,
                },
                register_interface::ObjectField {
                    name: "associateWithPrivateIp".into(),
                    value: &associate_with_private_ip_binding,
                },
                register_interface::ObjectField {
                    name: "customerOwnedIpv4Pool".into(),
                    value: &customer_owned_ipv4_pool_binding,
                },
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "ipamPoolId".into(),
                    value: &ipam_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "networkBorderGroup".into(),
                    value: &network_border_group_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterface".into(),
                    value: &network_interface_binding,
                },
                register_interface::ObjectField {
                    name: "publicIpv4Pool".into(),
                    value: &public_ipv4_pool_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpc".into(),
                    value: &vpc_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "address".into(),
                },
                register_interface::ResultField {
                    name: "allocationId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "associateWithPrivateIp".into(),
                },
                register_interface::ResultField {
                    name: "associationId".into(),
                },
                register_interface::ResultField {
                    name: "carrierIp".into(),
                },
                register_interface::ResultField {
                    name: "customerOwnedIp".into(),
                },
                register_interface::ResultField {
                    name: "customerOwnedIpv4Pool".into(),
                },
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "instance".into(),
                },
                register_interface::ResultField {
                    name: "ipamPoolId".into(),
                },
                register_interface::ResultField {
                    name: "networkBorderGroup".into(),
                },
                register_interface::ResultField {
                    name: "networkInterface".into(),
                },
                register_interface::ResultField {
                    name: "privateDns".into(),
                },
                register_interface::ResultField {
                    name: "privateIp".into(),
                },
                register_interface::ResultField {
                    name: "ptrRecord".into(),
                },
                register_interface::ResultField {
                    name: "publicDns".into(),
                },
                register_interface::ResultField {
                    name: "publicIp".into(),
                },
                register_interface::ResultField {
                    name: "publicIpv4Pool".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpc".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EipResult {
            address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("address").unwrap(),
            ),
            allocation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            associate_with_private_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associateWithPrivateIp").unwrap(),
            ),
            association_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associationId").unwrap(),
            ),
            carrier_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("carrierIp").unwrap(),
            ),
            customer_owned_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerOwnedIp").unwrap(),
            ),
            customer_owned_ipv4_pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerOwnedIpv4Pool").unwrap(),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instance").unwrap(),
            ),
            ipam_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamPoolId").unwrap(),
            ),
            network_border_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkBorderGroup").unwrap(),
            ),
            network_interface: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterface").unwrap(),
            ),
            private_dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDns").unwrap(),
            ),
            private_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIp").unwrap(),
            ),
            ptr_record: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ptrRecord").unwrap(),
            ),
            public_dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicDns").unwrap(),
            ),
            public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIp").unwrap(),
            ),
            public_ipv4_pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpv4Pool").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc: pulumi_wasm_rust::__private::into_domain(hashmap.remove("vpc").unwrap()),
        }
    }
}
