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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AliasArgs {
        /// Description of the alias.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Lambda Function name or ARN.
        #[builder(into)]
        pub function_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Lambda function version for which you are creating the alias. Pattern: `(\$LATEST|[0-9]+)`.
        #[builder(into)]
        pub function_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name for the alias you are creating. Pattern: `(?!^[0-9]+$)([a-zA-Z0-9-_]+)`
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Lambda alias' route configuration settings. Fields documented below
        #[builder(into, default)]
        pub routing_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::AliasRoutingConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct AliasResult {
        /// The Amazon Resource Name (ARN) identifying your Lambda function alias.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the alias.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Lambda Function name or ARN.
        pub function_name: pulumi_gestalt_rust::Output<String>,
        /// Lambda function version for which you are creating the alias. Pattern: `(\$LATEST|[0-9]+)`.
        pub function_version: pulumi_gestalt_rust::Output<String>,
        /// The ARN to be used for invoking Lambda Function from API Gateway - to be used in `aws.apigateway.Integration`'s `uri`
        pub invoke_arn: pulumi_gestalt_rust::Output<String>,
        /// Name for the alias you are creating. Pattern: `(?!^[0-9]+$)([a-zA-Z0-9-_]+)`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Lambda alias' route configuration settings. Fields documented below
        pub routing_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::lambda::AliasRoutingConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AliasArgs,
    ) -> AliasResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let function_name_binding_1 = args.function_name.get_output(context);
        let function_name_binding = function_name_binding_1.get_inner();
        let function_version_binding_1 = args.function_version.get_output(context);
        let function_version_binding = function_version_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let routing_config_binding_1 = args.routing_config.get_output(context);
        let routing_config_binding = routing_config_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lambda/alias:Alias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        AliasResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            function_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("functionName"),
            ),
            function_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("functionVersion"),
            ),
            invoke_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("invokeArn"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            routing_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routingConfig"),
            ),
        }
    }
}
