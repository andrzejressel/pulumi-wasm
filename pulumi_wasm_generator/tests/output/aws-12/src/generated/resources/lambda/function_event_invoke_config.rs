/// Manages an asynchronous invocation configuration for a Lambda Function or Alias. More information about asynchronous invocations and the configurable values can be found in the [Lambda Developer Guide](https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html).
///
/// ## Example Usage
///
/// ### Destination Configuration
///
/// > **NOTE:** Ensure the Lambda Function IAM Role has necessary permissions for the destination, such as `sqs:SendMessage` or `sns:Publish`, otherwise the API will return a generic `InvalidParameterValueException: The destination ARN arn:PARTITION:SERVICE:REGION:ACCOUNT:RESOURCE is invalid.` error.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = function_event_invoke_config::create(
///         "example",
///         FunctionEventInvokeConfigArgs::builder()
///             .destination_config(
///                 FunctionEventInvokeConfigDestinationConfig::builder()
///                     .onFailure(
///                         FunctionEventInvokeConfigDestinationConfigOnFailure::builder()
///                             .destination("${exampleAwsSqsQueue.arn}")
///                             .build_struct(),
///                     )
///                     .onSuccess(
///                         FunctionEventInvokeConfigDestinationConfigOnSuccess::builder()
///                             .destination("${exampleAwsSnsTopic.arn}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .function_name("${exampleAwsLambdaAlias.functionName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Error Handling Configuration
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = function_event_invoke_config::create(
///         "example",
///         FunctionEventInvokeConfigArgs::builder()
///             .function_name("${exampleAwsLambdaAlias.functionName}")
///             .maximum_event_age_in_seconds(60)
///             .maximum_retry_attempts(0)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Configuration for Alias Name
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = function_event_invoke_config::create(
///         "example",
///         FunctionEventInvokeConfigArgs::builder()
///             .function_name("${exampleAwsLambdaAlias.functionName}")
///             .qualifier("${exampleAwsLambdaAlias.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Configuration for Function Latest Unpublished Version
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = function_event_invoke_config::create(
///         "example",
///         FunctionEventInvokeConfigArgs::builder()
///             .function_name("${exampleAwsLambdaFunction.functionName}")
///             .qualifier("$LATEST")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Configuration for Function Published Version
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = function_event_invoke_config::create(
///         "example",
///         FunctionEventInvokeConfigArgs::builder()
///             .function_name("${exampleAwsLambdaFunction.functionName}")
///             .qualifier("${exampleAwsLambdaFunction.version}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ARN with qualifier:
///
/// Name without qualifier (all versions and aliases):
///
/// Name with qualifier:
///
/// __Using `pulumi import` to import__ Lambda Function Event Invoke Configs using the fully qualified Function name or Amazon Resource Name (ARN). For example:
///
/// ARN without qualifier (all versions and aliases):
///
/// ```sh
/// $ pulumi import aws:lambda/functionEventInvokeConfig:FunctionEventInvokeConfig example arn:aws:us-east-1:123456789012:function:my_function
/// ```
/// ARN with qualifier:
///
/// ```sh
/// $ pulumi import aws:lambda/functionEventInvokeConfig:FunctionEventInvokeConfig example arn:aws:us-east-1:123456789012:function:my_function:production
/// ```
/// Name without qualifier (all versions and aliases):
///
/// ```sh
/// $ pulumi import aws:lambda/functionEventInvokeConfig:FunctionEventInvokeConfig example my_function
/// ```
/// Name with qualifier:
///
/// ```sh
/// $ pulumi import aws:lambda/functionEventInvokeConfig:FunctionEventInvokeConfig example my_function:production
/// ```
pub mod function_event_invoke_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionEventInvokeConfigArgs {
        /// Configuration block with destination configuration. See below for details.
        #[builder(into, default)]
        pub destination_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::lambda::FunctionEventInvokeConfigDestinationConfig,
            >,
        >,
        /// Name or Amazon Resource Name (ARN) of the Lambda Function, omitting any version or alias qualifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub function_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Maximum age of a request that Lambda sends to a function for processing in seconds. Valid values between 60 and 21600.
        #[builder(into, default)]
        pub maximum_event_age_in_seconds: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Maximum number of times to retry when the function returns an error. Valid values between 0 and 2. Defaults to 2.
        #[builder(into, default)]
        pub maximum_retry_attempts: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Lambda Function published version, `$LATEST`, or Lambda Alias name.
        #[builder(into, default)]
        pub qualifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FunctionEventInvokeConfigResult {
        /// Configuration block with destination configuration. See below for details.
        pub destination_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::lambda::FunctionEventInvokeConfigDestinationConfig,
            >,
        >,
        /// Name or Amazon Resource Name (ARN) of the Lambda Function, omitting any version or alias qualifier.
        ///
        /// The following arguments are optional:
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// Maximum age of a request that Lambda sends to a function for processing in seconds. Valid values between 60 and 21600.
        pub maximum_event_age_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Maximum number of times to retry when the function returns an error. Valid values between 0 and 2. Defaults to 2.
        pub maximum_retry_attempts: pulumi_wasm_rust::Output<Option<i32>>,
        /// Lambda Function published version, `$LATEST`, or Lambda Alias name.
        pub qualifier: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FunctionEventInvokeConfigArgs,
    ) -> FunctionEventInvokeConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_config_binding = args
            .destination_config
            .get_output(context)
            .get_inner();
        let function_name_binding = args.function_name.get_output(context).get_inner();
        let maximum_event_age_in_seconds_binding = args
            .maximum_event_age_in_seconds
            .get_output(context)
            .get_inner();
        let maximum_retry_attempts_binding = args
            .maximum_retry_attempts
            .get_output(context)
            .get_inner();
        let qualifier_binding = args.qualifier.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lambda/functionEventInvokeConfig:FunctionEventInvokeConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationConfig".into(),
                    value: &destination_config_binding,
                },
                register_interface::ObjectField {
                    name: "functionName".into(),
                    value: &function_name_binding,
                },
                register_interface::ObjectField {
                    name: "maximumEventAgeInSeconds".into(),
                    value: &maximum_event_age_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "maximumRetryAttempts".into(),
                    value: &maximum_retry_attempts_binding,
                },
                register_interface::ObjectField {
                    name: "qualifier".into(),
                    value: &qualifier_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "destinationConfig".into(),
                },
                register_interface::ResultField {
                    name: "functionName".into(),
                },
                register_interface::ResultField {
                    name: "maximumEventAgeInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "maximumRetryAttempts".into(),
                },
                register_interface::ResultField {
                    name: "qualifier".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FunctionEventInvokeConfigResult {
            destination_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationConfig").unwrap(),
            ),
            function_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionName").unwrap(),
            ),
            maximum_event_age_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumEventAgeInSeconds").unwrap(),
            ),
            maximum_retry_attempts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumRetryAttempts").unwrap(),
            ),
            qualifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("qualifier").unwrap(),
            ),
        }
    }
}
