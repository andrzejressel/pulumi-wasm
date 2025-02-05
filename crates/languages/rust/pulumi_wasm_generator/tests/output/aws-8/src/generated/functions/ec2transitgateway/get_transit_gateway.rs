pub mod get_transit_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTransitGatewayArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetTransitGatewayFilter,
                >,
            >,
        >,
        /// Identifier of the EC2 Transit Gateway.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the EC2 Transit Gateway
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetTransitGatewayResult {
        /// Private Autonomous System Number (ASN) for the Amazon side of a BGP session
        pub amazon_side_asn: pulumi_wasm_rust::Output<i32>,
        /// EC2 Transit Gateway ARN
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Identifier of the default association route table
        pub association_default_route_table_id: pulumi_wasm_rust::Output<String>,
        /// Whether resource attachment requests are automatically accepted
        pub auto_accept_shared_attachments: pulumi_wasm_rust::Output<String>,
        /// Whether resource attachments are automatically associated with the default association route table
        pub default_route_table_association: pulumi_wasm_rust::Output<String>,
        /// Whether resource attachments automatically propagate routes to the default propagation route table
        pub default_route_table_propagation: pulumi_wasm_rust::Output<String>,
        /// Description of the EC2 Transit Gateway
        pub description: pulumi_wasm_rust::Output<String>,
        /// Whether DNS support is enabled
        pub dns_support: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetTransitGatewayFilter,
                >,
            >,
        >,
        /// EC2 Transit Gateway identifier
        pub id: pulumi_wasm_rust::Output<String>,
        /// Whether Multicast support is enabled
        pub multicast_support: pulumi_wasm_rust::Output<String>,
        /// Identifier of the AWS account that owns the EC2 Transit Gateway
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the default propagation route table
        pub propagation_default_route_table_id: pulumi_wasm_rust::Output<String>,
        /// Whether Security Group Referencing Support is enabled
        pub security_group_referencing_support: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The list of associated CIDR blocks
        pub transit_gateway_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether VPN Equal Cost Multipath Protocol support is enabled
        pub vpn_ecmp_support: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTransitGatewayArgs,
    ) -> GetTransitGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2transitgateway/getTransitGateway:getTransitGateway".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTransitGatewayResult {
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
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
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
            transit_gateway_cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("transitGatewayCidrBlocks"),
            ),
            vpn_ecmp_support: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpnEcmpSupport"),
            ),
        }
    }
}
