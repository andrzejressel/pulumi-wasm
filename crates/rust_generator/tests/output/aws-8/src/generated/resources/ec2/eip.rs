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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod eip {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EipArgs {
        /// IP address from an EC2 BYOIP pool. This option is only available for VPC EIPs.
        #[builder(into, default)]
        pub address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-specified primary or secondary private IP address to associate with the Elastic IP address. If no private IP address is specified, the Elastic IP address is associated with the primary private IP address.
        #[builder(into, default)]
        pub associate_with_private_ip: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// ID  of a customer-owned address pool. For more on customer owned IP addressed check out [Customer-owned IP addresses guide](https://docs.aws.amazon.com/outposts/latest/userguide/outposts-networking-components.html#ip-addressing).
        #[builder(into, default)]
        pub customer_owned_ipv4_pool: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates if this EIP is for use in VPC (`vpc`).
        #[builder(into, default)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// EC2 instance ID.
        #[builder(into, default)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of an IPAM pool which has an Amazon-provided or BYOIP public IPv4 CIDR provisioned to it.
        #[builder(into, default)]
        pub ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Location from which the IP address is advertised. Use this parameter to limit the address to this location.
        #[builder(into, default)]
        pub network_border_group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network interface ID to associate with.
        #[builder(into, default)]
        pub network_interface: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// EC2 IPv4 address pool identifier or `amazon`.
        /// This option is only available for VPC EIPs.
        #[builder(into, default)]
        pub public_ipv4_pool: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. Tags can only be applied to EIPs in a VPC. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
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
        pub vpc: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EipResult {
        /// IP address from an EC2 BYOIP pool. This option is only available for VPC EIPs.
        pub address: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID that AWS assigns to represent the allocation of the Elastic IP address for use with instances in a VPC.
        pub allocation_id: pulumi_gestalt_rust::Output<String>,
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// User-specified primary or secondary private IP address to associate with the Elastic IP address. If no private IP address is specified, the Elastic IP address is associated with the primary private IP address.
        pub associate_with_private_ip: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID representing the association of the address with an instance in a VPC.
        pub association_id: pulumi_gestalt_rust::Output<String>,
        /// Carrier IP address.
        pub carrier_ip: pulumi_gestalt_rust::Output<String>,
        /// Customer owned IP.
        pub customer_owned_ip: pulumi_gestalt_rust::Output<String>,
        /// ID  of a customer-owned address pool. For more on customer owned IP addressed check out [Customer-owned IP addresses guide](https://docs.aws.amazon.com/outposts/latest/userguide/outposts-networking-components.html#ip-addressing).
        pub customer_owned_ipv4_pool: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates if this EIP is for use in VPC (`vpc`).
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// EC2 instance ID.
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// The ID of an IPAM pool which has an Amazon-provided or BYOIP public IPv4 CIDR provisioned to it.
        pub ipam_pool_id: pulumi_gestalt_rust::Output<String>,
        /// Location from which the IP address is advertised. Use this parameter to limit the address to this location.
        pub network_border_group: pulumi_gestalt_rust::Output<String>,
        /// Network interface ID to associate with.
        pub network_interface: pulumi_gestalt_rust::Output<String>,
        /// The Private DNS associated with the Elastic IP address (if in VPC).
        pub private_dns: pulumi_gestalt_rust::Output<String>,
        /// Contains the private IP address (if in VPC).
        pub private_ip: pulumi_gestalt_rust::Output<String>,
        /// The DNS pointer (PTR) record for the IP address.
        pub ptr_record: pulumi_gestalt_rust::Output<String>,
        /// Public DNS associated with the Elastic IP address.
        pub public_dns: pulumi_gestalt_rust::Output<String>,
        /// Contains the public IP address.
        pub public_ip: pulumi_gestalt_rust::Output<String>,
        /// EC2 IPv4 address pool identifier or `amazon`.
        /// This option is only available for VPC EIPs.
        pub public_ipv4_pool: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. Tags can only be applied to EIPs in a VPC. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Boolean if the EIP is in a VPC or not. Use `domain` instead.
        /// Defaults to `true` unless the region supports EC2-Classic.
        ///
        /// > **NOTE:** You can specify either the `instance` ID or the `network_interface` ID, but not both. Including both will **not** return an error from the AWS API, but will have undefined behavior. See the relevant [AssociateAddress API Call][1] for more information.
        ///
        /// > **NOTE:** Specifying both `public_ipv4_pool` and `address` won't cause an error but `address` will be used in the
        /// case both options are defined as the api only requires one or the other.
        pub vpc: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EipArgs,
    ) -> EipResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let address_binding = args.address.get_output(context);
        let associate_with_private_ip_binding = args
            .associate_with_private_ip
            .get_output(context);
        let customer_owned_ipv4_pool_binding = args
            .customer_owned_ipv4_pool
            .get_output(context);
        let domain_binding = args.domain.get_output(context);
        let instance_binding = args.instance.get_output(context);
        let ipam_pool_id_binding = args.ipam_pool_id.get_output(context);
        let network_border_group_binding = args.network_border_group.get_output(context);
        let network_interface_binding = args.network_interface.get_output(context);
        let public_ipv4_pool_binding = args.public_ipv4_pool.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_binding = args.vpc.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/eip:Eip".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "address".into(),
                    value: address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "associateWithPrivateIp".into(),
                    value: associate_with_private_ip_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerOwnedIpv4Pool".into(),
                    value: customer_owned_ipv4_pool_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: instance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipamPoolId".into(),
                    value: ipam_pool_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkBorderGroup".into(),
                    value: network_border_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterface".into(),
                    value: network_interface_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicIpv4Pool".into(),
                    value: public_ipv4_pool_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpc".into(),
                    value: vpc_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EipResult {
            address: o.get_field("address"),
            allocation_id: o.get_field("allocationId"),
            arn: o.get_field("arn"),
            associate_with_private_ip: o.get_field("associateWithPrivateIp"),
            association_id: o.get_field("associationId"),
            carrier_ip: o.get_field("carrierIp"),
            customer_owned_ip: o.get_field("customerOwnedIp"),
            customer_owned_ipv4_pool: o.get_field("customerOwnedIpv4Pool"),
            domain: o.get_field("domain"),
            instance: o.get_field("instance"),
            ipam_pool_id: o.get_field("ipamPoolId"),
            network_border_group: o.get_field("networkBorderGroup"),
            network_interface: o.get_field("networkInterface"),
            private_dns: o.get_field("privateDns"),
            private_ip: o.get_field("privateIp"),
            ptr_record: o.get_field("ptrRecord"),
            public_dns: o.get_field("publicDns"),
            public_ip: o.get_field("publicIp"),
            public_ipv4_pool: o.get_field("publicIpv4Pool"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc: o.get_field("vpc"),
        }
    }
}
