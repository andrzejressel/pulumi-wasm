#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_transit_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTransitGatewayArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetTransitGatewayFilter,
                >,
            >,
        >,
        /// Identifier of the EC2 Transit Gateway.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the EC2 Transit Gateway
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetTransitGatewayResult {
        /// Private Autonomous System Number (ASN) for the Amazon side of a BGP session
        pub amazon_side_asn: pulumi_gestalt_rust::Output<i32>,
        /// EC2 Transit Gateway ARN
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the default association route table
        pub association_default_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// Whether resource attachment requests are automatically accepted
        pub auto_accept_shared_attachments: pulumi_gestalt_rust::Output<String>,
        /// Whether resource attachments are automatically associated with the default association route table
        pub default_route_table_association: pulumi_gestalt_rust::Output<String>,
        /// Whether resource attachments automatically propagate routes to the default propagation route table
        pub default_route_table_propagation: pulumi_gestalt_rust::Output<String>,
        /// Description of the EC2 Transit Gateway
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Whether DNS support is enabled
        pub dns_support: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetTransitGatewayFilter,
                >,
            >,
        >,
        /// EC2 Transit Gateway identifier
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Whether Multicast support is enabled
        pub multicast_support: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the AWS account that owns the EC2 Transit Gateway
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the default propagation route table
        pub propagation_default_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// Whether Security Group Referencing Support is enabled
        pub security_group_referencing_support: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The list of associated CIDR blocks
        pub transit_gateway_cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether VPN Equal Cost Multipath Protocol support is enabled
        pub vpn_ecmp_support: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTransitGatewayArgs,
    ) -> GetTransitGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2transitgateway/getTransitGateway:getTransitGateway".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTransitGatewayResult {
            amazon_side_asn: o.get_field("amazonSideAsn"),
            arn: o.get_field("arn"),
            association_default_route_table_id: o
                .get_field("associationDefaultRouteTableId"),
            auto_accept_shared_attachments: o.get_field("autoAcceptSharedAttachments"),
            default_route_table_association: o.get_field("defaultRouteTableAssociation"),
            default_route_table_propagation: o.get_field("defaultRouteTablePropagation"),
            description: o.get_field("description"),
            dns_support: o.get_field("dnsSupport"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            multicast_support: o.get_field("multicastSupport"),
            owner_id: o.get_field("ownerId"),
            propagation_default_route_table_id: o
                .get_field("propagationDefaultRouteTableId"),
            security_group_referencing_support: o
                .get_field("securityGroupReferencingSupport"),
            tags: o.get_field("tags"),
            transit_gateway_cidr_blocks: o.get_field("transitGatewayCidrBlocks"),
            vpn_ecmp_support: o.get_field("vpnEcmpSupport"),
        }
    }
}
