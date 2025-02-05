/// Provides an Amazon Connect Lambda Function Association. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html) and [Invoke AWS Lambda functions](https://docs.aws.amazon.com/connect/latest/adminguide/connect-lambda-functions.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = lambda_function_association::create(
///         "example",
///         LambdaFunctionAssociationArgs::builder()
///             .function_arn("${exampleAwsLambdaFunction.arn}")
///             .instance_id("${exampleAwsConnectInstance.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_connect_lambda_function_association` using the `instance_id` and `function_arn` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/lambdaFunctionAssociation:LambdaFunctionAssociation example aaaaaaaa-bbbb-cccc-dddd-111111111111,arn:aws:lambda:us-west-2:123456789123:function:example
/// ```
pub mod lambda_function_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LambdaFunctionAssociationArgs {
        /// Amazon Resource Name (ARN) of the Lambda Function, omitting any version or alias qualifier.
        #[builder(into)]
        pub function_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LambdaFunctionAssociationResult {
        /// Amazon Resource Name (ARN) of the Lambda Function, omitting any version or alias qualifier.
        pub function_arn: pulumi_wasm_rust::Output<String>,
        /// The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
        pub instance_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LambdaFunctionAssociationArgs,
    ) -> LambdaFunctionAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let function_arn_binding = args.function_arn.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:connect/lambdaFunctionAssociation:LambdaFunctionAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        LambdaFunctionAssociationResult {
            function_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("functionArn"),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
        }
    }
}
