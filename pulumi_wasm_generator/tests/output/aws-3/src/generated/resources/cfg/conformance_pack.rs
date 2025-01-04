/// Manages a Config Conformance Pack. More information about this collection of Config rules and remediation actions can be found in the
/// [Conformance Packs](https://docs.aws.amazon.com/config/latest/developerguide/conformance-packs.html) documentation.
/// Sample Conformance Pack templates may be found in the
/// [AWS Config Rules Repository](https://github.com/awslabs/aws-config-rules/tree/master/aws-config-conformance-packs).
///
/// > **NOTE:** The account must have a Configuration Recorder with proper IAM permissions before the Conformance Pack will
/// successfully create or update. See also the
/// `aws.cfg.Recorder` resource.
///
/// ## Example Usage
///
/// ### Template Body
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = conformance_pack::create(
///         "example",
///         ConformancePackArgs::builder()
///             .input_parameters(
///                 vec![
///                     ConformancePackInputParameter::builder()
///                     .parameterName("AccessKeysRotatedParameterMaxAccessKeyAge")
///                     .parameterValue("90").build_struct(),
///                 ],
///             )
///             .name("example")
///             .template_body(
///                 "Parameters:\n  AccessKeysRotatedParameterMaxAccessKeyAge:\n    Type: String\nResources:\n  IAMPasswordPolicy:\n    Properties:\n      ConfigRuleName: IAMPasswordPolicy\n      Source:\n        Owner: AWS\n        SourceIdentifier: IAM_PASSWORD_POLICY\n    Type: AWS::Config::ConfigRule\n",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Template S3 URI
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = conformance_pack::create(
///         "example",
///         ConformancePackArgs::builder()
///             .name("example")
///             .template_s_3_uri(
///                 "s3://${exampleBucketV2.bucket}/${exampleBucketObjectv2.key}",
///             )
///             .build_struct(),
///     );
///     let exampleBucketObjectv2 = bucket_objectv_2::create(
///         "exampleBucketObjectv2",
///         BucketObjectv2Args::builder()
///             .bucket("${exampleBucketV2.id}")
///             .content(
///                 "Resources:\n  IAMPasswordPolicy:\n    Properties:\n      ConfigRuleName: IAMPasswordPolicy\n      Source:\n        Owner: AWS\n        SourceIdentifier: IAM_PASSWORD_POLICY\n    Type: AWS::Config::ConfigRule",
///             )
///             .key("example-key")
///             .build_struct(),
///     );
///     let exampleBucketV2 = bucket_v_2::create(
///         "exampleBucketV2",
///         BucketV2Args::builder().bucket("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Config Conformance Packs using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/conformancePack:ConformancePack example example
/// ```
pub mod conformance_pack {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConformancePackArgs {
        /// Amazon S3 bucket where AWS Config stores conformance pack templates. Maximum length of 63.
        #[builder(into, default)]
        pub delivery_s3_bucket: pulumi_wasm_rust::Output<Option<String>>,
        /// The prefix for the Amazon S3 bucket. Maximum length of 1024.
        #[builder(into, default)]
        pub delivery_s3_key_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of configuration blocks describing input parameters passed to the conformance pack template. Documented below. When configured, the parameters must also be included in the `template_body` or in the template stored in Amazon S3 if using `template_s3_uri`.
        #[builder(into, default)]
        pub input_parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cfg::ConformancePackInputParameter>>,
        >,
        /// The name of the conformance pack. Must begin with a letter and contain from 1 to 256 alphanumeric characters and hyphens.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A string containing full conformance pack template body. Maximum length of 51200. Drift detection is not possible with this argument.
        #[builder(into, default)]
        pub template_body: pulumi_wasm_rust::Output<Option<String>>,
        /// Location of file, e.g., `s3://bucketname/prefix`, containing the template body. The uri must point to the conformance pack template that is located in an Amazon S3 bucket in the same region as the conformance pack. Maximum length of 1024. Drift detection is not possible with this argument.
        #[builder(into, default)]
        pub template_s3_uri: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConformancePackResult {
        /// Amazon Resource Name (ARN) of the conformance pack.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Amazon S3 bucket where AWS Config stores conformance pack templates. Maximum length of 63.
        pub delivery_s3_bucket: pulumi_wasm_rust::Output<Option<String>>,
        /// The prefix for the Amazon S3 bucket. Maximum length of 1024.
        pub delivery_s3_key_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of configuration blocks describing input parameters passed to the conformance pack template. Documented below. When configured, the parameters must also be included in the `template_body` or in the template stored in Amazon S3 if using `template_s3_uri`.
        pub input_parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cfg::ConformancePackInputParameter>>,
        >,
        /// The name of the conformance pack. Must begin with a letter and contain from 1 to 256 alphanumeric characters and hyphens.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A string containing full conformance pack template body. Maximum length of 51200. Drift detection is not possible with this argument.
        pub template_body: pulumi_wasm_rust::Output<Option<String>>,
        /// Location of file, e.g., `s3://bucketname/prefix`, containing the template body. The uri must point to the conformance pack template that is located in an Amazon S3 bucket in the same region as the conformance pack. Maximum length of 1024. Drift detection is not possible with this argument.
        pub template_s3_uri: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConformancePackArgs) -> ConformancePackResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let delivery_s3_bucket_binding = args.delivery_s3_bucket.get_inner();
        let delivery_s3_key_prefix_binding = args.delivery_s3_key_prefix.get_inner();
        let input_parameters_binding = args.input_parameters.get_inner();
        let name_binding = args.name.get_inner();
        let template_body_binding = args.template_body.get_inner();
        let template_s3_uri_binding = args.template_s3_uri.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cfg/conformancePack:ConformancePack".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deliveryS3Bucket".into(),
                    value: &delivery_s3_bucket_binding,
                },
                register_interface::ObjectField {
                    name: "deliveryS3KeyPrefix".into(),
                    value: &delivery_s3_key_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "inputParameters".into(),
                    value: &input_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "templateBody".into(),
                    value: &template_body_binding,
                },
                register_interface::ObjectField {
                    name: "templateS3Uri".into(),
                    value: &template_s3_uri_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "deliveryS3Bucket".into(),
                },
                register_interface::ResultField {
                    name: "deliveryS3KeyPrefix".into(),
                },
                register_interface::ResultField {
                    name: "inputParameters".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "templateBody".into(),
                },
                register_interface::ResultField {
                    name: "templateS3Uri".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConformancePackResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            delivery_s3_bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryS3Bucket").unwrap(),
            ),
            delivery_s3_key_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryS3KeyPrefix").unwrap(),
            ),
            input_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputParameters").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            template_body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateBody").unwrap(),
            ),
            template_s3_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateS3Uri").unwrap(),
            ),
        }
    }
}
