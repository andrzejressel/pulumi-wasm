#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_route_table_propagations {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRouteTablePropagationsArgs {
        /// Custom filter block as described below.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetRouteTablePropagationsFilter,
                >,
            >,
        >,
        /// Identifier of EC2 Transit Gateway Route Table.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRouteTablePropagationsResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetRouteTablePropagationsFilter,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of Transit Gateway Route Table Association identifiers.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRouteTablePropagationsArgs,
    ) -> GetRouteTablePropagationsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let transit_gateway_route_table_id_binding = args
            .transit_gateway_route_table_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2transitgateway/getRouteTablePropagations:getRouteTablePropagations"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayRouteTableId".into(),
                    value: transit_gateway_route_table_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRouteTablePropagationsResult {
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            ids: o.get_field("ids"),
            transit_gateway_route_table_id: o.get_field("transitGatewayRouteTableId"),
        }
    }
}
