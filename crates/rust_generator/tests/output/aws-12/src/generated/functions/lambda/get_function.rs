#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetFunctionArgs,
    ) -> GetFunctionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let function_name_binding = args.function_name.get_output(context);
        let qualifier_binding = args.qualifier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lambda/getFunction:getFunction".into(),
            version: super::super::super::get_version(),
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
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFunctionResult {
            architectures: o.get_field("architectures"),
            arn: o.get_field("arn"),
            code_sha256: o.get_field("codeSha256"),
            code_signing_config_arn: o.get_field("codeSigningConfigArn"),
            dead_letter_config: o.get_field("deadLetterConfig"),
            description: o.get_field("description"),
            environment: o.get_field("environment"),
            ephemeral_storages: o.get_field("ephemeralStorages"),
            file_system_configs: o.get_field("fileSystemConfigs"),
            function_name: o.get_field("functionName"),
            handler: o.get_field("handler"),
            id: o.get_field("id"),
            image_uri: o.get_field("imageUri"),
            invoke_arn: o.get_field("invokeArn"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            last_modified: o.get_field("lastModified"),
            layers: o.get_field("layers"),
            logging_configs: o.get_field("loggingConfigs"),
            memory_size: o.get_field("memorySize"),
            qualified_arn: o.get_field("qualifiedArn"),
            qualified_invoke_arn: o.get_field("qualifiedInvokeArn"),
            qualifier: o.get_field("qualifier"),
            reserved_concurrent_executions: o.get_field("reservedConcurrentExecutions"),
            role: o.get_field("role"),
            runtime: o.get_field("runtime"),
            signing_job_arn: o.get_field("signingJobArn"),
            signing_profile_version_arn: o.get_field("signingProfileVersionArn"),
            source_code_hash: o.get_field("sourceCodeHash"),
            source_code_size: o.get_field("sourceCodeSize"),
            tags: o.get_field("tags"),
            timeout: o.get_field("timeout"),
            tracing_config: o.get_field("tracingConfig"),
            version: o.get_field("version"),
            vpc_config: o.get_field("vpcConfig"),
        }
    }
}
