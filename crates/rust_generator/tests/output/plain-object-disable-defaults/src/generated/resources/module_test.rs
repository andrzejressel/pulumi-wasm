#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod module_test {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct moduleTestArgs {
        #[builder(into, default)]
        pub mod1: pulumi_gestalt_rust::InputOrOutput<Option<super::types::mod1::Typ>>,
        #[builder(into, default)]
        pub val: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Typ>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: moduleTestArgs,
    ) {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let mod1_binding = args.mod1.get_output(context);
        let val_binding = args.val.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "example:index:moduleTest".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mod1".into(),
                    value: mod1_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "val".into(),
                    value: val_binding.get_id(),
                },
            ],
        };
        context.register_resource(request);
    }
}
