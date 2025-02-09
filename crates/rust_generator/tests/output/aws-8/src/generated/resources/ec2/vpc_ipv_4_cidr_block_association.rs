/// Provides a resource to associate additional IPv4 CIDR blocks with a VPC.
///
/// When a VPC is created, a primary IPv4 CIDR block for the VPC must be specified.
/// The `aws.ec2.VpcIpv4CidrBlockAssociation` resource allows further IPv4 CIDR blocks to be added to the VPC.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = vpc::create(
///         "main",
///         VpcArgs::builder().cidr_block("10.0.0.0/16").build_struct(),
///     );
///     let secondaryCidr = vpc_ipv_4_cidr_block_association::create(
///         "secondaryCidr",
///         VpcIpv4CidrBlockAssociationArgs::builder()
///             .cidr_block("172.20.0.0/16")
///             .vpc_id("${main.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_vpc_ipv4_cidr_block_association` using the VPC CIDR Association ID. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpv4CidrBlockAssociation:VpcIpv4CidrBlockAssociation example vpc-cidr-assoc-xxxxxxxx
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_ipv_4_cidr_block_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpv4CidrBlockAssociationArgs {
        /// The IPv4 CIDR block for the VPC. CIDR can be explicitly set or it can be derived from IPAM using `ipv4_netmask_length`.
        #[builder(into, default)]
        pub cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of an IPv4 IPAM pool you want to use for allocating this VPC's CIDR. IPAM is a VPC feature that you can use to automate your IP address management workflows including assigning, tracking, troubleshooting, and auditing IP addresses across AWS Regions and accounts. Using IPAM you can monitor IP address usage throughout your AWS Organization.
        #[builder(into, default)]
        pub ipv4_ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The netmask length of the IPv4 CIDR you want to allocate to this VPC. Requires specifying a `ipv4_ipam_pool_id`.
        #[builder(into, default)]
        pub ipv4_netmask_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the VPC to make the association with.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcIpv4CidrBlockAssociationResult {
        /// The IPv4 CIDR block for the VPC. CIDR can be explicitly set or it can be derived from IPAM using `ipv4_netmask_length`.
        pub cidr_block: pulumi_gestalt_rust::Output<String>,
        /// The ID of an IPv4 IPAM pool you want to use for allocating this VPC's CIDR. IPAM is a VPC feature that you can use to automate your IP address management workflows including assigning, tracking, troubleshooting, and auditing IP addresses across AWS Regions and accounts. Using IPAM you can monitor IP address usage throughout your AWS Organization.
        pub ipv4_ipam_pool_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The netmask length of the IPv4 CIDR you want to allocate to this VPC. Requires specifying a `ipv4_ipam_pool_id`.
        pub ipv4_netmask_length: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the VPC to make the association with.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VpcIpv4CidrBlockAssociationArgs,
    ) -> VpcIpv4CidrBlockAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cidr_block_binding_1 = args.cidr_block.get_output(context);
        let cidr_block_binding = cidr_block_binding_1.get_inner();
        let ipv4_ipam_pool_id_binding_1 = args.ipv4_ipam_pool_id.get_output(context);
        let ipv4_ipam_pool_id_binding = ipv4_ipam_pool_id_binding_1.get_inner();
        let ipv4_netmask_length_binding_1 = args.ipv4_netmask_length.get_output(context);
        let ipv4_netmask_length_binding = ipv4_netmask_length_binding_1.get_inner();
        let vpc_id_binding_1 = args.vpc_id.get_output(context);
        let vpc_id_binding = vpc_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpv4CidrBlockAssociation:VpcIpv4CidrBlockAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding,
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
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcIpv4CidrBlockAssociationResult {
            cidr_block: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cidrBlock"),
            ),
            ipv4_ipam_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv4IpamPoolId"),
            ),
            ipv4_netmask_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv4NetmaskLength"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
