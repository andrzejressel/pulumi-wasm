pub mod example_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExampleServerArgs {
        #[builder(into, default)]
        pub map_array_enum: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    std::collections::HashMap<
                        String,
                        super::types::AnnotationStoreSchemaValueType,
                    >,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ExampleServerResult {
        pub map_array_enum: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    std::collections::HashMap<
                        String,
                        super::types::AnnotationStoreSchemaValueType,
                    >,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ExampleServerArgs,
    ) -> ExampleServerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let map_array_enum_binding = args.map_array_enum.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "example:index:ExampleServer".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mapArrayEnum".into(),
                    value: &map_array_enum_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ExampleServerResult {
            map_array_enum: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mapArrayEnum"),
            ),
        }
    }
}
