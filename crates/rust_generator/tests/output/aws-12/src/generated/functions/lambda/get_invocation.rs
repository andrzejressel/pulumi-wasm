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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInvocationArgs,
    ) -> GetInvocationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let function_name_binding_1 = args.function_name.get_output(context);
        let function_name_binding = function_name_binding_1.get_inner();
        let input_binding_1 = args.input.get_output(context);
        let input_binding = input_binding_1.get_inner();
        let qualifier_binding_1 = args.qualifier.get_output(context);
        let qualifier_binding = qualifier_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lambda/getInvocation:getInvocation".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "functionName".into(),
                    value: &function_name_binding,
                },
                register_interface::ObjectField {
                    name: "input".into(),
                    value: &input_binding,
                },
                register_interface::ObjectField {
                    name: "qualifier".into(),
                    value: &qualifier_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInvocationResult {
            function_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("functionName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            input: pulumi_gestalt_rust::__private::into_domain(o.extract_field("input")),
            qualifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("qualifier"),
            ),
            result: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("result"),
            ),
        }
    }
}
