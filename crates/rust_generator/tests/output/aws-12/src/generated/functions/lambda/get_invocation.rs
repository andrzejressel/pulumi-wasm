#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_invocation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInvocationArgs {
        /// Name of the lambda function.
        #[builder(into)]
        pub function_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// String in JSON format that is passed as payload to the lambda function.
        #[builder(into)]
        pub input: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Qualifier (a.k.a version) of the lambda function. Defaults
        /// to `$LATEST`.
        #[builder(into, default)]
        pub qualifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInvocationResult {
        pub function_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub input: pulumi_gestalt_rust::Output<String>,
        pub qualifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// String result of the lambda function invocation.
        pub result: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInvocationArgs,
    ) -> GetInvocationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let function_name_binding = args.function_name.get_output(context);
        let input_binding = args.input.get_output(context);
        let qualifier_binding = args.qualifier.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lambda/getInvocation:getInvocation".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionName".into(),
                    value: &function_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "input".into(),
                    value: &input_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "qualifier".into(),
                    value: &qualifier_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInvocationResult {
            function_name: o.get_field("functionName"),
            id: o.get_field("id"),
            input: o.get_field("input"),
            qualifier: o.get_field("qualifier"),
            result: o.get_field("result"),
        }
    }
}
