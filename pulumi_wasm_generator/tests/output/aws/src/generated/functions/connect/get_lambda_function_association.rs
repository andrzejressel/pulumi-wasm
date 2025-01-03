pub mod get_lambda_function_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLambdaFunctionAssociationArgs {
        /// ARN of the Lambda Function, omitting any version or alias qualifier.
        #[builder(into)]
        pub function_arn: pulumi_wasm_rust::Output<String>,
        /// Identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetLambdaFunctionAssociationResult {
        pub function_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetLambdaFunctionAssociationArgs,
    ) -> GetLambdaFunctionAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let function_arn_binding = args.function_arn.get_inner();
        let instance_id_binding = args.instance_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getLambdaFunctionAssociation:getLambdaFunctionAssociation"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "functionArn".into(),
                    value: &function_arn_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "functionArn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLambdaFunctionAssociationResult {
            function_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionArn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
        }
    }
}
