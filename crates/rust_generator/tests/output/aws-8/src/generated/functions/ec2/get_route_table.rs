#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_route_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRouteTableArgs {
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetRouteTableFilter>>,
        >,
        /// ID of an Internet Gateway or Virtual Private Gateway which is connected to the Route Table (not exported if not passed as a parameter).
        #[builder(into, default)]
        pub gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the specific Route Table to retrieve.
        #[builder(into, default)]
        pub route_table_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of a Subnet which is connected to the Route Table (not exported if not passed as a parameter).
        #[builder(into, default)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match a pair on the desired Route Table.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the VPC that the desired Route Table belongs to.
        #[builder(into, default)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRouteTableResult {
        /// ARN of the route table.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// List of associations with attributes detailed below.
        pub associations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetRouteTableAssociation>,
        >,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetRouteTableFilter>>,
        >,
        /// Gateway ID. Only set when associated with an Internet Gateway or Virtual Private Gateway.
        pub gateway_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ID of the AWS account that owns the route table.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Route Table ID.
        pub route_table_id: pulumi_gestalt_rust::Output<String>,
        /// List of routes with attributes detailed below.
        pub routes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetRouteTableRoute>,
        >,
        /// Subnet ID. Only set when associated with a subnet.
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
        args: GetRouteTableArgs,
    ) -> GetRouteTableResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let gateway_id_binding = args.gateway_id.get_output(context);
        let route_table_id_binding = args.route_table_id.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getRouteTable:getRouteTable".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayId".into(),
                    value: gateway_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeTableId".into(),
                    value: route_table_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: subnet_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: vpc_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRouteTableResult {
            arn: o.get_field("arn"),
            associations: o.get_field("associations"),
            filters: o.get_field("filters"),
            gateway_id: o.get_field("gatewayId"),
            id: o.get_field("id"),
            owner_id: o.get_field("ownerId"),
            route_table_id: o.get_field("routeTableId"),
            routes: o.get_field("routes"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
