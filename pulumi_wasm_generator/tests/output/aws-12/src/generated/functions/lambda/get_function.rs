pub mod get_function {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFunctionArgs {
        /// Name of the lambda function.
        #[builder(into)]
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// Alias name or version number of the lambda functionE.g., `$LATEST`, `my-alias`, or `1`. When not included: the data source resolves to the most recent published version; if no published version exists: it resolves to the most recent unpublished version.
        #[builder(into, default)]
        pub qualifier: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFunctionResult {
        /// Instruction set architecture for the Lambda function.
        pub architectures: pulumi_wasm_rust::Output<Vec<String>>,
        /// Unqualified (no `:QUALIFIER` or `:VERSION` suffix) ARN identifying your Lambda Function. See also `qualified_arn`.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Base64-encoded representation of raw SHA-256 sum of the zip file.
        pub code_sha256: pulumi_wasm_rust::Output<String>,
        /// ARN for a Code Signing Configuration.
        pub code_signing_config_arn: pulumi_wasm_rust::Output<String>,
        /// Configure the function's *dead letter queue*.
        pub dead_letter_config: pulumi_wasm_rust::Output<
            super::super::super::types::lambda::GetFunctionDeadLetterConfig,
        >,
        /// Description of what your Lambda Function does.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Lambda environment's configuration settings.
        pub environment: pulumi_wasm_rust::Output<
            super::super::super::types::lambda::GetFunctionEnvironment,
        >,
        /// Amount of Ephemeral storage(`/tmp`) allocated for the Lambda Function.
        pub ephemeral_storages: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::lambda::GetFunctionEphemeralStorage>,
        >,
        /// Connection settings for an Amazon EFS file system.
        pub file_system_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::lambda::GetFunctionFileSystemConfig>,
        >,
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// Function entrypoint in your code.
        pub handler: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// URI of the container image.
        pub image_uri: pulumi_wasm_rust::Output<String>,
        /// ARN to be used for invoking Lambda Function from API Gateway. **NOTE:** Starting with `v4.51.0` of the provider, this will *not* include the qualifier.
        pub invoke_arn: pulumi_wasm_rust::Output<String>,
        /// ARN for the KMS encryption key.
        pub kms_key_arn: pulumi_wasm_rust::Output<String>,
        /// Date this resource was last modified.
        pub last_modified: pulumi_wasm_rust::Output<String>,
        /// List of Lambda Layer ARNs attached to your Lambda Function.
        pub layers: pulumi_wasm_rust::Output<Vec<String>>,
        /// Advanced logging settings.
        pub logging_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::lambda::GetFunctionLoggingConfig>,
        >,
        /// Amount of memory in MB your Lambda Function can use at runtime.
        pub memory_size: pulumi_wasm_rust::Output<i32>,
        /// Qualified (`:QUALIFIER` or `:VERSION` suffix) ARN identifying your Lambda Function. See also `arn`.
        pub qualified_arn: pulumi_wasm_rust::Output<String>,
        /// Qualified (`:QUALIFIER` or `:VERSION` suffix) ARN to be used for invoking Lambda Function from API Gateway. See also `invoke_arn`.
        pub qualified_invoke_arn: pulumi_wasm_rust::Output<String>,
        pub qualifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The amount of reserved concurrent executions for this lambda function or `-1` if unreserved.
        pub reserved_concurrent_executions: pulumi_wasm_rust::Output<i32>,
        /// IAM role attached to the Lambda Function.
        pub role: pulumi_wasm_rust::Output<String>,
        /// Runtime environment for the Lambda function.
        pub runtime: pulumi_wasm_rust::Output<String>,
        /// ARN of a signing job.
        pub signing_job_arn: pulumi_wasm_rust::Output<String>,
        /// The ARN for a signing profile version.
        pub signing_profile_version_arn: pulumi_wasm_rust::Output<String>,
        /// (**Deprecated** use `code_sha256` instead) Base64-encoded representation of raw SHA-256 sum of the zip file.
        pub source_code_hash: pulumi_wasm_rust::Output<String>,
        /// Size in bytes of the function .zip file.
        pub source_code_size: pulumi_wasm_rust::Output<i32>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Function execution time at which Lambda should terminate the function.
        pub timeout: pulumi_wasm_rust::Output<i32>,
        /// Tracing settings of the function.
        pub tracing_config: pulumi_wasm_rust::Output<
            super::super::super::types::lambda::GetFunctionTracingConfig,
        >,
        /// The version of the Lambda function returned. If `qualifier` is not set, this will resolve to the most recent published version. If no published version of the function exists, `version` will resolve to `$LATEST`.
        pub version: pulumi_wasm_rust::Output<String>,
        /// VPC configuration associated with your Lambda function.
        pub vpc_config: pulumi_wasm_rust::Output<
            super::super::super::types::lambda::GetFunctionVpcConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFunctionArgs) -> GetFunctionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let function_name_binding = args.function_name.get_inner();
        let qualifier_binding = args.qualifier.get_inner();
        let tags_binding = args.tags.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "architectures".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "codeSha256".into(),
                },
                register_interface::ResultField {
                    name: "codeSigningConfigArn".into(),
                },
                register_interface::ResultField {
                    name: "deadLetterConfig".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "environment".into(),
                },
                register_interface::ResultField {
                    name: "ephemeralStorages".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemConfigs".into(),
                },
                register_interface::ResultField {
                    name: "functionName".into(),
                },
                register_interface::ResultField {
                    name: "handler".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageUri".into(),
                },
                register_interface::ResultField {
                    name: "invokeArn".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "lastModified".into(),
                },
                register_interface::ResultField {
                    name: "layers".into(),
                },
                register_interface::ResultField {
                    name: "loggingConfigs".into(),
                },
                register_interface::ResultField {
                    name: "memorySize".into(),
                },
                register_interface::ResultField {
                    name: "qualifiedArn".into(),
                },
                register_interface::ResultField {
                    name: "qualifiedInvokeArn".into(),
                },
                register_interface::ResultField {
                    name: "qualifier".into(),
                },
                register_interface::ResultField {
                    name: "reservedConcurrentExecutions".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
                register_interface::ResultField {
                    name: "runtime".into(),
                },
                register_interface::ResultField {
                    name: "signingJobArn".into(),
                },
                register_interface::ResultField {
                    name: "signingProfileVersionArn".into(),
                },
                register_interface::ResultField {
                    name: "sourceCodeHash".into(),
                },
                register_interface::ResultField {
                    name: "sourceCodeSize".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "timeout".into(),
                },
                register_interface::ResultField {
                    name: "tracingConfig".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "vpcConfig".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFunctionResult {
            architectures: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("architectures").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            code_sha256: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("codeSha256").unwrap(),
            ),
            code_signing_config_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("codeSigningConfigArn").unwrap(),
            ),
            dead_letter_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deadLetterConfig").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            environment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environment").unwrap(),
            ),
            ephemeral_storages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ephemeralStorages").unwrap(),
            ),
            file_system_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemConfigs").unwrap(),
            ),
            function_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionName").unwrap(),
            ),
            handler: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("handler").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageUri").unwrap(),
            ),
            invoke_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invokeArn").unwrap(),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
            ),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModified").unwrap(),
            ),
            layers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("layers").unwrap(),
            ),
            logging_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingConfigs").unwrap(),
            ),
            memory_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memorySize").unwrap(),
            ),
            qualified_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("qualifiedArn").unwrap(),
            ),
            qualified_invoke_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("qualifiedInvokeArn").unwrap(),
            ),
            qualifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("qualifier").unwrap(),
            ),
            reserved_concurrent_executions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservedConcurrentExecutions").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
            runtime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtime").unwrap(),
            ),
            signing_job_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingJobArn").unwrap(),
            ),
            signing_profile_version_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingProfileVersionArn").unwrap(),
            ),
            source_code_hash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceCodeHash").unwrap(),
            ),
            source_code_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceCodeSize").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeout").unwrap(),
            ),
            tracing_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tracingConfig").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            vpc_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConfig").unwrap(),
            ),
        }
    }
}
