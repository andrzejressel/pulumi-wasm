/// Resource for managing an AWS Lambda Runtime Management Config.
///
/// Refer to the [AWS Lambda documentation](https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html) for supported runtimes.
///
/// > Deletion of this resource returns the runtime update mode to `Auto` (the default behavior).
/// To leave the configured runtime management options in-place, use a `removed` block with the destroy lifecycle set to `false`.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = runtime_management_config::create(
///         "example",
///         RuntimeManagementConfigArgs::builder()
///             .function_name("${test.functionName}")
///             .update_runtime_on("FunctionUpdate")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### `Manual` Update
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = runtime_management_config::create(
///         "example",
///         RuntimeManagementConfigArgs::builder()
///             .function_name("${test.functionName}")
///             .runtime_version_arn("arn:aws:lambda:us-east-1::runtime:abcd1234")
///             .update_runtime_on("Manual")
///             .build_struct(),
///     );
/// }
/// ```
///
/// > Once the runtime update mode is set to `Manual`, the `aws.lambda.Function` `runtime` cannot be updated. To upgrade a runtime, the `update_runtime_on` argument must be set to `Auto` or `FunctionUpdate` prior to changing the function's `runtime` argument.
///
/// ## Import
///
/// Using `pulumi import`, import Lambda Runtime Management Config using a comma-delimited string combining `function_name` and `qualifier`. For example:
///
/// ```sh
/// $ pulumi import aws:lambda/runtimeManagementConfig:RuntimeManagementConfig example my-function,$LATEST
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod runtime_management_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuntimeManagementConfigArgs {
        /// Name or ARN of the Lambda function.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub function_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the function. This can be `$LATEST` or a published version number. If omitted, this resource will manage the runtime configuration for `$LATEST`.
        #[builder(into, default)]
        pub qualifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the runtime version. Only required when `update_runtime_on` is `Manual`.
        #[builder(into, default)]
        pub runtime_version_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Runtime update mode. Valid values are `Auto`, `FunctionUpdate`, and `Manual`. When a function is created, the default mode is `Auto`.
        #[builder(into, default)]
        pub update_runtime_on: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RuntimeManagementConfigResult {
        /// ARN of the function.
        pub function_arn: pulumi_gestalt_rust::Output<String>,
        /// Name or ARN of the Lambda function.
        ///
        /// The following arguments are optional:
        pub function_name: pulumi_gestalt_rust::Output<String>,
        /// Version of the function. This can be `$LATEST` or a published version number. If omitted, this resource will manage the runtime configuration for `$LATEST`.
        pub qualifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the runtime version. Only required when `update_runtime_on` is `Manual`.
        pub runtime_version_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Runtime update mode. Valid values are `Auto`, `FunctionUpdate`, and `Manual`. When a function is created, the default mode is `Auto`.
        pub update_runtime_on: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RuntimeManagementConfigArgs,
    ) -> RuntimeManagementConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let function_name_binding = args.function_name.get_output(context);
        let qualifier_binding = args.qualifier.get_output(context);
        let runtime_version_arn_binding = args.runtime_version_arn.get_output(context);
        let update_runtime_on_binding = args.update_runtime_on.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lambda/runtimeManagementConfig:RuntimeManagementConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionName".into(),
                    value: function_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "qualifier".into(),
                    value: qualifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtimeVersionArn".into(),
                    value: runtime_version_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "updateRuntimeOn".into(),
                    value: update_runtime_on_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RuntimeManagementConfigResult {
            function_arn: o.get_field("functionArn"),
            function_name: o.get_field("functionName"),
            qualifier: o.get_field("qualifier"),
            runtime_version_arn: o.get_field("runtimeVersionArn"),
            update_runtime_on: o.get_field("updateRuntimeOn"),
        }
    }
}
