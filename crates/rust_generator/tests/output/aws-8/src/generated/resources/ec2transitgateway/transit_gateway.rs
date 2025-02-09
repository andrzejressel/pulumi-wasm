/// Manages an EC2 Transit Gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod transit_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransitGatewayArgs {
        /// Private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is `64512` to `65534` for 16-bit ASNs and `4200000000` to `4294967294` for 32-bit ASNs. Default value: `64512`.
        ///
        /// > **NOTE:** Modifying `amazon_side_asn` on a Transit Gateway with active BGP sessions is [not allowed](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ModifyTransitGatewayOptions.html). You must first delete all Transit Gateway attachments that have BGP configured prior to modifying `amazon_side_asn`.
        #[builder(into, default)]
        pub amazon_side_asn: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Whether resource attachment requests are automatically accepted. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub auto_accept_shared_attachments: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether resource attachments are automatically associated with the default association route table. Valid values: `disable`, `enable`. Default value: `enable`.
        #[builder(into, default)]
        pub default_route_table_association: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether resource attachments automatically propagate routes to the default propagation route table. Valid values: `disable`, `enable`. Default value: `enable`.
        #[builder(into, default)]
        pub default_route_table_propagation: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Description of the EC2 Transit Gateway.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether DNS support is enabled. Valid values: `disable`, `enable`. Default value: `enable`.
        #[builder(into, default)]
        pub dns_support: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether Multicast support is enabled. Required to use `ec2_transit_gateway_multicast_domain`. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub multicast_support: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether Security Group Referencing Support is enabled. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub security_group_referencing_support: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Key-value tags for the EC2 Transit Gateway. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.
        #[builder(into, default)]
        pub transit_gateway_cidr_blocks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Whether VPN Equal Cost Multipath Protocol support is enabled. Valid values: `disable`, `enable`. Default value: `enable`.
        #[builder(into, default)]
        pub vpn_ecmp_support: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TransitGatewayResult {
        /// Private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is `64512` to `65534` for 16-bit ASNs and `4200000000` to `4294967294` for 32-bit ASNs. Default value: `64512`.
        ///
        /// > **NOTE:** Modifying `amazon_side_asn` on a Transit Gateway with active BGP sessions is [not allowed](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ModifyTransitGatewayOptions.html). You must first delete all Transit Gateway attachments that have BGP configured prior to modifying `amazon_side_asn`.
        pub amazon_side_asn: pulumi_gestalt_rust::Output<Option<i32>>,
        /// EC2 Transit Gateway Amazon Resource Name (ARN)
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the default association route table
        pub association_default_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// Whether resource attachment requests are automatically accepted. Valid values: `disable`, `enable`. Default value: `disable`.
        pub auto_accept_shared_attachments: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether resource attachments are automatically associated with the default association route table. Valid values: `disable`, `enable`. Default value: `enable`.
        pub default_route_table_association: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether resource attachments automatically propagate routes to the default propagation route table. Valid values: `disable`, `enable`. Default value: `enable`.
        pub default_route_table_propagation: pulumi_gestalt_rust::Output<Option<String>>,
        /// Description of the EC2 Transit Gateway.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether DNS support is enabled. Valid values: `disable`, `enable`. Default value: `enable`.
        pub dns_support: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether Multicast support is enabled. Required to use `ec2_transit_gateway_multicast_domain`. Valid values: `disable`, `enable`. Default value: `disable`.
        pub multicast_support: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of the AWS account that owns the EC2 Transit Gateway
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the default propagation route table
        pub propagation_default_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// Whether Security Group Referencing Support is enabled. Valid values: `disable`, `enable`. Default value: `disable`.
        pub security_group_referencing_support: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Key-value tags for the EC2 Transit Gateway. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// One or more IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.
        pub transit_gateway_cidr_blocks: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// Whether VPN Equal Cost Multipath Protocol support is enabled. Valid values: `disable`, `enable`. Default value: `enable`.
        pub vpn_ecmp_support: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TransitGatewayArgs,
    ) -> TransitGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let amazon_side_asn_binding = args.amazon_side_asn.get_output(context);
        let auto_accept_shared_attachments_binding = args
            .auto_accept_shared_attachments
            .get_output(context);
        let default_route_table_association_binding = args
            .default_route_table_association
            .get_output(context);
        let default_route_table_propagation_binding = args
            .default_route_table_propagation
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let dns_support_binding = args.dns_support.get_output(context);
        let multicast_support_binding = args.multicast_support.get_output(context);
        let security_group_referencing_support_binding = args
            .security_group_referencing_support
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let transit_gateway_cidr_blocks_binding = args
            .transit_gateway_cidr_blocks
            .get_output(context);
        let vpn_ecmp_support_binding = args.vpn_ecmp_support.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/transitGateway:TransitGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "amazonSideAsn".into(),
                    value: amazon_side_asn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoAcceptSharedAttachments".into(),
                    value: auto_accept_shared_attachments_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultRouteTableAssociation".into(),
                    value: default_route_table_association_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultRouteTablePropagation".into(),
                    value: default_route_table_propagation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsSupport".into(),
                    value: dns_support_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multicastSupport".into(),
                    value: multicast_support_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupReferencingSupport".into(),
                    value: security_group_referencing_support_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayCidrBlocks".into(),
                    value: transit_gateway_cidr_blocks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnEcmpSupport".into(),
                    value: vpn_ecmp_support_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TransitGatewayResult {
            amazon_side_asn: o.get_field("amazonSideAsn"),
            arn: o.get_field("arn"),
            association_default_route_table_id: o
                .get_field("associationDefaultRouteTableId"),
            auto_accept_shared_attachments: o.get_field("autoAcceptSharedAttachments"),
            default_route_table_association: o.get_field("defaultRouteTableAssociation"),
            default_route_table_propagation: o.get_field("defaultRouteTablePropagation"),
            description: o.get_field("description"),
            dns_support: o.get_field("dnsSupport"),
            multicast_support: o.get_field("multicastSupport"),
            owner_id: o.get_field("ownerId"),
            propagation_default_route_table_id: o
                .get_field("propagationDefaultRouteTableId"),
            security_group_referencing_support: o
                .get_field("securityGroupReferencingSupport"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            transit_gateway_cidr_blocks: o.get_field("transitGatewayCidrBlocks"),
            vpn_ecmp_support: o.get_field("vpnEcmpSupport"),
        }
    }
}
