#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        #[builder(into, default)]
        pub baz: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        pub baz: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceArgs,
    ) -> ResourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let baz_binding = args.baz.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "foo-bar:deeply/nested/module:Resource".into(),
            name: name.to_string(),
            version: super::super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baz".into(),
                    value: baz_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceResult {
            baz: o.get_field("baz"),
        }
    }
}
