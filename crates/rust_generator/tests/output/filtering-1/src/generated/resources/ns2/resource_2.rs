#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Resource2Args {
        #[builder(into, default)]
        pub common_type: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::common::CommonType>,
        >,
        #[builder(into, default)]
        pub type2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ns2::Type2>,
        >,
    }
    #[allow(dead_code)]
    pub struct Resource2Result {
        pub common_type: pulumi_gestalt_rust::Output<
            Option<super::super::types::common::CommonType>,
        >,
        pub type2: pulumi_gestalt_rust::Output<Option<super::super::types::ns2::Type2>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: Resource2Args,
    ) -> Resource2Result {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let common_type_binding = args.common_type.get_output(context);
        let type2_binding = args.type2.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "example:ns2:Resource2".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "commonType".into(),
                    value: common_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type2".into(),
                    value: type2_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        Resource2Result {
            common_type: o.get_field("commonType"),
            type2: o.get_field("type2"),
        }
    }
}
