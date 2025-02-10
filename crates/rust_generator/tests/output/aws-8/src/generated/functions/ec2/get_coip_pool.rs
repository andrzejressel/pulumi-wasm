#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_coip_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCoipPoolArgs {
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetCoipPoolFilter>>,
        >,
        /// Local Gateway Route Table Id assigned to desired COIP Pool
        #[builder(into, default)]
        pub local_gateway_route_table_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// ID of the specific COIP Pool to retrieve.
        #[builder(into, default)]
        pub pool_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match
        /// a pair on the desired COIP Pool.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetCoipPoolResult {
        /// ARN of the COIP pool
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetCoipPoolFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub local_gateway_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// Set of CIDR blocks in pool
        pub pool_cidrs: pulumi_gestalt_rust::Output<Vec<String>>,
        pub pool_id: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCoipPoolArgs,
    ) -> GetCoipPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let local_gateway_route_table_id_binding = args
            .local_gateway_route_table_id
            .get_output(context);
        let pool_id_binding = args.pool_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getCoipPool:getCoipPool".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localGatewayRouteTableId".into(),
                    value: local_gateway_route_table_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "poolId".into(),
                    value: pool_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCoipPoolResult {
            arn: o.get_field("arn"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            local_gateway_route_table_id: o.get_field("localGatewayRouteTableId"),
            pool_cidrs: o.get_field("poolCidrs"),
            pool_id: o.get_field("poolId"),
            tags: o.get_field("tags"),
        }
    }
}
