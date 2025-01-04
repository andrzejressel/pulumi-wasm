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
pub mod layer_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LayerVersionArgs {
        /// Path to the function's deployment package within the local filesystem. If defined, The `s3_`-prefixed options cannot be used.
        #[builder(into, default)]
        pub code: pulumi_wasm_rust::Output<Option<String>>,
        /// List of [Architectures](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-CompatibleArchitectures) this layer is compatible with. Currently `x86_64` and `arm64` can be specified.
        #[builder(into, default)]
        pub compatible_architectures: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of [Runtimes](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-CompatibleRuntimes) this layer is compatible with. Up to 15 runtimes can be specified.
        #[builder(into, default)]
        pub compatible_runtimes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Description of what your Lambda Layer does.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique name for your Lambda Layer
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub layer_name: pulumi_wasm_rust::Output<String>,
        /// License info for your Lambda Layer. See [License Info](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-LicenseInfo).
        #[builder(into, default)]
        pub license_info: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 bucket location containing the function's deployment package. Conflicts with `filename`. This bucket must reside in the same AWS region where you are creating the Lambda function.
        #[builder(into, default)]
        pub s3_bucket: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 key of an object containing the function's deployment package. Conflicts with `filename`.
        #[builder(into, default)]
        pub s3_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Object version containing the function's deployment package. Conflicts with `filename`.
        #[builder(into, default)]
        pub s3_object_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to retain the old version of a previously deployed Lambda Layer. Default is `false`. When this is not set to `true`, changing any of `compatible_architectures`, `compatible_runtimes`, `description`, `filename`, `layer_name`, `license_info`, `s3_bucket`, `s3_key`, `s3_object_version`, or `source_code_hash` forces deletion of the existing layer version and creation of a new layer version.
        #[builder(into, default)]
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Virtual attribute used to trigger replacement when source code changes. Must be set to a base64-encoded SHA256 hash of the package file specified with either `filename` or `s3_key`.
        #[builder(into, default)]
        pub source_code_hash: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LayerVersionResult {
        /// ARN of the Lambda Layer with version.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Path to the function's deployment package within the local filesystem. If defined, The `s3_`-prefixed options cannot be used.
        pub code: pulumi_wasm_rust::Output<Option<String>>,
        /// Base64-encoded representation of raw SHA-256 sum of the zip file.
        pub code_sha256: pulumi_wasm_rust::Output<String>,
        /// List of [Architectures](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-CompatibleArchitectures) this layer is compatible with. Currently `x86_64` and `arm64` can be specified.
        pub compatible_architectures: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of [Runtimes](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-CompatibleRuntimes) this layer is compatible with. Up to 15 runtimes can be specified.
        pub compatible_runtimes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Date this resource was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Description of what your Lambda Layer does.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the Lambda Layer without version.
        pub layer_arn: pulumi_wasm_rust::Output<String>,
        /// Unique name for your Lambda Layer
        ///
        /// The following arguments are optional:
        pub layer_name: pulumi_wasm_rust::Output<String>,
        /// License info for your Lambda Layer. See [License Info](https://docs.aws.amazon.com/lambda/latest/dg/API_PublishLayerVersion.html#SSS-PublishLayerVersion-request-LicenseInfo).
        pub license_info: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 bucket location containing the function's deployment package. Conflicts with `filename`. This bucket must reside in the same AWS region where you are creating the Lambda function.
        pub s3_bucket: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 key of an object containing the function's deployment package. Conflicts with `filename`.
        pub s3_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Object version containing the function's deployment package. Conflicts with `filename`.
        pub s3_object_version: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of a signing job.
        pub signing_job_arn: pulumi_wasm_rust::Output<String>,
        /// ARN for a signing profile version.
        pub signing_profile_version_arn: pulumi_wasm_rust::Output<String>,
        /// Whether to retain the old version of a previously deployed Lambda Layer. Default is `false`. When this is not set to `true`, changing any of `compatible_architectures`, `compatible_runtimes`, `description`, `filename`, `layer_name`, `license_info`, `s3_bucket`, `s3_key`, `s3_object_version`, or `source_code_hash` forces deletion of the existing layer version and creation of a new layer version.
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Virtual attribute used to trigger replacement when source code changes. Must be set to a base64-encoded SHA256 hash of the package file specified with either `filename` or `s3_key`.
        pub source_code_hash: pulumi_wasm_rust::Output<String>,
        /// Size in bytes of the function .zip file.
        pub source_code_size: pulumi_wasm_rust::Output<i32>,
        /// Lambda Layer version.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LayerVersionArgs) -> LayerVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let code_binding = args.code.get_inner();
        let compatible_architectures_binding = args.compatible_architectures.get_inner();
        let compatible_runtimes_binding = args.compatible_runtimes.get_inner();
        let description_binding = args.description.get_inner();
        let layer_name_binding = args.layer_name.get_inner();
        let license_info_binding = args.license_info.get_inner();
        let s3_bucket_binding = args.s3_bucket.get_inner();
        let s3_key_binding = args.s3_key.get_inner();
        let s3_object_version_binding = args.s3_object_version.get_inner();
        let skip_destroy_binding = args.skip_destroy.get_inner();
        let source_code_hash_binding = args.source_code_hash.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lambda/layerVersion:LayerVersion".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "code".into(),
                    value: &code_binding,
                },
                register_interface::ObjectField {
                    name: "compatibleArchitectures".into(),
                    value: &compatible_architectures_binding,
                },
                register_interface::ObjectField {
                    name: "compatibleRuntimes".into(),
                    value: &compatible_runtimes_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "layerName".into(),
                    value: &layer_name_binding,
                },
                register_interface::ObjectField {
                    name: "licenseInfo".into(),
                    value: &license_info_binding,
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
                    name: "sourceCodeHash".into(),
                    value: &source_code_hash_binding,
                },
            ]),
            results: Vec::from([
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
                    name: "compatibleArchitectures".into(),
                },
                register_interface::ResultField {
                    name: "compatibleRuntimes".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "layerArn".into(),
                },
                register_interface::ResultField {
                    name: "layerName".into(),
                },
                register_interface::ResultField {
                    name: "licenseInfo".into(),
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
                    name: "sourceCodeHash".into(),
                },
                register_interface::ResultField {
                    name: "sourceCodeSize".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LayerVersionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("code").unwrap(),
            ),
            code_sha256: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("codeSha256").unwrap(),
            ),
            compatible_architectures: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compatibleArchitectures").unwrap(),
            ),
            compatible_runtimes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compatibleRuntimes").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            layer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("layerArn").unwrap(),
            ),
            layer_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("layerName").unwrap(),
            ),
            license_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseInfo").unwrap(),
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
            source_code_hash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceCodeHash").unwrap(),
            ),
            source_code_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceCodeSize").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
