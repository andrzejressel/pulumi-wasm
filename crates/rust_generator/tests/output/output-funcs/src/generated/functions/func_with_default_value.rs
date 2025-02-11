#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod func_with_default_value {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FuncWithDefaultValueArgs {
        #[builder(into)]
        pub a: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub b: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FuncWithDefaultValueResult {
        pub r: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: FuncWithDefaultValueArgs,
    ) -> FuncWithDefaultValueResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let a_binding = args.a.get_output(context);
        let b_binding = args.b.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "mypkg::funcWithDefaultValue".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "a".into(),
                    value: &a_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "b".into(),
                    value: &b_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        FuncWithDefaultValueResult {
            r: o.get_field("r"),
        }
    }
}
