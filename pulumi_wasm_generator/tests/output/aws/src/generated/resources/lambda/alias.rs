/// Creates a Lambda function alias. Creates an alias that points to the specified Lambda function version.
///
/// For information about Lambda and how to use it, see [What is AWS Lambda?](http://docs.aws.amazon.com/lambda/latest/dg/welcome.html)
/// For information about function aliases, see [CreateAlias](http://docs.aws.amazon.com/lambda/latest/dg/API_CreateAlias.html) and [AliasRoutingConfiguration](https://docs.aws.amazon.com/lambda/latest/dg/API_AliasRoutingConfiguration.html) in the API docs.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testLambdaAlias:
///     type: aws:lambda:Alias
///     name: test_lambda_alias
///     properties:
///       name: my_alias
///       description: a sample description
///       functionName: ${lambdaFunctionTest.arn}
///       functionVersion: '1'
///       routingConfig:
///         additionalVersionWeights:
///           '2': 0.5
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lambda Function Aliases using the `function_name/alias`. For example:
///
/// ```sh
/// $ pulumi import aws:lambda/alias:Alias test_lambda_alias my_test_lambda_function/my_alias
/// ```
pub mod alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AliasArgs {
        /// Description of the alias.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Lambda Function name or ARN.
        #[builder(into)]
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// Lambda function version for which you are creating the alias. Pattern: `(\$LATEST|[0-9]+)`.
        #[builder(into)]
        pub function_version: pulumi_wasm_rust::Output<String>,
        /// Name for the alias you are creating. Pattern: `(?!^[0-9]+$)([a-zA-Z0-9-_]+)`
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Lambda alias' route configuration settings. Fields documented below
        #[builder(into, default)]
        pub routing_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::AliasRoutingConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct AliasResult {
        /// The Amazon Resource Name (ARN) identifying your Lambda function alias.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the alias.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Lambda Function name or ARN.
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// Lambda function version for which you are creating the alias. Pattern: `(\$LATEST|[0-9]+)`.
        pub function_version: pulumi_wasm_rust::Output<String>,
        /// The ARN to be used for invoking Lambda Function from API Gateway - to be used in `aws.apigateway.Integration`'s `uri`
        pub invoke_arn: pulumi_wasm_rust::Output<String>,
        /// Name for the alias you are creating. Pattern: `(?!^[0-9]+$)([a-zA-Z0-9-_]+)`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Lambda alias' route configuration settings. Fields documented below
        pub routing_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::AliasRoutingConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AliasArgs) -> AliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let function_name_binding = args.function_name.get_inner();
        let function_version_binding = args.function_version.get_inner();
        let name_binding = args.name.get_inner();
        let routing_config_binding = args.routing_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lambda/alias:Alias".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "functionName".into(),
                    value: &function_name_binding,
                },
                register_interface::ObjectField {
                    name: "functionVersion".into(),
                    value: &function_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "routingConfig".into(),
                    value: &routing_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "functionName".into(),
                },
                register_interface::ResultField {
                    name: "functionVersion".into(),
                },
                register_interface::ResultField {
                    name: "invokeArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "routingConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AliasResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            function_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionName").unwrap(),
            ),
            function_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionVersion").unwrap(),
            ),
            invoke_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invokeArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            routing_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingConfig").unwrap(),
            ),
        }
    }
}