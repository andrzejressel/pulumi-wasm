/// Provides a Lambda function URL resource. A function URL is a dedicated HTTP(S) endpoint for a Lambda function.
///
/// See the [AWS Lambda documentation](https://docs.aws.amazon.com/lambda/latest/dg/lambda-urls.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testLatest = function_url::create(
///         "testLatest",
///         FunctionUrlArgs::builder()
///             .authorization_type("NONE")
///             .function_name("${test.functionName}")
///             .build_struct(),
///     );
///     let testLive = function_url::create(
///         "testLive",
///         FunctionUrlArgs::builder()
///             .authorization_type("AWS_IAM")
///             .cors(
///                 FunctionUrlCors::builder()
///                     .allowCredentials(true)
///                     .allowHeaders(vec!["date", "keep-alive",])
///                     .allowMethods(vec!["*",])
///                     .allowOrigins(vec!["*",])
///                     .exposeHeaders(vec!["keep-alive", "date",])
///                     .maxAge(86400)
///                     .build_struct(),
///             )
///             .function_name("${test.functionName}")
///             .qualifier("my_alias")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lambda function URLs using the `function_name` or `function_name/qualifier`. For example:
///
/// ```sh
/// $ pulumi import aws:lambda/functionUrl:FunctionUrl test_lambda_url my_test_lambda_function
/// ```
pub mod function_url {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionUrlArgs {
        /// The type of authentication that the function URL uses. Set to `"AWS_IAM"` to restrict access to authenticated IAM users only. Set to `"NONE"` to bypass IAM authentication and create a public endpoint. See the [AWS documentation](https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html) for more details.
        #[builder(into)]
        pub authorization_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The [cross-origin resource sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) settings for the function URL. Documented below.
        #[builder(into, default)]
        pub cors: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::lambda::FunctionUrlCors>,
        >,
        /// The name (or ARN) of the Lambda function.
        #[builder(into)]
        pub function_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Determines how the Lambda function responds to an invocation. Valid values are `BUFFERED` (default) and `RESPONSE_STREAM`. See more in [Configuring a Lambda function to stream responses](https://docs.aws.amazon.com/lambda/latest/dg/configuration-response-streaming.html).
        #[builder(into, default)]
        pub invoke_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The alias name or `"$LATEST"`.
        #[builder(into, default)]
        pub qualifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FunctionUrlResult {
        /// The type of authentication that the function URL uses. Set to `"AWS_IAM"` to restrict access to authenticated IAM users only. Set to `"NONE"` to bypass IAM authentication and create a public endpoint. See the [AWS documentation](https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html) for more details.
        pub authorization_type: pulumi_wasm_rust::Output<String>,
        /// The [cross-origin resource sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) settings for the function URL. Documented below.
        pub cors: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionUrlCors>,
        >,
        /// The Amazon Resource Name (ARN) of the function.
        pub function_arn: pulumi_wasm_rust::Output<String>,
        /// The name (or ARN) of the Lambda function.
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// The HTTP URL endpoint for the function in the format `https://<url_id>.lambda-url.<region>.on.aws/`.
        pub function_url: pulumi_wasm_rust::Output<String>,
        /// Determines how the Lambda function responds to an invocation. Valid values are `BUFFERED` (default) and `RESPONSE_STREAM`. See more in [Configuring a Lambda function to stream responses](https://docs.aws.amazon.com/lambda/latest/dg/configuration-response-streaming.html).
        pub invoke_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The alias name or `"$LATEST"`.
        pub qualifier: pulumi_wasm_rust::Output<Option<String>>,
        /// A generated ID for the endpoint.
        pub url_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FunctionUrlArgs,
    ) -> FunctionUrlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorization_type_binding = args
            .authorization_type
            .get_output(context)
            .get_inner();
        let cors_binding = args.cors.get_output(context).get_inner();
        let function_name_binding = args.function_name.get_output(context).get_inner();
        let invoke_mode_binding = args.invoke_mode.get_output(context).get_inner();
        let qualifier_binding = args.qualifier.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lambda/functionUrl:FunctionUrl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizationType".into(),
                    value: &authorization_type_binding,
                },
                register_interface::ObjectField {
                    name: "cors".into(),
                    value: &cors_binding,
                },
                register_interface::ObjectField {
                    name: "functionName".into(),
                    value: &function_name_binding,
                },
                register_interface::ObjectField {
                    name: "invokeMode".into(),
                    value: &invoke_mode_binding,
                },
                register_interface::ObjectField {
                    name: "qualifier".into(),
                    value: &qualifier_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FunctionUrlResult {
            authorization_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorizationType"),
            ),
            cors: pulumi_wasm_rust::__private::into_domain(o.extract_field("cors")),
            function_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("functionArn"),
            ),
            function_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("functionName"),
            ),
            function_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("functionUrl"),
            ),
            invoke_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("invokeMode"),
            ),
            qualifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("qualifier"),
            ),
            url_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("urlId")),
        }
    }
}
