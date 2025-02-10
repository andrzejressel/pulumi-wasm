/// Resource for managing an AWS EC2 (Elastic Compute Cloud) Transit Gateway Default Route Table Propagation.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = default_route_table_propagation::create(
///         "example",
///         DefaultRouteTablePropagationArgs::builder()
///             .transit_gateway_id("${exampleAwsEc2TransitGateway.id}")
///             .transit_gateway_route_table_id(
///                 "${exampleAwsEc2TransitGatewayRouteTable.id}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod default_route_table_propagation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultRouteTablePropagationArgs {
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::ec2transitgateway::DefaultRouteTablePropagationTimeouts,
            >,
        >,
        /// ID of the Transit Gateway to change the default association route table on.
        #[builder(into)]
        pub transit_gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the Transit Gateway Route Table to be made the default association route table.
        #[builder(into)]
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DefaultRouteTablePropagationResult {
        pub original_default_route_table_id: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::ec2transitgateway::DefaultRouteTablePropagationTimeouts,
            >,
        >,
        /// ID of the Transit Gateway to change the default association route table on.
        pub transit_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the Transit Gateway Route Table to be made the default association route table.
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultRouteTablePropagationArgs,
    ) -> DefaultRouteTablePropagationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let timeouts_binding = args.timeouts.get_output(context);
        let transit_gateway_id_binding = args.transit_gateway_id.get_output(context);
        let transit_gateway_route_table_id_binding = args
            .transit_gateway_route_table_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/defaultRouteTablePropagation:DefaultRouteTablePropagation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayId".into(),
                    value: transit_gateway_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayRouteTableId".into(),
                    value: transit_gateway_route_table_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefaultRouteTablePropagationResult {
            original_default_route_table_id: o.get_field("originalDefaultRouteTableId"),
            timeouts: o.get_field("timeouts"),
            transit_gateway_id: o.get_field("transitGatewayId"),
            transit_gateway_route_table_id: o.get_field("transitGatewayRouteTableId"),
        }
    }
}
