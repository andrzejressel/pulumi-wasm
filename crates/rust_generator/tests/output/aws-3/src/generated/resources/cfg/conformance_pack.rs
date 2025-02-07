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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConformancePackArgs {
        /// Amazon S3 bucket where AWS Config stores conformance pack templates. Maximum length of 63.
        #[builder(into, default)]
        pub delivery_s3_bucket: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The prefix for the Amazon S3 bucket. Maximum length of 1024.
        #[builder(into, default)]
        pub delivery_s3_key_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of configuration blocks describing input parameters passed to the conformance pack template. Documented below. When configured, the parameters must also be included in the `template_body` or in the template stored in Amazon S3 if using `template_s3_uri`.
        #[builder(into, default)]
        pub input_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cfg::ConformancePackInputParameter>>,
        >,
        /// The name of the conformance pack. Must begin with a letter and contain from 1 to 256 alphanumeric characters and hyphens.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A string containing full conformance pack template body. Maximum length of 51200. Drift detection is not possible with this argument.
        #[builder(into, default)]
        pub template_body: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Location of file, e.g., `s3://bucketname/prefix`, containing the template body. The uri must point to the conformance pack template that is located in an Amazon S3 bucket in the same region as the conformance pack. Maximum length of 1024. Drift detection is not possible with this argument.
        #[builder(into, default)]
        pub template_s3_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConformancePackResult {
        /// Amazon Resource Name (ARN) of the conformance pack.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Amazon S3 bucket where AWS Config stores conformance pack templates. Maximum length of 63.
        pub delivery_s3_bucket: pulumi_gestalt_rust::Output<Option<String>>,
        /// The prefix for the Amazon S3 bucket. Maximum length of 1024.
        pub delivery_s3_key_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of configuration blocks describing input parameters passed to the conformance pack template. Documented below. When configured, the parameters must also be included in the `template_body` or in the template stored in Amazon S3 if using `template_s3_uri`.
        pub input_parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cfg::ConformancePackInputParameter>>,
        >,
        /// The name of the conformance pack. Must begin with a letter and contain from 1 to 256 alphanumeric characters and hyphens.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A string containing full conformance pack template body. Maximum length of 51200. Drift detection is not possible with this argument.
        pub template_body: pulumi_gestalt_rust::Output<Option<String>>,
        /// Location of file, e.g., `s3://bucketname/prefix`, containing the template body. The uri must point to the conformance pack template that is located in an Amazon S3 bucket in the same region as the conformance pack. Maximum length of 1024. Drift detection is not possible with this argument.
        pub template_s3_uri: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ConformancePackArgs,
    ) -> ConformancePackResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let delivery_s3_bucket_binding = args
            .delivery_s3_bucket
            .get_output(context)
            .get_inner();
        let delivery_s3_key_prefix_binding = args
            .delivery_s3_key_prefix
            .get_output(context)
            .get_inner();
        let input_parameters_binding = args
            .input_parameters
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let template_body_binding = args.template_body.get_output(context).get_inner();
        let template_s3_uri_binding = args
            .template_s3_uri
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cfg/conformancePack:ConformancePack".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConformancePackResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            delivery_s3_bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deliveryS3Bucket"),
            ),
            delivery_s3_key_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deliveryS3KeyPrefix"),
            ),
            input_parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputParameters"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            template_body: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateBody"),
            ),
            template_s3_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateS3Uri"),
            ),
        }
    }
}
