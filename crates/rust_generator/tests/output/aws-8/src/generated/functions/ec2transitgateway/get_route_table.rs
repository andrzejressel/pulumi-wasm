#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_route_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRouteTableArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::ec2transitgateway::GetRouteTableFilter>,
            >,
        >,
        /// Identifier of the EC2 Transit Gateway Route Table.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the EC2 Transit Gateway Route Table
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRouteTableResult {
        /// EC2 Transit Gateway Route Table ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Boolean whether this is the default association route table for the EC2 Transit Gateway
        pub default_association_route_table: pulumi_gestalt_rust::Output<bool>,
        /// Boolean whether this is the default propagation route table for the EC2 Transit Gateway
        pub default_propagation_route_table: pulumi_gestalt_rust::Output<bool>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::ec2transitgateway::GetRouteTableFilter>,
            >,
        >,
        /// EC2 Transit Gateway Route Table identifier
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Route Table
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// EC2 Transit Gateway identifier
        pub transit_gateway_id: pulumi_gestalt_rust::Output<String>,
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
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2transitgateway/getRouteTable:getRouteTable".into(),
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
        GetRouteTableResult {
            arn: o.get_field("arn"),
            default_association_route_table: o.get_field("defaultAssociationRouteTable"),
            default_propagation_route_table: o.get_field("defaultPropagationRouteTable"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            tags: o.get_field("tags"),
            transit_gateway_id: o.get_field("transitGatewayId"),
        }
    }
}
