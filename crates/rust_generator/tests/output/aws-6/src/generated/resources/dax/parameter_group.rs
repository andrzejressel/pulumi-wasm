/// Provides a DAX Parameter Group resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = parameter_group::create(
///         "example",
///         ParameterGroupArgs::builder()
///             .name("example")
///             .parameters(
///                 vec![
///                     ParameterGroupParameter::builder().name("query-ttl-millis")
///                     .value("100000").build_struct(), ParameterGroupParameter::builder()
///                     .name("record-ttl-millis").value("100000").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DAX Parameter Group using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:dax/parameterGroup:ParameterGroup example my_dax_pg
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod parameter_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ParameterGroupArgs {
        /// A description of the parameter group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the parameter group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parameters of the parameter group.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::dax::ParameterGroupParameter>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ParameterGroupResult {
        /// A description of the parameter group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the parameter group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parameters of the parameter group.
        pub parameters: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dax::ParameterGroupParameter>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ParameterGroupArgs,
    ) -> ParameterGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dax/parameterGroup:ParameterGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ParameterGroupResult {
            description: o.get_field("description"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
        }
    }
}
