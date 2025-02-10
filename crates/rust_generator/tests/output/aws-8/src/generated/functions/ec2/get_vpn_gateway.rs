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
        context: &pulumi_gestalt_rust::Context,
        args: GetVpnGatewayArgs,
    ) -> GetVpnGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let amazon_side_asn_binding = args.amazon_side_asn.get_output(context);
        let attached_vpc_id_binding = args.attached_vpc_id.get_output(context);
        let availability_zone_binding = args.availability_zone.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let state_binding = args.state.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getVpnGateway:getVpnGateway".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "amazonSideAsn".into(),
                    value: amazon_side_asn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attachedVpcId".into(),
                    value: attached_vpc_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: availability_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVpnGatewayResult {
            amazon_side_asn: o.get_field("amazonSideAsn"),
            arn: o.get_field("arn"),
            attached_vpc_id: o.get_field("attachedVpcId"),
            availability_zone: o.get_field("availabilityZone"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
        }
    }
}
