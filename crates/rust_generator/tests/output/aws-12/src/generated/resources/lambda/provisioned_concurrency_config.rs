/// Manages a Lambda Provisioned Concurrency Configuration.
///
/// > **NOTE:** Setting `skip_destroy` to `true` means that the AWS Provider will _not_ destroy a provisioned concurrency configuration, even when running `pulumi destroy`. The configuration is thus an intentional dangling resource that is _not_ managed by Pulumi and may incur extra expense in your AWS account.
///
/// ## Example Usage
///
/// ### Alias Name
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = provisioned_concurrency_config::create(
///         "example",
///         ProvisionedConcurrencyConfigArgs::builder()
///             .function_name("${exampleAwsLambdaAlias.functionName}")
///             .provisioned_concurrent_executions(1)
///             .qualifier("${exampleAwsLambdaAlias.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Function Version
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = provisioned_concurrency_config::create(
///         "example",
///         ProvisionedConcurrencyConfigArgs::builder()
///             .function_name("${exampleAwsLambdaFunction.functionName}")
///             .provisioned_concurrent_executions(1)
///             .qualifier("${exampleAwsLambdaFunction.version}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a Lambda Provisioned Concurrency Configuration using the `function_name` and `qualifier` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:lambda/provisionedConcurrencyConfig:ProvisionedConcurrencyConfig example my_function,production
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod provisioned_concurrency_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProvisionedConcurrencyConfigArgs {
        /// Name or Amazon Resource Name (ARN) of the Lambda Function.
        #[builder(into)]
        pub function_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Amount of capacity to allocate. Must be greater than or equal to `1`.
        #[builder(into)]
        pub provisioned_concurrent_executions: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Lambda Function version or Lambda Alias name.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub qualifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to retain the provisoned concurrency configuration upon destruction. Defaults to `false`. If set to `true`, the resource in simply removed from state instead.
        #[builder(into, default)]
        pub skip_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ProvisionedConcurrencyConfigResult {
        /// Name or Amazon Resource Name (ARN) of the Lambda Function.
        pub function_name: pulumi_gestalt_rust::Output<String>,
        /// Amount of capacity to allocate. Must be greater than or equal to `1`.
        pub provisioned_concurrent_executions: pulumi_gestalt_rust::Output<i32>,
        /// Lambda Function version or Lambda Alias name.
        ///
        /// The following arguments are optional:
        pub qualifier: pulumi_gestalt_rust::Output<String>,
        /// Whether to retain the provisoned concurrency configuration upon destruction. Defaults to `false`. If set to `true`, the resource in simply removed from state instead.
        pub skip_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProvisionedConcurrencyConfigArgs,
    ) -> ProvisionedConcurrencyConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let function_name_binding = args.function_name.get_output(context);
        let provisioned_concurrent_executions_binding = args
            .provisioned_concurrent_executions
            .get_output(context);
        let qualifier_binding = args.qualifier.get_output(context);
        let skip_destroy_binding = args.skip_destroy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lambda/provisionedConcurrencyConfig:ProvisionedConcurrencyConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionName".into(),
                    value: function_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "provisionedConcurrentExecutions".into(),
                    value: provisioned_concurrent_executions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "qualifier".into(),
                    value: qualifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipDestroy".into(),
                    value: skip_destroy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProvisionedConcurrencyConfigResult {
            function_name: o.get_field("functionName"),
            provisioned_concurrent_executions: o
                .get_field("provisionedConcurrentExecutions"),
            qualifier: o.get_field("qualifier"),
            skip_destroy: o.get_field("skipDestroy"),
        }
    }
}
