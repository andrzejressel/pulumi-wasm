#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_1 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Resource1Args {
        #[builder(into, default)]
        pub common_type: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::common::CommonType>,
        >,
        #[builder(into, default)]
        pub type1: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ns1::Type1>,
        >,
    }
    #[allow(dead_code)]
    pub struct Resource1Result {
        pub common_type: pulumi_gestalt_rust::Output<
            Option<super::super::types::common::CommonType>,
        >,
        pub type1: pulumi_gestalt_rust::Output<Option<super::super::types::ns1::Type1>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: Resource1Args,
    ) -> Resource1Result {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let common_type_binding = args.common_type.get_output(context);
        let type1_binding = args.type1.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "example:ns1:Resource1".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "commonType".into(),
                    value: common_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type1".into(),
                    value: type1_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        Resource1Result {
            common_type: o.get_field("commonType"),
            type1: o.get_field("type1"),
        }
    }
}
