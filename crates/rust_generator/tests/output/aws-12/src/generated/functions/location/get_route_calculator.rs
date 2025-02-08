#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRouteCalculatorArgs,
    ) -> GetRouteCalculatorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let calculator_name_binding = args
            .calculator_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:location/getRouteCalculator:getRouteCalculator".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "calculatorName".into(),
                    value: &calculator_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRouteCalculatorResult {
            calculator_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("calculatorArn"),
            ),
            calculator_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("calculatorName"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            data_source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataSource"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
