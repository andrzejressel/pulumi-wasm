#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod type_ {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TypeArgs {
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::super::super::types::impl_::let_::loop_::Type>,
        >,
    }
    #[allow(dead_code)]
    pub struct TypeResult {
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context, args: TypeArgs) -> TypeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "example:impl/let/loop:Type".into(),
            version: super::super::super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        TypeResult {
            type_: o.get_field("type"),
        }
    }
}
