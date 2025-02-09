#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod example_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExampleServerArgs {
        #[builder(into, default)]
        pub properties_collection: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    pulumi_gestalt_rust::OneOf2<
                        super::types::ServerPropertiesForReplica,
                        super::types::ServerPropertiesForRestore,
                    >,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ExampleServerResult {
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExampleServerArgs,
    ) -> ExampleServerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let properties_collection_binding = args
            .properties_collection
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "example:index:ExampleServer".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "propertiesCollection".into(),
                    value: properties_collection_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExampleServerResult {
            name: o.get_field("name"),
        }
    }
}
