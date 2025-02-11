/// Provides a Lambda Layer Version resource. Lambda Layers allow you to reuse shared bits of code across multiple lambda functions.
///
/// For information about Lambda Layers and how to use them, see [AWS Lambda Layers](https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html).
///
/// > **NOTE:** Setting `skip_destroy` to `true` means that the AWS Provider will _not_ destroy any layer version, even when running destroy. Layer versions are thus intentional dangling resources that are _not_ managed by the provider and may incur extra expense in your AWS account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   lambdaLayer:
///     type: aws:lambda:LayerVersion
///     name: lambda_layer
///     properties:
///       code:
///         fn::FileArchive: lambda_layer_payload.zip
///       layerName: lambda_layer_name
///       compatibleRuntimes:
///         - nodejs20.x
/// ```
///
/// ## Specifying the Deployment Package
///
/// AWS Lambda Layers expect source code to be provided as a deployment package whose structure varies depending on which `compatible_runtimes` this layer specifies.
/// See [Runtimes](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-CompatibleRuntimes) for the valid values of `compatible_runtimes`.
///
/// Once you have created your deployment package you can specify it either directly as a local file (using the `filename` argument) or
/// indirectly via Amazon S3 (using the `s3_bucket`, `s3_key` and `s3_object_version` arguments). When providing the deployment
/// package via S3 it may be useful to use the `aws.s3.BucketObjectv2` resource to upload it.
///
/// For larger deployment packages it is recommended by Amazon to upload via S3, since the S3 API has better support for uploading large files efficiently.
///
/// ## Import
///
/// Using `pulumi import`, import Lambda Layers using `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:lambda/layerVersion:LayerVersion test_layer arn:aws:lambda:_REGION_:_ACCOUNT_ID_:layer:_LAYER_NAME_:_LAYER_VERSION_
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod layer_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LayerVersionArgs {
        /// Path to the function's deployment package within the local filesystem. If defined, The `s3_`-prefixed options cannot be used.
        #[builder(into, default)]
        pub code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of [Architectures](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-CompatibleArchitectures) this layer is compatible with. Currently `x86_64` and `arm64` can be specified.
        #[builder(into, default)]
        pub compatible_architectures: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// List of [Runtimes](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-CompatibleRuntimes) this layer is compatible with. Up to 15 runtimes can be specified.
        #[builder(into, default)]
        pub compatible_runtimes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Description of what your Lambda Layer does.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name for your Lambda Layer
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub layer_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// License info for your Lambda Layer. See [License Info](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-LicenseInfo).
        #[builder(into, default)]
        pub license_info: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// S3 bucket location containing the function's deployment package. Conflicts with `filename`. This bucket must reside in the same AWS region where you are creating the Lambda function.
        #[builder(into, default)]
        pub s3_bucket: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// S3 key of an object containing the function's deployment package. Conflicts with `filename`.
        #[builder(into, default)]
        pub s3_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Object version containing the function's deployment package. Conflicts with `filename`.
        #[builder(into, default)]
        pub s3_object_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to retain the old version of a previously deployed Lambda Layer. Default is `false`. When this is not set to `true`, changing any of `compatible_architectures`, `compatible_runtimes`, `description`, `filename`, `layer_name`, `license_info`, `s3_bucket`, `s3_key`, `s3_object_version`, or `source_code_hash` forces deletion of the existing layer version and creation of a new layer version.
        #[builder(into, default)]
        pub skip_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Virtual attribute used to trigger replacement when source code changes. Must be set to a base64-encoded SHA256 hash of the package file specified with either `filename` or `s3_key`.
        #[builder(into, default)]
        pub source_code_hash: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LayerVersionResult {
        /// ARN of the Lambda Layer with version.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Path to the function's deployment package within the local filesystem. If defined, The `s3_`-prefixed options cannot be used.
        pub code: pulumi_gestalt_rust::Output<Option<String>>,
        /// Base64-encoded representation of raw SHA-256 sum of the zip file.
        pub code_sha256: pulumi_gestalt_rust::Output<String>,
        /// List of [Architectures](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-CompatibleArchitectures) this layer is compatible with. Currently `x86_64` and `arm64` can be specified.
        pub compatible_architectures: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of [Runtimes](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-CompatibleRuntimes) this layer is compatible with. Up to 15 runtimes can be specified.
        pub compatible_runtimes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Date this resource was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Description of what your Lambda Layer does.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the Lambda Layer without version.
        pub layer_arn: pulumi_gestalt_rust::Output<String>,
        /// Unique name for your Lambda Layer
        ///
        /// The following arguments are optional:
        pub layer_name: pulumi_gestalt_rust::Output<String>,
        /// License info for your Lambda Layer. See [License Info](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-LicenseInfo).
        pub license_info: pulumi_gestalt_rust::Output<Option<String>>,
        /// S3 bucket location containing the function's deployment package. Conflicts with `filename`. This bucket must reside in the same AWS region where you are creating the Lambda function.
        pub s3_bucket: pulumi_gestalt_rust::Output<Option<String>>,
        /// S3 key of an object containing the function's deployment package. Conflicts with `filename`.
        pub s3_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Object version containing the function's deployment package. Conflicts with `filename`.
        pub s3_object_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of a signing job.
        pub signing_job_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN for a signing profile version.
        pub signing_profile_version_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether to retain the old version of a previously deployed Lambda Layer. Default is `false`. When this is not set to `true`, changing any of `compatible_architectures`, `compatible_runtimes`, `description`, `filename`, `layer_name`, `license_info`, `s3_bucket`, `s3_key`, `s3_object_version`, or `source_code_hash` forces deletion of the existing layer version and creation of a new layer version.
        pub skip_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Virtual attribute used to trigger replacement when source code changes. Must be set to a base64-encoded SHA256 hash of the package file specified with either `filename` or `s3_key`.
        pub source_code_hash: pulumi_gestalt_rust::Output<String>,
        /// Size in bytes of the function .zip file.
        pub source_code_size: pulumi_gestalt_rust::Output<i32>,
        /// Lambda Layer version.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LayerVersionArgs,
    ) -> LayerVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let code_binding = args.code.get_output(context);
        let compatible_architectures_binding = args
            .compatible_architectures
            .get_output(context);
        let compatible_runtimes_binding = args.compatible_runtimes.get_output(context);
        let description_binding = args.description.get_output(context);
        let layer_name_binding = args.layer_name.get_output(context);
        let license_info_binding = args.license_info.get_output(context);
        let s3_bucket_binding = args.s3_bucket.get_output(context);
        let s3_key_binding = args.s3_key.get_output(context);
        let s3_object_version_binding = args.s3_object_version.get_output(context);
        let skip_destroy_binding = args.skip_destroy.get_output(context);
        let source_code_hash_binding = args.source_code_hash.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lambda/layerVersion:LayerVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "code".into(),
                    value: &code_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "compatibleArchitectures".into(),
                    value: &compatible_architectures_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "compatibleRuntimes".into(),
                    value: &compatible_runtimes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "layerName".into(),
                    value: &layer_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseInfo".into(),
                    value: &license_info_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Bucket".into(),
                    value: &s3_bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Key".into(),
                    value: &s3_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3ObjectVersion".into(),
                    value: &s3_object_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceCodeHash".into(),
                    value: &source_code_hash_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LayerVersionResult {
            arn: o.get_field("arn"),
            code: o.get_field("code"),
            code_sha256: o.get_field("codeSha256"),
            compatible_architectures: o.get_field("compatibleArchitectures"),
            compatible_runtimes: o.get_field("compatibleRuntimes"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            layer_arn: o.get_field("layerArn"),
            layer_name: o.get_field("layerName"),
            license_info: o.get_field("licenseInfo"),
            s3_bucket: o.get_field("s3Bucket"),
            s3_key: o.get_field("s3Key"),
            s3_object_version: o.get_field("s3ObjectVersion"),
            signing_job_arn: o.get_field("signingJobArn"),
            signing_profile_version_arn: o.get_field("signingProfileVersionArn"),
            skip_destroy: o.get_field("skipDestroy"),
            source_code_hash: o.get_field("sourceCodeHash"),
            source_code_size: o.get_field("sourceCodeSize"),
            version: o.get_field("version"),
        }
    }
}
