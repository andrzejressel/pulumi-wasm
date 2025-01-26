/// Manages an EC2 Transit Gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = transit_gateway::create(
///         "example",
///         TransitGatewayArgs::builder().description("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway` using the EC2 Transit Gateway identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/transitGateway:TransitGateway example tgw-12345678
/// ```
pub mod transit_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransitGatewayArgs {
        /// Private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is `64512` to `65534` for 16-bit ASNs and `4200000000` to `4294967294` for 32-bit ASNs. Default value: `64512`.
        ///
        /// > **NOTE:** Modifying `amazon_side_asn` on a Transit Gateway with active BGP sessions is [not allowed](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ModifyTransitGatewayOptions.html). You must first delete all Transit Gateway attachments that have BGP configured prior to modifying `amazon_side_asn`.
        #[builder(into, default)]
        pub amazon_side_asn: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Whether resource attachment requests are automatically accepted. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub auto_accept_shared_attachments: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether resource attachments are automatically associated with the default association route table. Valid values: `disable`, `enable`. Default value: `enable`.
        #[builder(into, default)]
        pub default_route_table_association: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether resource attachments automatically propagate routes to the default propagation route table. Valid values: `disable`, `enable`. Default value: `enable`.
        #[builder(into, default)]
        pub default_route_table_propagation: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Description of the EC2 Transit Gateway.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether DNS support is enabled. Valid values: `disable`, `enable`. Default value: `enable`.
        #[builder(into, default)]
        pub dns_support: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether Multicast support is enabled. Required to use `ec2_transit_gateway_multicast_domain`. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub multicast_support: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether Security Group Referencing Support is enabled. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub security_group_referencing_support: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Key-value tags for the EC2 Transit Gateway. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.
        #[builder(into, default)]
        pub transit_gateway_cidr_blocks: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Whether VPN Equal Cost Multipath Protocol support is enabled. Valid values: `disable`, `enable`. Default value: `enable`.
        #[builder(into, default)]
        pub vpn_ecmp_support: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TransitGatewayResult {
        /// Private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is `64512` to `65534` for 16-bit ASNs and `4200000000` to `4294967294` for 32-bit ASNs. Default value: `64512`.
        ///
        /// > **NOTE:** Modifying `amazon_side_asn` on a Transit Gateway with active BGP sessions is [not allowed](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ModifyTransitGatewayOptions.html). You must first delete all Transit Gateway attachments that have BGP configured prior to modifying `amazon_side_asn`.
        pub amazon_side_asn: pulumi_wasm_rust::Output<Option<i32>>,
        /// EC2 Transit Gateway Amazon Resource Name (ARN)
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Identifier of the default association route table
        pub association_default_route_table_id: pulumi_wasm_rust::Output<String>,
        /// Whether resource attachment requests are automatically accepted. Valid values: `disable`, `enable`. Default value: `disable`.
        pub auto_accept_shared_attachments: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether resource attachments are automatically associated with the default association route table. Valid values: `disable`, `enable`. Default value: `enable`.
        pub default_route_table_association: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether resource attachments automatically propagate routes to the default propagation route table. Valid values: `disable`, `enable`. Default value: `enable`.
        pub default_route_table_propagation: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the EC2 Transit Gateway.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether DNS support is enabled. Valid values: `disable`, `enable`. Default value: `enable`.
        pub dns_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether Multicast support is enabled. Required to use `ec2_transit_gateway_multicast_domain`. Valid values: `disable`, `enable`. Default value: `disable`.
        pub multicast_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the AWS account that owns the EC2 Transit Gateway
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the default propagation route table
        pub propagation_default_route_table_id: pulumi_wasm_rust::Output<String>,
        /// Whether Security Group Referencing Support is enabled. Valid values: `disable`, `enable`. Default value: `disable`.
        pub security_group_referencing_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value tags for the EC2 Transit Gateway. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// One or more IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.
        pub transit_gateway_cidr_blocks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether VPN Equal Cost Multipath Protocol support is enabled. Valid values: `disable`, `enable`. Default value: `enable`.
        pub vpn_ecmp_support: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TransitGatewayArgs,
    ) -> TransitGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let amazon_side_asn_binding = args
            .amazon_side_asn
            .get_output(context)
            .get_inner();
        let auto_accept_shared_attachments_binding = args
            .auto_accept_shared_attachments
            .get_output(context)
            .get_inner();
        let default_route_table_association_binding = args
            .default_route_table_association
            .get_output(context)
            .get_inner();
        let default_route_table_propagation_binding = args
            .default_route_table_propagation
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let dns_support_binding = args.dns_support.get_output(context).get_inner();
        let multicast_support_binding = args
            .multicast_support
            .get_output(context)
            .get_inner();
        let security_group_referencing_support_binding = args
            .security_group_referencing_support
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let transit_gateway_cidr_blocks_binding = args
            .transit_gateway_cidr_blocks
            .get_output(context)
            .get_inner();
        let vpn_ecmp_support_binding = args
            .vpn_ecmp_support
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/transitGateway:TransitGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "amazonSideAsn".into(),
                    value: &amazon_side_asn_binding,
                },
                register_interface::ObjectField {
                    name: "autoAcceptSharedAttachments".into(),
                    value: &auto_accept_shared_attachments_binding,
                },
                register_interface::ObjectField {
                    name: "defaultRouteTableAssociation".into(),
                    value: &default_route_table_association_binding,
                },
                register_interface::ObjectField {
                    name: "defaultRouteTablePropagation".into(),
                    value: &default_route_table_propagation_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dnsSupport".into(),
                    value: &dns_support_binding,
                },
                register_interface::ObjectField {
                    name: "multicastSupport".into(),
                    value: &multicast_support_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupReferencingSupport".into(),
                    value: &security_group_referencing_support_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayCidrBlocks".into(),
                    value: &transit_gateway_cidr_blocks_binding,
                },
                register_interface::ObjectField {
                    name: "vpnEcmpSupport".into(),
                    value: &vpn_ecmp_support_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TransitGatewayResult {
            amazon_side_asn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("amazonSideAsn"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            association_default_route_table_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("associationDefaultRouteTableId"),
            ),
            auto_accept_shared_attachments: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoAcceptSharedAttachments"),
            ),
            default_route_table_association: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultRouteTableAssociation"),
            ),
            default_route_table_propagation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultRouteTablePropagation"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dns_support: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsSupport"),
            ),
            multicast_support: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("multicastSupport"),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            propagation_default_route_table_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("propagationDefaultRouteTableId"),
            ),
            security_group_referencing_support: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityGroupReferencingSupport"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            transit_gateway_cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("transitGatewayCidrBlocks"),
            ),
            vpn_ecmp_support: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpnEcmpSupport"),
            ),
        }
    }
}
