#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpn_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpnGatewayArgs {
        /// Autonomous System Number (ASN) for the Amazon side of the specific VPN Gateway to retrieve.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub amazon_side_asn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of a VPC attached to the specific VPN Gateway to retrieve.
        #[builder(into, default)]
        pub attached_vpc_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Availability Zone of the specific VPN Gateway to retrieve.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetVpnGatewayFilter>>,
        >,
        /// ID of the specific VPN Gateway to retrieve.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// State of the specific VPN Gateway to retrieve.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match
        /// a pair on the desired VPN Gateway.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpnGatewayResult {
        pub amazon_side_asn: pulumi_gestalt_rust::Output<String>,
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub attached_vpc_id: pulumi_gestalt_rust::Output<String>,
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpnGatewayFilter>>,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVpnGatewayArgs,
    ) -> GetVpnGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let amazon_side_asn_binding_1 = args.amazon_side_asn.get_output(context);
        let amazon_side_asn_binding = amazon_side_asn_binding_1.get_inner();
        let attached_vpc_id_binding_1 = args.attached_vpc_id.get_output(context);
        let attached_vpc_id_binding = attached_vpc_id_binding_1.get_inner();
        let availability_zone_binding_1 = args.availability_zone.get_output(context);
        let availability_zone_binding = availability_zone_binding_1.get_inner();
        let filters_binding_1 = args.filters.get_output(context);
        let filters_binding = filters_binding_1.get_inner();
        let id_binding_1 = args.id.get_output(context);
        let id_binding = id_binding_1.get_inner();
        let state_binding_1 = args.state.get_output(context);
        let state_binding = state_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getVpnGateway:getVpnGateway".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "amazonSideAsn".into(),
                    value: &amazon_side_asn_binding,
                },
                register_interface::ObjectField {
                    name: "attachedVpcId".into(),
                    value: &attached_vpc_id_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVpnGatewayResult {
            amazon_side_asn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("amazonSideAsn"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            attached_vpc_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachedVpcId"),
            ),
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
