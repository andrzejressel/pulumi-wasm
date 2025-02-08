#[allow(clippy::doc_lazy_continuation)]
pub mod get_function {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFunctionArgs {
        /// Name of the lambda function.
        #[builder(into)]
        pub function_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Alias name or version number of the lambda functionE.g., `$LATEST`, `my-alias`, or `1`. When not included: the data source resolves to the most recent published version; if no published version exists: it resolves to the most recent unpublished version.
        #[builder(into, default)]
        pub qualifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFunctionResult {
        /// Instruction set architecture for the Lambda function.
        pub architectures: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Unqualified (no `:QUALIFIER` or `:VERSION` suffix) ARN identifying your Lambda Function. See also `qualified_arn`.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Base64-encoded representation of raw SHA-256 sum of the zip file.
        pub code_sha256: pulumi_gestalt_rust::Output<String>,
        /// ARN for a Code Signing Configuration.
        pub code_signing_config_arn: pulumi_gestalt_rust::Output<String>,
        /// Configure the function's *dead letter queue*.
        pub dead_letter_config: pulumi_gestalt_rust::Output<
            super::super::super::types::lambda::GetFunctionDeadLetterConfig,
        >,
        /// Description of what your Lambda Function does.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Lambda environment's configuration settings.
        pub environment: pulumi_gestalt_rust::Output<
            super::super::super::types::lambda::GetFunctionEnvironment,
        >,
        /// Amount of Ephemeral storage(`/tmp`) allocated for the Lambda Function.
        pub ephemeral_storages: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lambda::GetFunctionEphemeralStorage>,
        >,
        /// Connection settings for an Amazon EFS file system.
        pub file_system_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lambda::GetFunctionFileSystemConfig>,
        >,
        pub function_name: pulumi_gestalt_rust::Output<String>,
        /// Function entrypoint in your code.
        pub handler: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// URI of the container image.
        pub image_uri: pulumi_gestalt_rust::Output<String>,
        /// ARN to be used for invoking Lambda Function from API Gateway. **NOTE:** Starting with `v4.51.0` of the provider, this will *not* include the qualifier.
        pub invoke_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN for the KMS encryption key.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Date this resource was last modified.
        pub last_modified: pulumi_gestalt_rust::Output<String>,
        /// List of Lambda Layer ARNs attached to your Lambda Function.
        pub layers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Advanced logging settings.
        pub logging_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lambda::GetFunctionLoggingConfig>,
        >,
        /// Amount of memory in MB your Lambda Function can use at runtime.
        pub memory_size: pulumi_gestalt_rust::Output<i32>,
        /// Qualified (`:QUALIFIER` or `:VERSION` suffix) ARN identifying your Lambda Function. See also `arn`.
        pub qualified_arn: pulumi_gestalt_rust::Output<String>,
        /// Qualified (`:QUALIFIER` or `:VERSION` suffix) ARN to be used for invoking Lambda Function from API Gateway. See also `invoke_arn`.
        pub qualified_invoke_arn: pulumi_gestalt_rust::Output<String>,
        pub qualifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// The amount of reserved concurrent executions for this lambda function or `-1` if unreserved.
        pub reserved_concurrent_executions: pulumi_gestalt_rust::Output<i32>,
        /// IAM role attached to the Lambda Function.
        pub role: pulumi_gestalt_rust::Output<String>,
        /// Runtime environment for the Lambda function.
        pub runtime: pulumi_gestalt_rust::Output<String>,
        /// ARN of a signing job.
        pub signing_job_arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN for a signing profile version.
        pub signing_profile_version_arn: pulumi_gestalt_rust::Output<String>,
        /// (**Deprecated** use `code_sha256` instead) Base64-encoded representation of raw SHA-256 sum of the zip file.
        pub source_code_hash: pulumi_gestalt_rust::Output<String>,
        /// Size in bytes of the function .zip file.
        pub source_code_size: pulumi_gestalt_rust::Output<i32>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Function execution time at which Lambda should terminate the function.
        pub timeout: pulumi_gestalt_rust::Output<i32>,
        /// Tracing settings of the function.
        pub tracing_config: pulumi_gestalt_rust::Output<
            super::super::super::types::lambda::GetFunctionTracingConfig,
        >,
        /// The version of the Lambda function returned. If `qualifier` is not set, this will resolve to the most recent published version. If no published version of the function exists, `version` will resolve to `$LATEST`.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// VPC configuration associated with your Lambda function.
        pub vpc_config: pulumi_gestalt_rust::Output<
            super::super::super::types::lambda::GetFunctionVpcConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFunctionArgs,
    ) -> GetFunctionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let function_name_binding = args.function_name.get_output(context).get_inner();
        let qualifier_binding = args.qualifier.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lambda/getFunction:getFunction".into(),
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
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFunctionResult {
            architectures: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("architectures"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            code_sha256: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("codeSha256"),
            ),
            code_signing_config_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("codeSigningConfigArn"),
            ),
            dead_letter_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deadLetterConfig"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            environment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environment"),
            ),
            ephemeral_storages: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ephemeralStorages"),
            ),
            file_system_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileSystemConfigs"),
            ),
            function_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("functionName"),
            ),
            handler: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("handler"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            image_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageUri"),
            ),
            invoke_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("invokeArn"),
            ),
            kms_key_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyArn"),
            ),
            last_modified: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModified"),
            ),
            layers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("layers"),
            ),
            logging_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingConfigs"),
            ),
            memory_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("memorySize"),
            ),
            qualified_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("qualifiedArn"),
            ),
            qualified_invoke_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("qualifiedInvokeArn"),
            ),
            qualifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("qualifier"),
            ),
            reserved_concurrent_executions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reservedConcurrentExecutions"),
            ),
            role: pulumi_gestalt_rust::__private::into_domain(o.extract_field("role")),
            runtime: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runtime"),
            ),
            signing_job_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("signingJobArn"),
            ),
            signing_profile_version_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("signingProfileVersionArn"),
            ),
            source_code_hash: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceCodeHash"),
            ),
            source_code_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceCodeSize"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeout"),
            ),
            tracing_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tracingConfig"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            vpc_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcConfig"),
            ),
        }
    }
}
