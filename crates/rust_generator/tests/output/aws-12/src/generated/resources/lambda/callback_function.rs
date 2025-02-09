/// A CallbackFunction is a special type of `aws.lambda.Function` that can be created out of an actual JavaScript function instance. The Pulumi compiler and runtime work in tandem to extract your function, package it up along with its dependencies, upload the package to AWS Lambda, and configure the resulting AWS Lambda resources automatically.
///
/// The JavaScript function may capture references to other variables in the surrounding code, including other resources and even imported modules. The Pulumi compiler figures out how to serialize the resulting closure as it uploads and configures the AWS Lambda. This works even if you are composing multiple functions together.
///
/// See [Function Serialization](https://www.pulumi.com/docs/concepts/inputs-outputs/function-serialization/) for additional details on this process.
///
/// ### Lambda Function Handler
///
/// You can provide the JavaScript function used for the Lambda Function's Handler either directly by setting the `callback` input property or instead specify the `callbackFactory`, which is a Javascript function that will be called to produce the callback function that is the entrypoint for the AWS Lambda.
/// Using `callbackFactory` is useful when there is expensive initialization work that should only be executed once. The factory-function will be invoked once when the final AWS Lambda module is loaded. It can run whatever code it needs, and will end by returning the actual function that Lambda will call into each time the Lambda is invoked.
///
/// It is recommended to use an async function, otherwise the Lambda execution will run until the `callback` parameter is called and the event loop is empty. See [Define Lambda function handler in Node.js](https://docs.aws.amazon.com/lambda/latest/dg/nodejs-handler.html) for additional details.
///
/// ### Lambda Function Permissions
///
/// If neither `role` nor `policies` is specified, `CallbackFunction` will create an IAM role and automatically use the following managed policies:
/// - `AWSLambda_FullAccess`
/// - `CloudWatchFullAccessV2`
/// - `CloudWatchEventsFullAccess`
/// - `AmazonS3FullAccess`
/// - `AmazonDynamoDBFullAccess`
/// - `AmazonSQSFullAccess`
/// - `AmazonKinesisFullAccess`
/// - `AWSCloudFormationReadOnlyAccess`
/// - `AmazonCognitoPowerUser`
/// - `AWSXrayWriteOnlyAccess`
///
/// ### Customizing Lambda function attributes
///
/// The Lambdas created by `aws.lambda.CallbackFunction` use reasonable defaults for CPU, memory, IAM, logging, and other configuration.
/// Should you need to customize these settings, the `aws.lambda.CallbackFunction` resource offers all of the underlying AWS Lambda settings.
///
/// For example, to increase the RAM available to your function to 256MB:
///
/// ```typescript
/// import * as aws from "@pulumi/aws";
///
/// // Create an AWS Lambda function with 256MB RAM
/// const lambda = new aws.lambda.CallbackFunction("docsHandlerFunc", {
///     callback: async(event: aws.s3.BucketEvent) => {
///         // ...
///     },
///
///     memorySize: 256 /* MB */,
/// });
/// ```
///
/// ### Adding/removing files from a function bundle
///
/// Occasionally you may need to customize the contents of function bundle before uploading it to AWS Lambda --- for example, to remove unneeded Node.js modules or add certain files or folders to the bundle explicitly. The `codePathOptions` property of `CallbackFunction` allows you to do this.
///
/// In this example, a local directory `./config` is added to the function bundle, while an unneeded Node.js module `mime` is removed:
///
/// ```typescript
/// import * as aws from "@pulumi/aws";
/// import * as fs from "fs";
///
/// const lambda = new aws.lambda.CallbackFunction("docsHandlerFunc", {
///     callback: async(event: aws.s3.BucketEvent) => {
///         // ...
///     },
///
///     codePathOptions: {
///
///         // Add local files or folders to the Lambda function bundle.
///         extraIncludePaths: [
///             "./config",
///         ],
///
///         // Remove unneeded Node.js packages from the bundle.
///         extraExcludePackages: [
///             "mime",
///         ],
///     },
/// });
/// ```
///
/// ### Using Lambda layers {#lambda-layers}
///
/// [Lambda layers](https://docs.aws.amazon.com/lambda/latest/dg/chapter-layers.html) allow you to share code, configuration, and other assets across multiple Lambda functions. At runtime, AWS Lambda extracts these files into the function's filesystem, where you can access their contents as though they belonged to the function bundle itself.
///
/// Layers are managed with the [`aws.lambda.LayerVersion`](/registry/packages/aws/api-docs/lambda/layerversion/) resource, and you can attach them to as many `lambda.Function` or `lambda.CallbackFunction` resources as you need using the function's `layers` property. Here, the preceding program is updated to package the `./config` folder as a Lambda layer instead:
///
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as aws from "@pulumi/aws";
/// import * as fs from "fs";
///
/// // Create a Lambda layer containing some shared configuration.
/// const configLayer = new aws.lambda.LayerVersion("config-layer", {
///     layerName: "my-config-layer",
///
///     // Use a Pulumi AssetArchive to zip up the contents of the folder.
///     code: new pulumi.asset.AssetArchive({
///         "config": new pulumi.asset.FileArchive("./config"),
///     }),
/// });
///
/// const lambda = new aws.lambda.CallbackFunction("docsHandlerFunc", {
///     callback: async(event: aws.s3.BucketEvent) => {
///         // ...
///     },
///
///     // Attach the config layer to the function.
///     layers: [
///         configLayer.arn,
///     ],
/// });
/// ```
///
/// Notice the path to the file is now `/opt/config/config.json` --- `/opt` being the path at which AWS Lambda extracts the contents of a layer. The configuration layer is now manageable and deployable independently of the Lambda itself, allowing changes to be applied immediately across all functions that use it.
///
/// #### Using layers for Node.js dependencies
///
/// This same approach can be used for sharing Node.js module dependencies. When you package your dependencies [at the proper path](https://docs.aws.amazon.com/lambda/latest/dg/packaging-layers.html) within the layer zip file, (e.g., `nodejs/node_modules`), AWS Lambda will unpack and expose them automatically to the functions that use them at runtime. This approach can be useful in monorepo scenarios such as the example below, which adds a locally built Node.js module as a layer, then references references the module from within the body of a `CallbackFunction`:
///
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as aws from "@pulumi/aws";
///
/// // Create a layer containing a locally built Node.js module.
/// const utilsLayer = new aws.lambda.LayerVersion("utils-layer", {
///     layerName: "utils",
///     code: new pulumi.asset.AssetArchive({
///
///         // Store the module under nodejs/node_modules to make it available
///         // on the Node.js module path.
///         "nodejs/node_modules/@my-alias/utils": new pulumi.asset.FileArchive("./layers/utils/dist"),
///     }),
/// });
///
/// const lambda =  new aws.lambda.CallbackFunction("docsHandlerFunc", {
///     callback: async (event: aws.s3.BucketEvent) => {
///
///         // Import the module from the layer at runtime.
///         const { sayHello } = await import("@my-alias/utils");
///
///         // Call a function from the utils module.
///         console.log(sayHello());
///     },
///
///     // Attach the utils layer to the function.
///     layers: [
///         utilsLayer.arn,
///     ],
/// });
/// ```
///
/// Notice the example uses the module name `@my-alias/utils`. To make this work, you'll need to add a few lines to your Pulumi project's `tsconfig.json` file to map your chosen module name to the path of the module's TypeScript source code:
///
/// ```javascript
/// {
///     "compilerOptions": {
///         // ...
///         "baseUrl": ".",
///         "paths": {
///             "@my-alias/utils": [
///                 "./layers/utils"
///             ]
///         }
///     },
///     // ...
/// }
/// ```
///
///
/// ## Example Usage
/// ### Basic Lambda Function
///
///
/// ### Lambda Function with expensive initialization work
///
///
/// ### API Gateway Handler Function
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod callback_function {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CallbackFunctionArgs {
        /// Instruction set architecture for your Lambda function. Valid values are `["x86_64"]` and `["arm64"]`. Default is `["x86_64"]`. Removing this attribute, function's architecture stay the same.
        #[builder(into, default)]
        pub architectures: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Javascript function to use as the entrypoint for the AWS Lambda out of. Either callback or callbackFactory must be provided.
        #[builder(into, default)]
        pub callback: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Javascript function that will be called to produce the callback function that is the entrypoint for the AWS Lambda. Either callback or callbackFactory must be provided.
        #[builder(into, default)]
        pub callback_factory: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Options to control which paths/packages should be included or excluded in the zip file containing the code for the AWS lambda.
        #[builder(into, default)]
        pub code_path_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::CodePathOptions>,
        >,
        /// To enable code signing for this function, specify the ARN of a code-signing configuration. A code-signing configuration includes a set of signing profiles, which define the trusted publishers for this function.
        #[builder(into, default)]
        pub code_signing_config_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub dead_letter_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::FunctionDeadLetterConfig>,
        >,
        /// Description of what your Lambda Function does.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub environment: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::FunctionEnvironment>,
        >,
        /// The amount of Ephemeral storage(`/tmp`) to allocate for the Lambda Function in MB. This parameter is used to expand the total amount of Ephemeral storage available, beyond the default amount of `512`MB. Detailed below.
        #[builder(into, default)]
        pub ephemeral_storage: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::FunctionEphemeralStorage>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub file_system_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::FunctionFileSystemConfig>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub image_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::FunctionImageConfig>,
        >,
        /// ECR image URI containing the function's deployment package. Exactly one of `filename`, `image_uri`,  or `s3_bucket` must be specified.
        #[builder(into, default)]
        pub image_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the AWS Key Management Service (KMS) key that is used to encrypt environment variables. If this configuration is not provided when environment variables are in use, AWS Lambda uses a default service key. If this configuration is provided when environment variables are not in use, the AWS Lambda API does not save this configuration and the provider will show a perpetual difference of adding the key. To fix the perpetual difference, remove this configuration.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of Lambda Layer Version ARNs (maximum of 5) to attach to your Lambda Function. See [Lambda Layers](https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html)
        #[builder(into, default)]
        pub layers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Configuration block used to specify advanced logging settings. Detailed below.
        #[builder(into, default)]
        pub logging_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::FunctionLoggingConfig>,
        >,
        /// Amount of memory in MB your Lambda Function can use at runtime. Defaults to `128`. See [Limits](https://docs.aws.amazon.com/lambda/latest/dg/limits.html)
        #[builder(into, default)]
        pub memory_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Unique name for your Lambda Function.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Lambda deployment package type. Valid values are `Zip` and `Image`. Defaults to `Zip`.
        #[builder(into, default)]
        pub package_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of IAM policy ARNs to attach to the Function. Only one of `role` or `policies` can be provided. If neither is provided, the default policies will be used instead.
        #[builder(into, default)]
        pub policies: pulumi_gestalt_rust::InputOrOutput<
            Option<
                pulumi_gestalt_rust::OneOf2<
                    std::collections::HashMap<String, String>,
                    Vec<String>,
                >,
            >,
        >,
        /// Whether to publish creation/change as new Lambda Function Version. Defaults to `false`.
        #[builder(into, default)]
        pub publish: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to replace the security groups on the function's VPC configuration prior to destruction.
        /// Removing these security group associations prior to function destruction can speed up security group deletion times of AWS's internal cleanup operations.
        /// By default, the security groups will be replaced with the `default` security group in the function's configured VPC.
        /// Set the `replacement_security_group_ids` attribute to use a custom list of security groups for replacement.
        #[builder(into, default)]
        pub replace_security_groups_on_destroy: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// List of security group IDs to assign to the function's VPC configuration prior to destruction.
        /// `replace_security_groups_on_destroy` must be set to `true` to use this attribute.
        #[builder(into, default)]
        pub replacement_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Amount of reserved concurrent executions for this lambda function. A value of `0` disables lambda from being triggered and `-1` removes any concurrency limitations. Defaults to Unreserved Concurrency Limits `-1`. See [Managing Concurrency](https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html)
        #[builder(into, default)]
        pub reserved_concurrent_executions: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The execution role for the Lambda Function. The role provides the function's identity and access to AWS services and resources. Only one of `role` or `policies` can be provided. If neither is provided, the default policies will be used instead.
        #[builder(into, default)]
        pub role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Lambda runtime to use. If not provided, will default to `NodeJS20dX`.
        #[builder(into, default)]
        pub runtime: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::Runtime>,
        >,
        /// S3 bucket location containing the function's deployment package. This bucket must reside in the same AWS region where you are creating the Lambda function. Exactly one of `filename`, `image_uri`, or `s3_bucket` must be specified. When `s3_bucket` is set, `s3_key` is required.
        #[builder(into, default)]
        pub s3_bucket: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// S3 key of an object containing the function's deployment package. When `s3_bucket` is set, `s3_key` is required.
        #[builder(into, default)]
        pub s3_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Object version containing the function's deployment package. Conflicts with `filename` and `image_uri`.
        #[builder(into, default)]
        pub s3_object_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set to true if you do not wish the function to be deleted at destroy time, and instead just remove the function from the Pulumi state.
        #[builder(into, default)]
        pub skip_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Snap start settings block. Detailed below.
        #[builder(into, default)]
        pub snap_start: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::FunctionSnapStart>,
        >,
        /// Virtual attribute used to trigger replacement when source code changes. Must be set to a base64-encoded SHA256 hash of the package file specified with either `filename` or `s3_key`.
        #[builder(into, default)]
        pub source_code_hash: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Amount of time your Lambda Function has to run in seconds. Defaults to `3`. See [Limits](https://docs.aws.amazon.com/lambda/latest/dg/limits.html).
        #[builder(into, default)]
        pub timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub tracing_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::FunctionTracingConfig>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub vpc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::FunctionVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct CallbackFunctionResult {
        /// Instruction set architecture for your Lambda function. Valid values are `["x86_64"]` and `["arm64"]`. Default is `["x86_64"]`. Removing this attribute, function's architecture stay the same.
        pub architectures: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Amazon Resource Name (ARN) identifying your Lambda Function.
        pub arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Path to the function's deployment package within the local filesystem. Exactly one of `filename`, `image_uri`, or `s3_bucket` must be specified.
        pub code: pulumi_gestalt_rust::Output<Option<String>>,
        /// Base64-encoded representation of raw SHA-256 sum of the zip file.
        pub code_sha256: pulumi_gestalt_rust::Output<Option<String>>,
        /// To enable code signing for this function, specify the ARN of a code-signing configuration. A code-signing configuration includes a set of signing profiles, which define the trusted publishers for this function.
        pub code_signing_config_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block. Detailed below.
        pub dead_letter_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::lambda::FunctionDeadLetterConfig>,
        >,
        /// Description of what your Lambda Function does.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block. Detailed below.
        pub environment: pulumi_gestalt_rust::Output<
            Option<super::super::types::lambda::FunctionEnvironment>,
        >,
        /// The amount of Ephemeral storage(`/tmp`) to allocate for the Lambda Function in MB. This parameter is used to expand the total amount of Ephemeral storage available, beyond the default amount of `512`MB. Detailed below.
        pub ephemeral_storage: pulumi_gestalt_rust::Output<
            Option<super::super::types::lambda::FunctionEphemeralStorage>,
        >,
        /// Configuration block. Detailed below.
        pub file_system_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::lambda::FunctionFileSystemConfig>,
        >,
        /// Function [entrypoint](https://docs.aws.amazon.com/lambda/latest/dg/walkthrough-custom-events-create-test-function.html) in your code.
        pub handler: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block. Detailed below.
        pub image_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::lambda::FunctionImageConfig>,
        >,
        /// ECR image URI containing the function's deployment package. Exactly one of `filename`, `image_uri`,  or `s3_bucket` must be specified.
        pub image_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN to be used for invoking Lambda Function from API Gateway - to be used in `aws.apigateway.Integration`'s `uri`.
        pub invoke_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the AWS Key Management Service (KMS) key that is used to encrypt environment variables. If this configuration is not provided when environment variables are in use, AWS Lambda uses a default service key. If this configuration is provided when environment variables are not in use, the AWS Lambda API does not save this configuration and the provider will show a perpetual difference of adding the key. To fix the perpetual difference, remove this configuration.
        pub kms_key_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Date this resource was last modified.
        pub last_modified: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of Lambda Layer Version ARNs (maximum of 5) to attach to your Lambda Function. See [Lambda Layers](https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html)
        pub layers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Configuration block used to specify advanced logging settings. Detailed below.
        pub logging_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::lambda::FunctionLoggingConfig>,
        >,
        /// Amount of memory in MB your Lambda Function can use at runtime. Defaults to `128`. See [Limits](https://docs.aws.amazon.com/lambda/latest/dg/limits.html)
        pub memory_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Unique name for your Lambda Function.
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Lambda deployment package type. Valid values are `Zip` and `Image`. Defaults to `Zip`.
        pub package_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to publish creation/change as new Lambda Function Version. Defaults to `false`.
        pub publish: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ARN identifying your Lambda Function Version (if versioning is enabled via `publish = true`).
        pub qualified_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Qualified ARN (ARN with lambda version number) to be used for invoking Lambda Function from API Gateway - to be used in `aws.apigateway.Integration`'s `uri`.
        pub qualified_invoke_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to replace the security groups on the function's VPC configuration prior to destruction.
        /// Removing these security group associations prior to function destruction can speed up security group deletion times of AWS's internal cleanup operations.
        /// By default, the security groups will be replaced with the `default` security group in the function's configured VPC.
        /// Set the `replacement_security_group_ids` attribute to use a custom list of security groups for replacement.
        pub replace_security_groups_on_destroy: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// List of security group IDs to assign to the function's VPC configuration prior to destruction.
        /// `replace_security_groups_on_destroy` must be set to `true` to use this attribute.
        pub replacement_security_group_ids: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// Amount of reserved concurrent executions for this lambda function. A value of `0` disables lambda from being triggered and `-1` removes any concurrency limitations. Defaults to Unreserved Concurrency Limits `-1`. See [Managing Concurrency](https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html)
        pub reserved_concurrent_executions: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Amazon Resource Name (ARN) of the function's execution role. The role provides the function's identity and access to AWS services and resources.
        ///
        /// The following arguments are optional:
        pub role: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IAM role assigned to this Lambda function. Will be undefined if an ARN was provided for the role input property.
        pub role_instance: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of the function's runtime. See [Runtimes](https://docs.aws.amazon.com/lambda/latest/dg/API_CreateFunction.html#SSS-CreateFunction-request-Runtime) for valid values.
        pub runtime: pulumi_gestalt_rust::Output<Option<String>>,
        /// S3 bucket location containing the function's deployment package. This bucket must reside in the same AWS region where you are creating the Lambda function. Exactly one of `filename`, `image_uri`, or `s3_bucket` must be specified. When `s3_bucket` is set, `s3_key` is required.
        pub s3_bucket: pulumi_gestalt_rust::Output<Option<String>>,
        /// S3 key of an object containing the function's deployment package. When `s3_bucket` is set, `s3_key` is required.
        pub s3_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Object version containing the function's deployment package. Conflicts with `filename` and `image_uri`.
        pub s3_object_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the signing job.
        pub signing_job_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the signing profile version.
        pub signing_profile_version_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set to true if you do not wish the function to be deleted at destroy time, and instead just remove the function from the Pulumi state.
        pub skip_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Snap start settings block. Detailed below.
        pub snap_start: pulumi_gestalt_rust::Output<
            Option<super::super::types::lambda::FunctionSnapStart>,
        >,
        /// Virtual attribute used to trigger replacement when source code changes. Must be set to a base64-encoded SHA256 hash of the package file specified with either `filename` or `s3_key`.
        pub source_code_hash: pulumi_gestalt_rust::Output<Option<String>>,
        /// Size in bytes of the function .zip file.
        pub source_code_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Amount of time your Lambda Function has to run in seconds. Defaults to `3`. See [Limits](https://docs.aws.amazon.com/lambda/latest/dg/limits.html).
        pub timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Configuration block. Detailed below.
        pub tracing_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::lambda::FunctionTracingConfig>,
        >,
        /// Latest published version of your Lambda Function.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block. Detailed below.
        pub vpc_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::lambda::FunctionVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CallbackFunctionArgs,
    ) -> CallbackFunctionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let architectures_binding = args.architectures.get_output(context);
        let callback_binding = args.callback.get_output(context);
        let callback_factory_binding = args.callback_factory.get_output(context);
        let code_path_options_binding = args.code_path_options.get_output(context);
        let code_signing_config_arn_binding = args
            .code_signing_config_arn
            .get_output(context);
        let dead_letter_config_binding = args.dead_letter_config.get_output(context);
        let description_binding = args.description.get_output(context);
        let environment_binding = args.environment.get_output(context);
        let ephemeral_storage_binding = args.ephemeral_storage.get_output(context);
        let file_system_config_binding = args.file_system_config.get_output(context);
        let image_config_binding = args.image_config.get_output(context);
        let image_uri_binding = args.image_uri.get_output(context);
        let kms_key_arn_binding = args.kms_key_arn.get_output(context);
        let layers_binding = args.layers.get_output(context);
        let logging_config_binding = args.logging_config.get_output(context);
        let memory_size_binding = args.memory_size.get_output(context);
        let name_binding = args.name.get_output(context);
        let package_type_binding = args.package_type.get_output(context);
        let policies_binding = args.policies.get_output(context);
        let publish_binding = args.publish.get_output(context);
        let replace_security_groups_on_destroy_binding = args
            .replace_security_groups_on_destroy
            .get_output(context);
        let replacement_security_group_ids_binding = args
            .replacement_security_group_ids
            .get_output(context);
        let reserved_concurrent_executions_binding = args
            .reserved_concurrent_executions
            .get_output(context);
        let role_binding = args.role.get_output(context);
        let runtime_binding = args.runtime.get_output(context);
        let s3_bucket_binding = args.s3_bucket.get_output(context);
        let s3_key_binding = args.s3_key.get_output(context);
        let s3_object_version_binding = args.s3_object_version.get_output(context);
        let skip_destroy_binding = args.skip_destroy.get_output(context);
        let snap_start_binding = args.snap_start.get_output(context);
        let source_code_hash_binding = args.source_code_hash.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeout_binding = args.timeout.get_output(context);
        let tracing_config_binding = args.tracing_config.get_output(context);
        let vpc_config_binding = args.vpc_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lambda/callbackFunction:CallbackFunction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "architectures".into(),
                    value: architectures_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "callback".into(),
                    value: callback_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "callbackFactory".into(),
                    value: callback_factory_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "codePathOptions".into(),
                    value: code_path_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "codeSigningConfigArn".into(),
                    value: code_signing_config_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deadLetterConfig".into(),
                    value: dead_letter_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environment".into(),
                    value: environment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ephemeralStorage".into(),
                    value: ephemeral_storage_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileSystemConfig".into(),
                    value: file_system_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageConfig".into(),
                    value: image_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageUri".into(),
                    value: image_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyArn".into(),
                    value: kms_key_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "layers".into(),
                    value: layers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingConfig".into(),
                    value: logging_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memorySize".into(),
                    value: memory_size_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packageType".into(),
                    value: package_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policies".into(),
                    value: policies_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publish".into(),
                    value: publish_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replaceSecurityGroupsOnDestroy".into(),
                    value: replace_security_groups_on_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replacementSecurityGroupIds".into(),
                    value: replacement_security_group_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reservedConcurrentExecutions".into(),
                    value: reserved_concurrent_executions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtime".into(),
                    value: runtime_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Bucket".into(),
                    value: s3_bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Key".into(),
                    value: s3_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3ObjectVersion".into(),
                    value: s3_object_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipDestroy".into(),
                    value: skip_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapStart".into(),
                    value: snap_start_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceCodeHash".into(),
                    value: source_code_hash_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeout".into(),
                    value: timeout_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tracingConfig".into(),
                    value: tracing_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfig".into(),
                    value: vpc_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CallbackFunctionResult {
            architectures: o.get_field("architectures"),
            arn: o.get_field("arn"),
            code: o.get_field("code"),
            code_sha256: o.get_field("codeSha256"),
            code_signing_config_arn: o.get_field("codeSigningConfigArn"),
            dead_letter_config: o.get_field("deadLetterConfig"),
            description: o.get_field("description"),
            environment: o.get_field("environment"),
            ephemeral_storage: o.get_field("ephemeralStorage"),
            file_system_config: o.get_field("fileSystemConfig"),
            handler: o.get_field("handler"),
            image_config: o.get_field("imageConfig"),
            image_uri: o.get_field("imageUri"),
            invoke_arn: o.get_field("invokeArn"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            last_modified: o.get_field("lastModified"),
            layers: o.get_field("layers"),
            logging_config: o.get_field("loggingConfig"),
            memory_size: o.get_field("memorySize"),
            name: o.get_field("name"),
            package_type: o.get_field("packageType"),
            publish: o.get_field("publish"),
            qualified_arn: o.get_field("qualifiedArn"),
            qualified_invoke_arn: o.get_field("qualifiedInvokeArn"),
            replace_security_groups_on_destroy: o
                .get_field("replaceSecurityGroupsOnDestroy"),
            replacement_security_group_ids: o.get_field("replacementSecurityGroupIds"),
            reserved_concurrent_executions: o.get_field("reservedConcurrentExecutions"),
            role: o.get_field("role"),
            role_instance: o.get_field("roleInstance"),
            runtime: o.get_field("runtime"),
            s3_bucket: o.get_field("s3Bucket"),
            s3_key: o.get_field("s3Key"),
            s3_object_version: o.get_field("s3ObjectVersion"),
            signing_job_arn: o.get_field("signingJobArn"),
            signing_profile_version_arn: o.get_field("signingProfileVersionArn"),
            skip_destroy: o.get_field("skipDestroy"),
            snap_start: o.get_field("snapStart"),
            source_code_hash: o.get_field("sourceCodeHash"),
            source_code_size: o.get_field("sourceCodeSize"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeout: o.get_field("timeout"),
            tracing_config: o.get_field("tracingConfig"),
            version: o.get_field("version"),
            vpc_config: o.get_field("vpcConfig"),
        }
    }
}
