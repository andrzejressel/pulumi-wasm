#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_nat_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNatGatewayArgs {
        /// Custom filter block as described below.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetNatGatewayFilter>>,
        >,
        /// ID of the specific NAT Gateway to retrieve.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// State of the NAT Gateway (pending | failed | available | deleting | deleted ).
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of subnet that the NAT Gateway resides in.
        #[builder(into, default)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match
        /// a pair on the desired NAT Gateway.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the VPC that the NAT Gateway resides in.
        #[builder(into, default)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNatGatewayResult {
        /// ID of the EIP allocated to the selected NAT Gateway.
        pub allocation_id: pulumi_gestalt_rust::Output<String>,
        /// The association ID of the Elastic IP address that's associated with the NAT Gateway. Only available when `connectivity_type` is `public`.
        pub association_id: pulumi_gestalt_rust::Output<String>,
        /// Connectivity type of the NAT Gateway.
        pub connectivity_type: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetNatGatewayFilter>>,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the ENI allocated to the selected NAT Gateway.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// Private IP address of the selected NAT Gateway.
        pub private_ip: pulumi_gestalt_rust::Output<String>,
        /// Public IP (EIP) address of the selected NAT Gateway.
        pub public_ip: pulumi_gestalt_rust::Output<String>,
        /// Secondary allocation EIP IDs for the selected NAT Gateway.
        pub secondary_allocation_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The number of secondary private IPv4 addresses assigned to the selected NAT Gateway.
        pub secondary_private_ip_address_count: pulumi_gestalt_rust::Output<i32>,
        /// Secondary private IPv4 addresses assigned to the selected NAT Gateway.
        pub secondary_private_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNatGatewayArgs,
    ) -> GetNatGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let state_binding = args.state.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getNatGateway:getNatGateway".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: &state_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNatGatewayResult {
            allocation_id: o.get_field("allocationId"),
            association_id: o.get_field("associationId"),
            connectivity_type: o.get_field("connectivityType"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            network_interface_id: o.get_field("networkInterfaceId"),
            private_ip: o.get_field("privateIp"),
            public_ip: o.get_field("publicIp"),
            secondary_allocation_ids: o.get_field("secondaryAllocationIds"),
            secondary_private_ip_address_count: o
                .get_field("secondaryPrivateIpAddressCount"),
            secondary_private_ip_addresses: o.get_field("secondaryPrivateIpAddresses"),
            state: o.get_field("state"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
