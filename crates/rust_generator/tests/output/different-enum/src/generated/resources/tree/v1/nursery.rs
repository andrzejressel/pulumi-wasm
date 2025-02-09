#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod nursery {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NurseryArgs {
        /// The sizes of trees available
        #[builder(into, default)]
        pub sizes: pulumi_gestalt_rust::InputOrOutput<
            Option<
                std::collections::HashMap<
                    String,
                    super::super::super::types::tree::v1::TreeSize,
                >,
            >,
        >,
        /// The varieties available
        #[builder(into)]
        pub varieties: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::super::types::tree::v1::RubberTreeVariety>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NurseryArgs,
    ) {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let sizes_binding = args.sizes.get_output(context);
        let varieties_binding = args.varieties.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "plant:tree/v1:Nursery".into(),
            name: name.to_string(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sizes".into(),
                    value: sizes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "varieties".into(),
                    value: varieties_binding.get_id(),
                },
            ],
        };
        context.register_resource(request);
    }
}
