/// Provides a Lambda Function resource. Lambda allows you to trigger execution of code in response to events in AWS, enabling serverless backend solutions. The Lambda Function itself includes source code and runtime configuration.
///
/// For information about Lambda and how to use it, see [What is AWS Lambda?](https://docs.aws.amazon.com/lambda/latest/dg/welcome.html)
///
///
/// > **NOTE:** Due to [AWS Lambda improved VPC networking changes that began deploying in September 2019](https://aws.amazon.com/blogs/compute/announcing-improved-vpc-networking-for-aws-lambda-functions/), EC2 subnets and security groups associated with Lambda Functions can take up to 45 minutes to successfully delete. To allow for successful deletion, the provider will wait for at least 45 minutes even if a shorter delete timeout is specified.
///
/// > **NOTE:** If you get a `KMSAccessDeniedException: Lambda was unable to decrypt the environment variables because KMS access was denied` error when invoking an `aws.lambda.Function` with environment variables, the IAM role associated with the function may have been deleted and recreated _after_ the function was created. You can fix the problem two ways: 1) updating the function's role to another role and then updating it back again to the recreated role, or 2) by using Pulumi to `taint` the function and `apply` your configuration again to recreate the function. (When you create a function, Lambda grants permissions on the KMS key to the function's IAM role. If the IAM role is recreated, the grant is no longer valid. Changing the function's role or recreating the function causes Lambda to update the grant.)
///
/// > To give an external source (like an EventBridge Rule, SNS, or S3) permission to access the Lambda function, use the `aws.lambda.Permission` resource. See [Lambda Permission Model](https://docs.aws.amazon.com/lambda/latest/dg/intro-permission-model.html) for more details. On the other hand, the `role` argument of this resource is the function's execution role for identity and access to AWS services and resources.
///
/// ## Example Usage
///
/// ### Basic Example
///
/// ```yaml
/// resources:
///   iamForLambda:
///     type: aws:iam:Role
///     name: iam_for_lambda
///     properties:
///       name: iam_for_lambda
///       assumeRolePolicy: ${assumeRole.json}
///   testLambda:
///     type: aws:lambda:Function
///     name: test_lambda
///     properties:
///       code:
///         fn::FileArchive: lambda_function_payload.zip
///       name: lambda_function_name
///       role: ${iamForLambda.arn}
///       handler: index.test
///       sourceCodeHash: ${lambda.outputBase64sha256}
///       runtime: nodejs18.x
///       environment:
///         variables:
///           foo: bar
/// variables:
///   assumeRole:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - lambda.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   lambda:
///     fn::invoke:
///       Function: archive:getFile
///       Arguments:
///         type: zip
///         sourceFile: lambda.js
///         outputPath: lambda_function_payload.zip
/// ```
///
/// ### Lambda Layers
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = layer_version::create(
///         "example",
///         LayerVersionArgs::builder().build_struct(),
///     );
///     let exampleFunction = function::create(
///         "exampleFunction",
///         FunctionArgs::builder().layers(vec!["${example.arn}",]).build_struct(),
///     );
/// }
/// ```
///
/// ### Lambda Ephemeral Storage
///
/// Lambda Function Ephemeral Storage(`/tmp`) allows you to configure the storage upto `10` GB. The default value set to `512` MB.
///
/// ```yaml
/// resources:
///   iamForLambda:
///     type: aws:iam:Role
///     name: iam_for_lambda
///     properties:
///       name: iam_for_lambda
///       assumeRolePolicy: ${assumeRole.json}
///   testLambda:
///     type: aws:lambda:Function
///     name: test_lambda
///     properties:
///       code:
///         fn::FileArchive: lambda_function_payload.zip
///       name: lambda_function_name
///       role: ${iamForLambda.arn}
///       handler: index.test
///       runtime: nodejs18.x
///       ephemeralStorage:
///         size: 10240
/// variables:
///   assumeRole:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - lambda.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ### Lambda File Systems
///
/// Lambda File Systems allow you to connect an Amazon Elastic File System (EFS) file system to a Lambda function to share data across function invocations, access existing data including large files, and save function state.
///
/// ```yaml
/// resources:
///   # A lambda function connected to an EFS file system
///   example:
///     type: aws:lambda:Function
///     properties:
///       fileSystemConfig:
///         arn: ${accessPointForLambda.arn}
///         localMountPath: /mnt/efs
///       vpcConfig:
///         subnetIds:
///           - ${subnetForLambda.id}
///         securityGroupIds:
///           - ${sgForLambda.id}
///     options:
///       dependson:
///         - ${alpha}
///   # EFS file system
///   efsForLambda:
///     type: aws:efs:FileSystem
///     name: efs_for_lambda
///     properties:
///       tags:
///         Name: efs_for_lambda
///   # Mount target connects the file system to the subnet
///   alpha:
///     type: aws:efs:MountTarget
///     properties:
///       fileSystemId: ${efsForLambda.id}
///       subnetId: ${subnetForLambda.id}
///       securityGroups:
///         - ${sgForLambda.id}
///   # EFS access point used by lambda file system
///   accessPointForLambda:
///     type: aws:efs:AccessPoint
///     name: access_point_for_lambda
///     properties:
///       fileSystemId: ${efsForLambda.id}
///       rootDirectory:
///         path: /lambda
///         creationInfo:
///           ownerGid: 1000
///           ownerUid: 1000
///           permissions: '777'
///       posixUser:
///         gid: 1000
///         uid: 1000
/// ```
///
/// ### Lambda retries
///
/// Lambda Functions allow you to configure error handling for asynchronous invocation. The settings that it supports are `Maximum age of event` and `Retry attempts` as stated in [Lambda documentation for Configuring error handling for asynchronous invocation](https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html#invocation-async-errors). To configure these settings, refer to the aws.lambda.FunctionEventInvokeConfig resource.
///
/// ## CloudWatch Logging and Permissions
///
/// For more information about CloudWatch Logs for Lambda, see the [Lambda User Guide](https://docs.aws.amazon.com/lambda/latest/dg/monitoring-functions-logs.html).
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let lambdaLogging = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["logs:CreateLogGroup", "logs:CreateLogStream",
///                     "logs:PutLogEvents",]).effect("Allow")
///                     .resources(vec!["arn:aws:logs:*:*:*",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let example = log_group::create(
///         "example",
///         LogGroupArgs::builder()
///             .name("/aws/lambda/${lambdaFunctionName}")
///             .retention_in_days(14)
///             .build_struct(),
///     );
///     let lambdaLoggingPolicy = policy::create(
///         "lambdaLoggingPolicy",
///         PolicyArgs::builder()
///             .description("IAM policy for logging from a lambda")
///             .name("lambda_logging")
///             .path("/")
///             .policy("${lambdaLogging.json}")
///             .build_struct(),
///     );
///     let lambdaLogs = role_policy_attachment::create(
///         "lambdaLogs",
///         RolePolicyAttachmentArgs::builder()
///             .policy_arn("${lambdaLoggingPolicy.arn}")
///             .role("${iamForLambda.name}")
///             .build_struct(),
///     );
///     let testLambda = function::create(
///         "testLambda",
///         FunctionArgs::builder()
///             .logging_config(
///                 FunctionLoggingConfig::builder().logFormat("Text").build_struct(),
///             )
///             .name("${lambdaFunctionName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Specifying the Deployment Package
///
/// AWS Lambda expects source code to be provided as a deployment package whose structure varies depending on which `runtime` is in use. See [Runtimes](https://docs.aws.amazon.com/lambda/latest/dg/API_CreateFunction.html#SSS-CreateFunction-request-Runtime) for the valid values of `runtime`. The expected structure of the deployment package can be found in [the AWS Lambda documentation for each runtime](https://docs.aws.amazon.com/lambda/latest/dg/deployment-package-v2.html).
///
/// Once you have created your deployment package you can specify it either directly as a local file (using the `filename` argument) or indirectly via Amazon S3 (using the `s3_bucket`, `s3_key` and `s3_object_version` arguments). When providing the deployment package via S3 it may be useful to use the `aws.s3.BucketObjectv2` resource to upload it.
///
/// For larger deployment packages it is recommended by Amazon to upload via S3, since the S3 API has better support for uploading large files efficiently.
///
/// ## Import
///
/// Using `pulumi import`, import Lambda Functions using the `function_name`. For example:
///
/// ```sh
/// $ pulumi import aws:lambda/function:Function test_lambda my_test_lambda_function
/// ```
pub mod function {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionArgs {
        /// Instruction set architecture for your Lambda function. Valid values are `["x86_64"]` and `["arm64"]`. Default is `["x86_64"]`. Removing this attribute, function's architecture stay the same.
        #[builder(into, default)]
        pub architectures: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Path to the function's deployment package within the local filesystem. Exactly one of `filename`, `image_uri`, or `s3_bucket` must be specified.
        #[builder(into, default)]
        pub code: pulumi_wasm_rust::Output<Option<String>>,
        /// To enable code signing for this function, specify the ARN of a code-signing configuration. A code-signing configuration includes a set of signing profiles, which define the trusted publishers for this function.
        #[builder(into, default)]
        pub code_signing_config_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub dead_letter_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionDeadLetterConfig>,
        >,
        /// Description of what your Lambda Function does.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub environment: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionEnvironment>,
        >,
        /// The amount of Ephemeral storage(`/tmp`) to allocate for the Lambda Function in MB. This parameter is used to expand the total amount of Ephemeral storage available, beyond the default amount of `512`MB. Detailed below.
        #[builder(into, default)]
        pub ephemeral_storage: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionEphemeralStorage>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub file_system_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionFileSystemConfig>,
        >,
        /// Function [entrypoint](https://docs.aws.amazon.com/lambda/latest/dg/walkthrough-custom-events-create-test-function.html) in your code.
        #[builder(into, default)]
        pub handler: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub image_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionImageConfig>,
        >,
        /// ECR image URI containing the function's deployment package. Exactly one of `filename`, `image_uri`,  or `s3_bucket` must be specified.
        #[builder(into, default)]
        pub image_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the AWS Key Management Service (KMS) key that is used to encrypt environment variables. If this configuration is not provided when environment variables are in use, AWS Lambda uses a default service key. If this configuration is provided when environment variables are not in use, the AWS Lambda API does not save this configuration and the provider will show a perpetual difference of adding the key. To fix the perpetual difference, remove this configuration.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// List of Lambda Layer Version ARNs (maximum of 5) to attach to your Lambda Function. See [Lambda Layers](https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html)
        #[builder(into, default)]
        pub layers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration block used to specify advanced logging settings. Detailed below.
        #[builder(into, default)]
        pub logging_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionLoggingConfig>,
        >,
        /// Amount of memory in MB your Lambda Function can use at runtime. Defaults to `128`. See [Limits](https://docs.aws.amazon.com/lambda/latest/dg/limits.html)
        #[builder(into, default)]
        pub memory_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Unique name for your Lambda Function.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Lambda deployment package type. Valid values are `Zip` and `Image`. Defaults to `Zip`.
        #[builder(into, default)]
        pub package_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to publish creation/change as new Lambda Function Version. Defaults to `false`.
        #[builder(into, default)]
        pub publish: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to replace the security groups on the function's VPC configuration prior to destruction.
        /// Removing these security group associations prior to function destruction can speed up security group deletion times of AWS's internal cleanup operations.
        /// By default, the security groups will be replaced with the `default` security group in the function's configured VPC.
        /// Set the `replacement_security_group_ids` attribute to use a custom list of security groups for replacement.
        #[builder(into, default)]
        pub replace_security_groups_on_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of security group IDs to assign to the function's VPC configuration prior to destruction.
        /// `replace_security_groups_on_destroy` must be set to `true` to use this attribute.
        #[builder(into, default)]
        pub replacement_security_group_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// Amount of reserved concurrent executions for this lambda function. A value of `0` disables lambda from being triggered and `-1` removes any concurrency limitations. Defaults to Unreserved Concurrency Limits `-1`. See [Managing Concurrency](https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html)
        #[builder(into, default)]
        pub reserved_concurrent_executions: pulumi_wasm_rust::Output<Option<i32>>,
        /// Amazon Resource Name (ARN) of the function's execution role. The role provides the function's identity and access to AWS services and resources.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
        /// Identifier of the function's runtime. See [Runtimes](https://docs.aws.amazon.com/lambda/latest/dg/API_CreateFunction.html#SSS-CreateFunction-request-Runtime) for valid values.
        #[builder(into, default)]
        pub runtime: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 bucket location containing the function's deployment package. This bucket must reside in the same AWS region where you are creating the Lambda function. Exactly one of `filename`, `image_uri`, or `s3_bucket` must be specified. When `s3_bucket` is set, `s3_key` is required.
        #[builder(into, default)]
        pub s3_bucket: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 key of an object containing the function's deployment package. When `s3_bucket` is set, `s3_key` is required.
        #[builder(into, default)]
        pub s3_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Object version containing the function's deployment package. Conflicts with `filename` and `image_uri`.
        #[builder(into, default)]
        pub s3_object_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Set to true if you do not wish the function to be deleted at destroy time, and instead just remove the function from the Pulumi state.
        #[builder(into, default)]
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Snap start settings block. Detailed below.
        #[builder(into, default)]
        pub snap_start: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionSnapStart>,
        >,
        /// Virtual attribute used to trigger replacement when source code changes. Must be set to a base64-encoded SHA256 hash of the package file specified with either `filename` or `s3_key`.
        #[builder(into, default)]
        pub source_code_hash: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Amount of time your Lambda Function has to run in seconds. Defaults to `3`. See [Limits](https://docs.aws.amazon.com/lambda/latest/dg/limits.html).
        #[builder(into, default)]
        pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub tracing_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionTracingConfig>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub vpc_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct FunctionResult {
        /// Instruction set architecture for your Lambda function. Valid values are `["x86_64"]` and `["arm64"]`. Default is `["x86_64"]`. Removing this attribute, function's architecture stay the same.
        pub architectures: pulumi_wasm_rust::Output<Vec<String>>,
        /// Amazon Resource Name (ARN) identifying your Lambda Function.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Path to the function's deployment package within the local filesystem. Exactly one of `filename`, `image_uri`, or `s3_bucket` must be specified.
        pub code: pulumi_wasm_rust::Output<Option<String>>,
        /// Base64-encoded representation of raw SHA-256 sum of the zip file.
        pub code_sha256: pulumi_wasm_rust::Output<String>,
        /// To enable code signing for this function, specify the ARN of a code-signing configuration. A code-signing configuration includes a set of signing profiles, which define the trusted publishers for this function.
        pub code_signing_config_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block. Detailed below.
        pub dead_letter_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionDeadLetterConfig>,
        >,
        /// Description of what your Lambda Function does.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block. Detailed below.
        pub environment: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionEnvironment>,
        >,
        /// The amount of Ephemeral storage(`/tmp`) to allocate for the Lambda Function in MB. This parameter is used to expand the total amount of Ephemeral storage available, beyond the default amount of `512`MB. Detailed below.
        pub ephemeral_storage: pulumi_wasm_rust::Output<
            super::super::types::lambda::FunctionEphemeralStorage,
        >,
        /// Configuration block. Detailed below.
        pub file_system_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionFileSystemConfig>,
        >,
        /// Function [entrypoint](https://docs.aws.amazon.com/lambda/latest/dg/walkthrough-custom-events-create-test-function.html) in your code.
        pub handler: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block. Detailed below.
        pub image_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionImageConfig>,
        >,
        /// ECR image URI containing the function's deployment package. Exactly one of `filename`, `image_uri`,  or `s3_bucket` must be specified.
        pub image_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN to be used for invoking Lambda Function from API Gateway - to be used in `aws.apigateway.Integration`'s `uri`.
        pub invoke_arn: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the AWS Key Management Service (KMS) key that is used to encrypt environment variables. If this configuration is not provided when environment variables are in use, AWS Lambda uses a default service key. If this configuration is provided when environment variables are not in use, the AWS Lambda API does not save this configuration and the provider will show a perpetual difference of adding the key. To fix the perpetual difference, remove this configuration.
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Date this resource was last modified.
        pub last_modified: pulumi_wasm_rust::Output<String>,
        /// List of Lambda Layer Version ARNs (maximum of 5) to attach to your Lambda Function. See [Lambda Layers](https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html)
        pub layers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration block used to specify advanced logging settings. Detailed below.
        pub logging_config: pulumi_wasm_rust::Output<
            super::super::types::lambda::FunctionLoggingConfig,
        >,
        /// Amount of memory in MB your Lambda Function can use at runtime. Defaults to `128`. See [Limits](https://docs.aws.amazon.com/lambda/latest/dg/limits.html)
        pub memory_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Unique name for your Lambda Function.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Lambda deployment package type. Valid values are `Zip` and `Image`. Defaults to `Zip`.
        pub package_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to publish creation/change as new Lambda Function Version. Defaults to `false`.
        pub publish: pulumi_wasm_rust::Output<Option<bool>>,
        /// ARN identifying your Lambda Function Version (if versioning is enabled via `publish = true`).
        pub qualified_arn: pulumi_wasm_rust::Output<String>,
        /// Qualified ARN (ARN with lambda version number) to be used for invoking Lambda Function from API Gateway - to be used in `aws.apigateway.Integration`'s `uri`.
        pub qualified_invoke_arn: pulumi_wasm_rust::Output<String>,
        /// Whether to replace the security groups on the function's VPC configuration prior to destruction.
        /// Removing these security group associations prior to function destruction can speed up security group deletion times of AWS's internal cleanup operations.
        /// By default, the security groups will be replaced with the `default` security group in the function's configured VPC.
        /// Set the `replacement_security_group_ids` attribute to use a custom list of security groups for replacement.
        pub replace_security_groups_on_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of security group IDs to assign to the function's VPC configuration prior to destruction.
        /// `replace_security_groups_on_destroy` must be set to `true` to use this attribute.
        pub replacement_security_group_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// Amount of reserved concurrent executions for this lambda function. A value of `0` disables lambda from being triggered and `-1` removes any concurrency limitations. Defaults to Unreserved Concurrency Limits `-1`. See [Managing Concurrency](https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html)
        pub reserved_concurrent_executions: pulumi_wasm_rust::Output<Option<i32>>,
        /// Amazon Resource Name (ARN) of the function's execution role. The role provides the function's identity and access to AWS services and resources.
        ///
        /// The following arguments are optional:
        pub role: pulumi_wasm_rust::Output<String>,
        /// Identifier of the function's runtime. See [Runtimes](https://docs.aws.amazon.com/lambda/latest/dg/API_CreateFunction.html#SSS-CreateFunction-request-Runtime) for valid values.
        pub runtime: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 bucket location containing the function's deployment package. This bucket must reside in the same AWS region where you are creating the Lambda function. Exactly one of `filename`, `image_uri`, or `s3_bucket` must be specified. When `s3_bucket` is set, `s3_key` is required.
        pub s3_bucket: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 key of an object containing the function's deployment package. When `s3_bucket` is set, `s3_key` is required.
        pub s3_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Object version containing the function's deployment package. Conflicts with `filename` and `image_uri`.
        pub s3_object_version: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the signing job.
        pub signing_job_arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the signing profile version.
        pub signing_profile_version_arn: pulumi_wasm_rust::Output<String>,
        /// Set to true if you do not wish the function to be deleted at destroy time, and instead just remove the function from the Pulumi state.
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Snap start settings block. Detailed below.
        pub snap_start: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionSnapStart>,
        >,
        /// Virtual attribute used to trigger replacement when source code changes. Must be set to a base64-encoded SHA256 hash of the package file specified with either `filename` or `s3_key`.
        pub source_code_hash: pulumi_wasm_rust::Output<String>,
        /// Size in bytes of the function .zip file.
        pub source_code_size: pulumi_wasm_rust::Output<i32>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Amount of time your Lambda Function has to run in seconds. Defaults to `3`. See [Limits](https://docs.aws.amazon.com/lambda/latest/dg/limits.html).
        pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// Configuration block. Detailed below.
        pub tracing_config: pulumi_wasm_rust::Output<
            super::super::types::lambda::FunctionTracingConfig,
        >,
        /// Latest published version of your Lambda Function.
        pub version: pulumi_wasm_rust::Output<String>,
        /// Configuration block. Detailed below.
        pub vpc_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::FunctionVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FunctionArgs) -> FunctionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let architectures_binding = args.architectures.get_inner();
        let code_binding = args.code.get_inner();
        let code_signing_config_arn_binding = args.code_signing_config_arn.get_inner();
        let dead_letter_config_binding = args.dead_letter_config.get_inner();
        let description_binding = args.description.get_inner();
        let environment_binding = args.environment.get_inner();
        let ephemeral_storage_binding = args.ephemeral_storage.get_inner();
        let file_system_config_binding = args.file_system_config.get_inner();
        let handler_binding = args.handler.get_inner();
        let image_config_binding = args.image_config.get_inner();
        let image_uri_binding = args.image_uri.get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_inner();
        let layers_binding = args.layers.get_inner();
        let logging_config_binding = args.logging_config.get_inner();
        let memory_size_binding = args.memory_size.get_inner();
        let name_binding = args.name.get_inner();
        let package_type_binding = args.package_type.get_inner();
        let publish_binding = args.publish.get_inner();
        let replace_security_groups_on_destroy_binding = args
            .replace_security_groups_on_destroy
            .get_inner();
        let replacement_security_group_ids_binding = args
            .replacement_security_group_ids
            .get_inner();
        let reserved_concurrent_executions_binding = args
            .reserved_concurrent_executions
            .get_inner();
        let role_binding = args.role.get_inner();
        let runtime_binding = args.runtime.get_inner();
        let s3_bucket_binding = args.s3_bucket.get_inner();
        let s3_key_binding = args.s3_key.get_inner();
        let s3_object_version_binding = args.s3_object_version.get_inner();
        let skip_destroy_binding = args.skip_destroy.get_inner();
        let snap_start_binding = args.snap_start.get_inner();
        let source_code_hash_binding = args.source_code_hash.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeout_binding = args.timeout.get_inner();
        let tracing_config_binding = args.tracing_config.get_inner();
        let vpc_config_binding = args.vpc_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lambda/function:Function".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "architectures".into(),
                    value: &architectures_binding,
                },
                register_interface::ObjectField {
                    name: "code".into(),
                    value: &code_binding,
                },
                register_interface::ObjectField {
                    name: "codeSigningConfigArn".into(),
                    value: &code_signing_config_arn_binding,
                },
                register_interface::ObjectField {
                    name: "deadLetterConfig".into(),
                    value: &dead_letter_config_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "environment".into(),
                    value: &environment_binding,
                },
                register_interface::ObjectField {
                    name: "ephemeralStorage".into(),
                    value: &ephemeral_storage_binding,
                },
                register_interface::ObjectField {
                    name: "fileSystemConfig".into(),
                    value: &file_system_config_binding,
                },
                register_interface::ObjectField {
                    name: "handler".into(),
                    value: &handler_binding,
                },
                register_interface::ObjectField {
                    name: "imageConfig".into(),
                    value: &image_config_binding,
                },
                register_interface::ObjectField {
                    name: "imageUri".into(),
                    value: &image_uri_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "layers".into(),
                    value: &layers_binding,
                },
                register_interface::ObjectField {
                    name: "loggingConfig".into(),
                    value: &logging_config_binding,
                },
                register_interface::ObjectField {
                    name: "memorySize".into(),
                    value: &memory_size_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "packageType".into(),
                    value: &package_type_binding,
                },
                register_interface::ObjectField {
                    name: "publish".into(),
                    value: &publish_binding,
                },
                register_interface::ObjectField {
                    name: "replaceSecurityGroupsOnDestroy".into(),
                    value: &replace_security_groups_on_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "replacementSecurityGroupIds".into(),
                    value: &replacement_security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "reservedConcurrentExecutions".into(),
                    value: &reserved_concurrent_executions_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "runtime".into(),
                    value: &runtime_binding,
                },
                register_interface::ObjectField {
                    name: "s3Bucket".into(),
                    value: &s3_bucket_binding,
                },
                register_interface::ObjectField {
                    name: "s3Key".into(),
                    value: &s3_key_binding,
                },
                register_interface::ObjectField {
                    name: "s3ObjectVersion".into(),
                    value: &s3_object_version_binding,
                },
                register_interface::ObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "snapStart".into(),
                    value: &snap_start_binding,
                },
                register_interface::ObjectField {
                    name: "sourceCodeHash".into(),
                    value: &source_code_hash_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding,
                },
                register_interface::ObjectField {
                    name: "tracingConfig".into(),
                    value: &tracing_config_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfig".into(),
                    value: &vpc_config_binding,
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
                    name: "code".into(),
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
                    name: "ephemeralStorage".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemConfig".into(),
                },
                register_interface::ResultField {
                    name: "handler".into(),
                },
                register_interface::ResultField {
                    name: "imageConfig".into(),
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
                    name: "loggingConfig".into(),
                },
                register_interface::ResultField {
                    name: "memorySize".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "packageType".into(),
                },
                register_interface::ResultField {
                    name: "publish".into(),
                },
                register_interface::ResultField {
                    name: "qualifiedArn".into(),
                },
                register_interface::ResultField {
                    name: "qualifiedInvokeArn".into(),
                },
                register_interface::ResultField {
                    name: "replaceSecurityGroupsOnDestroy".into(),
                },
                register_interface::ResultField {
                    name: "replacementSecurityGroupIds".into(),
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
                    name: "s3Bucket".into(),
                },
                register_interface::ResultField {
                    name: "s3Key".into(),
                },
                register_interface::ResultField {
                    name: "s3ObjectVersion".into(),
                },
                register_interface::ResultField {
                    name: "signingJobArn".into(),
                },
                register_interface::ResultField {
                    name: "signingProfileVersionArn".into(),
                },
                register_interface::ResultField {
                    name: "skipDestroy".into(),
                },
                register_interface::ResultField {
                    name: "snapStart".into(),
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
                    name: "tagsAll".into(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FunctionResult {
            architectures: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("architectures").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("code").unwrap(),
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
            ephemeral_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ephemeralStorage").unwrap(),
            ),
            file_system_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemConfig").unwrap(),
            ),
            handler: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("handler").unwrap(),
            ),
            image_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageConfig").unwrap(),
            ),
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
            logging_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingConfig").unwrap(),
            ),
            memory_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memorySize").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            package_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageType").unwrap(),
            ),
            publish: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publish").unwrap(),
            ),
            qualified_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("qualifiedArn").unwrap(),
            ),
            qualified_invoke_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("qualifiedInvokeArn").unwrap(),
            ),
            replace_security_groups_on_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replaceSecurityGroupsOnDestroy").unwrap(),
            ),
            replacement_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replacementSecurityGroupIds").unwrap(),
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
            s3_bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3Bucket").unwrap(),
            ),
            s3_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3Key").unwrap(),
            ),
            s3_object_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3ObjectVersion").unwrap(),
            ),
            signing_job_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingJobArn").unwrap(),
            ),
            signing_profile_version_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingProfileVersionArn").unwrap(),
            ),
            skip_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipDestroy").unwrap(),
            ),
            snap_start: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapStart").unwrap(),
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
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
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