#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod function_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Function2Args {
        #[builder(into, default)]
        pub common_type: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::common::CommonType>,
        >,
        #[builder(into, default)]
        pub type2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::ns2::Type2>,
        >,
    }
    #[allow(dead_code)]
    pub struct Function2Result {
        pub result: pulumi_gestalt_rust::Output<Option<f64>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: Function2Args,
    ) -> Function2Result {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let common_type_binding = args.common_type.get_output(context);
        let type2_binding = args.type2.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "example:ns2:Function2".into(),
            version: super::super::super::get_version(),
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
        let o = context.invoke_resource(request);
        Function2Result {
            result: o.get_field("result"),
        }
    }
}
