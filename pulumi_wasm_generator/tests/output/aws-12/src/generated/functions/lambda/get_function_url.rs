pub mod get_function_url {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFunctionUrlArgs {
        /// The name (or ARN) of the Lambda function.
        #[builder(into)]
        pub function_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Alias name or `"$LATEST"`.
        #[builder(into, default)]
        pub qualifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetFunctionUrlResult {
        /// Type of authentication that the function URL uses.
        pub authorization_type: pulumi_wasm_rust::Output<String>,
        /// The [cross-origin resource sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) settings for the function URL. See the `aws.lambda.FunctionUrl` resource documentation for more details.
        pub cors: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::lambda::GetFunctionUrlCor>,
        >,
        /// When the function URL was created, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime).
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// ARN of the function.
        pub function_arn: pulumi_wasm_rust::Output<String>,
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// HTTP URL endpoint for the function in the format `https://<url_id>.lambda-url.<region>.on.aws/`.
        pub function_url: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Whether the Lambda function responds in `BUFFERED` or `RESPONSE_STREAM` mode.
        pub invoke_mode: pulumi_wasm_rust::Output<String>,
        /// When the function URL configuration was last updated, in [ISO-8601 format](https://www.w3.org/TR/NOTE-datetime).
        pub last_modified_time: pulumi_wasm_rust::Output<String>,
        pub qualifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Generated ID for the endpoint.
        pub url_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetFunctionUrlArgs,
    ) -> GetFunctionUrlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let function_name_binding = args.function_name.get_output(context).get_inner();
        let qualifier_binding = args.qualifier.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lambda/getFunctionUrl:getFunctionUrl".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "functionName".into(),
                    value: &function_name_binding,
                },
                register_interface::ObjectField {
                    name: "qualifier".into(),
                    value: &qualifier_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authorizationType".into(),
                },
                register_interface::ResultField {
                    name: "cors".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "functionArn".into(),
                },
                register_interface::ResultField {
                    name: "functionName".into(),
                },
                register_interface::ResultField {
                    name: "functionUrl".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "invokeMode".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedTime".into(),
                },
                register_interface::ResultField {
                    name: "qualifier".into(),
                },
                register_interface::ResultField {
                    name: "urlId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFunctionUrlResult {
            authorization_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationType").unwrap(),
            ),
            cors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cors").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            function_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionArn").unwrap(),
            ),
            function_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionName").unwrap(),
            ),
            function_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionUrl").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            invoke_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invokeMode").unwrap(),
            ),
            last_modified_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedTime").unwrap(),
            ),
            qualifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("qualifier").unwrap(),
            ),
            url_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("urlId").unwrap(),
            ),
        }
    }
}
