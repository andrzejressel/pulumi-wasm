/// Provides a Location Service Route Calculator.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = route_calculation::create(
///         "example",
///         RouteCalculationArgs::builder()
///             .calculator_name("example")
///             .data_source("Here")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_location_route_calculator` using the route calculator name. For example:
///
/// ```sh
/// $ pulumi import aws:location/routeCalculation:RouteCalculation example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route_calculation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteCalculationArgs {
        /// The name of the route calculator resource.
        #[builder(into)]
        pub calculator_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the data provider of traffic and road network data.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub data_source: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The optional description for the route calculator resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the route calculator. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RouteCalculationResult {
        /// The Amazon Resource Name (ARN) for the Route calculator resource. Use the ARN when you specify a resource across AWS.
        pub calculator_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the route calculator resource.
        pub calculator_name: pulumi_gestalt_rust::Output<String>,
        /// The timestamp for when the route calculator resource was created in ISO 8601 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Specifies the data provider of traffic and road network data.
        ///
        /// The following arguments are optional:
        pub data_source: pulumi_gestalt_rust::Output<String>,
        /// The optional description for the route calculator resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value tags for the route calculator. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The timestamp for when the route calculator resource was last update in ISO 8601.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteCalculationArgs,
    ) -> RouteCalculationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let calculator_name_binding = args.calculator_name.get_output(context);
        let data_source_binding = args.data_source.get_output(context);
        let description_binding = args.description.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:location/routeCalculation:RouteCalculation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "calculatorName".into(),
                    value: calculator_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSource".into(),
                    value: data_source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouteCalculationResult {
            calculator_arn: o.get_field("calculatorArn"),
            calculator_name: o.get_field("calculatorName"),
            create_time: o.get_field("createTime"),
            data_source: o.get_field("dataSource"),
            description: o.get_field("description"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            update_time: o.get_field("updateTime"),
        }
    }
}
