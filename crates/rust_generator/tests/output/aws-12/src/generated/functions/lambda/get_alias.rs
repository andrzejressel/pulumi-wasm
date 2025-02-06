pub mod get_alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAliasArgs {
        /// Name of the aliased Lambda function.
        #[builder(into)]
        pub function_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the Lambda alias.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAliasResult {
        /// ARN identifying the Lambda function alias.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of alias.
        pub description: pulumi_wasm_rust::Output<String>,
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// Lambda function version which the alias uses.
        pub function_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN to be used for invoking Lambda Function from API Gateway - to be used in aws_api_gateway_integration's `uri`.
        pub invoke_arn: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAliasArgs,
    ) -> GetAliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let function_name_binding = args.function_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lambda/getAlias:getAlias".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "functionName".into(),
                    value: &function_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAliasResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            function_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("functionName"),
            ),
            function_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("functionVersion"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            invoke_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("invokeArn"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
