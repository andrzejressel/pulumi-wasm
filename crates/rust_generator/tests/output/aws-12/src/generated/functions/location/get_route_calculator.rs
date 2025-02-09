#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_route_calculator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRouteCalculatorArgs {
        /// Name of the route calculator resource.
        #[builder(into)]
        pub calculator_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the route calculator.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRouteCalculatorResult {
        /// ARN for the Route calculator resource. Use the ARN when you specify a resource across AWS.
        pub calculator_arn: pulumi_gestalt_rust::Output<String>,
        pub calculator_name: pulumi_gestalt_rust::Output<String>,
        /// Timestamp for when the route calculator resource was created in ISO 8601 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Data provider of traffic and road network data.
        pub data_source: pulumi_gestalt_rust::Output<String>,
        /// Optional description of the route calculator resource.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the route calculator.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Timestamp for when the route calculator resource was last updated in ISO 8601 format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRouteCalculatorArgs,
    ) -> GetRouteCalculatorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let calculator_name_binding = args.calculator_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:location/getRouteCalculator:getRouteCalculator".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "calculatorName".into(),
                    value: calculator_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRouteCalculatorResult {
            calculator_arn: o.get_field("calculatorArn"),
            calculator_name: o.get_field("calculatorName"),
            create_time: o.get_field("createTime"),
            data_source: o.get_field("dataSource"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            tags: o.get_field("tags"),
            update_time: o.get_field("updateTime"),
        }
    }
}
