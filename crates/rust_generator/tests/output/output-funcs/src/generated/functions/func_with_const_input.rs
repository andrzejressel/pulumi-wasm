#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod func_with_const_input {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FuncWithConstInputArgs {
        #[builder(into, default)]
        pub plain_input: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::constants::ConstStringFixed>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context, args: FuncWithConstInputArgs) {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let plain_input_binding = args.plain_input.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "mypkg::funcWithConstInput".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plainInput".into(),
                    value: &plain_input_binding.drop_type(),
                },
            ],
        };
        context.invoke_resource(request);
    }
}
