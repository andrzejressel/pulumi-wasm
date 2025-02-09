#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod function_1 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Function1Args {
        #[builder(into, default)]
        pub common_type: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::common::CommonType>,
        >,
        #[builder(into, default)]
        pub type1: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::ns1::Type1>,
        >,
    }
    #[allow(dead_code)]
    pub struct Function1Result {
        pub result: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: Function1Args,
    ) -> Function1Result {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let common_type_binding = args.common_type.get_output(context);
        let type1_binding = args.type1.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "example:ns1:Function1".into(),
            version: super::super::super::get_version(),
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
        let o = context.invoke_resource(request);
        Function1Result {
            result: o.get_field("result"),
        }
    }
}
