/// Resource for managing an AWS Comprehend Document Classifier.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let documents = bucket_objectv_2::create(
///         "documents",
///         BucketObjectv2Args::builder().build_struct(),
///     );
///     let entities = bucket_objectv_2::create(
///         "entities",
///         BucketObjectv2Args::builder().build_struct(),
///     );
///     let example = document_classifier::create(
///         "example",
///         DocumentClassifierArgs::builder()
///             .data_access_role_arn("${exampleAwsIamRole.arn}")
///             .input_data_config(
///                 DocumentClassifierInputDataConfig::builder()
///                     .s3Uri("s3://${test.bucket}/${documents.id}")
///                     .build_struct(),
///             )
///             .language_code("en")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Comprehend Document Classifier using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:comprehend/documentClassifier:DocumentClassifier example arn:aws:comprehend:us-west-2:123456789012:document_classifier/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod document_classifier {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DocumentClassifierArgs {
        /// The ARN for an IAM Role which allows Comprehend to read the training and testing data.
        #[builder(into)]
        pub data_access_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration for the training and testing data.
        /// See the `input_data_config` Configuration Block section below.
        #[builder(into)]
        pub input_data_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::comprehend::DocumentClassifierInputDataConfig,
        >,
        /// Two-letter language code for the language.
        /// One of `en`, `es`, `fr`, `it`, `de`, or `pt`.
        #[builder(into)]
        pub language_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The document classification mode.
        /// One of `MULTI_CLASS` or `MULTI_LABEL`.
        /// `MULTI_CLASS` is also known as "Single Label" in the AWS Console.
        #[builder(into, default)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// KMS Key used to encrypt trained Document Classifiers.
        /// Can be a KMS Key ID or a KMS Key ARN.
        #[builder(into, default)]
        pub model_kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name for the Document Classifier.
        /// Has a maximum length of 63 characters.
        /// Can contain upper- and lower-case letters, numbers, and hypen (`-`).
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for the output results of training.
        /// See the `output_data_config` Configuration Block section below.
        #[builder(into, default)]
        pub output_data_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::comprehend::DocumentClassifierOutputDataConfig>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` Configuration Block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name for the version of the Document Classifier.
        /// Each version must have a unique name within the Document Classifier.
        /// If omitted, the provider will assign a random, unique version name.
        /// If explicitly set to `""`, no version name will be set.
        /// Has a maximum length of 63 characters.
        /// Can contain upper- and lower-case letters, numbers, and hypen (`-`).
        /// Conflicts with `version_name_prefix`.
        #[builder(into, default)]
        pub version_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique version name beginning with the specified prefix.
        /// Has a maximum length of 37 characters.
        /// Can contain upper- and lower-case letters, numbers, and hypen (`-`).
        /// Conflicts with `version_name`.
        #[builder(into, default)]
        pub version_name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// KMS Key used to encrypt storage volumes during job processing.
        /// Can be a KMS Key ID or a KMS Key ARN.
        #[builder(into, default)]
        pub volume_kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration parameters for VPC to contain Document Classifier resources.
        /// See the `vpc_config` Configuration Block section below.
        #[builder(into, default)]
        pub vpc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::comprehend::DocumentClassifierVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct DocumentClassifierResult {
        /// ARN of the Document Classifier version.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN for an IAM Role which allows Comprehend to read the training and testing data.
        pub data_access_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration for the training and testing data.
        /// See the `input_data_config` Configuration Block section below.
        pub input_data_config: pulumi_gestalt_rust::Output<
            super::super::types::comprehend::DocumentClassifierInputDataConfig,
        >,
        /// Two-letter language code for the language.
        /// One of `en`, `es`, `fr`, `it`, `de`, or `pt`.
        pub language_code: pulumi_gestalt_rust::Output<String>,
        /// The document classification mode.
        /// One of `MULTI_CLASS` or `MULTI_LABEL`.
        /// `MULTI_CLASS` is also known as "Single Label" in the AWS Console.
        pub mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// KMS Key used to encrypt trained Document Classifiers.
        /// Can be a KMS Key ID or a KMS Key ARN.
        pub model_kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name for the Document Classifier.
        /// Has a maximum length of 63 characters.
        /// Can contain upper- and lower-case letters, numbers, and hypen (`-`).
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration for the output results of training.
        /// See the `output_data_config` Configuration Block section below.
        pub output_data_config: pulumi_gestalt_rust::Output<
            super::super::types::comprehend::DocumentClassifierOutputDataConfig,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` Configuration Block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name for the version of the Document Classifier.
        /// Each version must have a unique name within the Document Classifier.
        /// If omitted, the provider will assign a random, unique version name.
        /// If explicitly set to `""`, no version name will be set.
        /// Has a maximum length of 63 characters.
        /// Can contain upper- and lower-case letters, numbers, and hypen (`-`).
        /// Conflicts with `version_name_prefix`.
        pub version_name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique version name beginning with the specified prefix.
        /// Has a maximum length of 37 characters.
        /// Can contain upper- and lower-case letters, numbers, and hypen (`-`).
        /// Conflicts with `version_name`.
        pub version_name_prefix: pulumi_gestalt_rust::Output<String>,
        /// KMS Key used to encrypt storage volumes during job processing.
        /// Can be a KMS Key ID or a KMS Key ARN.
        pub volume_kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration parameters for VPC to contain Document Classifier resources.
        /// See the `vpc_config` Configuration Block section below.
        pub vpc_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::comprehend::DocumentClassifierVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DocumentClassifierArgs,
    ) -> DocumentClassifierResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let data_access_role_arn_binding_1 = args
            .data_access_role_arn
            .get_output(context);
        let data_access_role_arn_binding = data_access_role_arn_binding_1.get_inner();
        let input_data_config_binding_1 = args.input_data_config.get_output(context);
        let input_data_config_binding = input_data_config_binding_1.get_inner();
        let language_code_binding_1 = args.language_code.get_output(context);
        let language_code_binding = language_code_binding_1.get_inner();
        let mode_binding_1 = args.mode.get_output(context);
        let mode_binding = mode_binding_1.get_inner();
        let model_kms_key_id_binding_1 = args.model_kms_key_id.get_output(context);
        let model_kms_key_id_binding = model_kms_key_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let output_data_config_binding_1 = args.output_data_config.get_output(context);
        let output_data_config_binding = output_data_config_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let version_name_binding_1 = args.version_name.get_output(context);
        let version_name_binding = version_name_binding_1.get_inner();
        let version_name_prefix_binding_1 = args.version_name_prefix.get_output(context);
        let version_name_prefix_binding = version_name_prefix_binding_1.get_inner();
        let volume_kms_key_id_binding_1 = args.volume_kms_key_id.get_output(context);
        let volume_kms_key_id_binding = volume_kms_key_id_binding_1.get_inner();
        let vpc_config_binding_1 = args.vpc_config.get_output(context);
        let vpc_config_binding = vpc_config_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:comprehend/documentClassifier:DocumentClassifier".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataAccessRoleArn".into(),
                    value: &data_access_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "inputDataConfig".into(),
                    value: &input_data_config_binding,
                },
                register_interface::ObjectField {
                    name: "languageCode".into(),
                    value: &language_code_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "modelKmsKeyId".into(),
                    value: &model_kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "outputDataConfig".into(),
                    value: &output_data_config_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "versionName".into(),
                    value: &version_name_binding,
                },
                register_interface::ObjectField {
                    name: "versionNamePrefix".into(),
                    value: &version_name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "volumeKmsKeyId".into(),
                    value: &volume_kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfig".into(),
                    value: &vpc_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DocumentClassifierResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            data_access_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataAccessRoleArn"),
            ),
            input_data_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputDataConfig"),
            ),
            language_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("languageCode"),
            ),
            mode: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mode")),
            model_kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modelKmsKeyId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            output_data_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outputDataConfig"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            version_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionName"),
            ),
            version_name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionNamePrefix"),
            ),
            volume_kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeKmsKeyId"),
            ),
            vpc_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcConfig"),
            ),
        }
    }
}
