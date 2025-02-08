/// Resource for managing an AWS Comprehend Entity Recognizer.
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
///     let example = entity_recognizer::create(
///         "example",
///         EntityRecognizerArgs::builder()
///             .data_access_role_arn("${exampleAwsIamRole.arn}")
///             .input_data_config(
///                 EntityRecognizerInputDataConfig::builder()
///                     .documents(
///                         EntityRecognizerInputDataConfigDocuments::builder()
///                             .s3Uri("s3://${documentsAwsS3Bucket.bucket}/${documents.id}")
///                             .build_struct(),
///                     )
///                     .entityList(
///                         EntityRecognizerInputDataConfigEntityList::builder()
///                             .s3Uri("s3://${entitiesAwsS3Bucket.bucket}/${entities.id}")
///                             .build_struct(),
///                     )
///                     .entityTypes(
///                         vec![
///                             EntityRecognizerInputDataConfigEntityType::builder(). type
///                             ("ENTITY_1").build_struct(),
///                             EntityRecognizerInputDataConfigEntityType::builder(). type
///                             ("ENTITY_2").build_struct(),
///                         ],
///                     )
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
/// Using `pulumi import`, import Comprehend Entity Recognizer using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:comprehend/entityRecognizer:EntityRecognizer example arn:aws:comprehend:us-west-2:123456789012:entity-recognizer/example
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod entity_recognizer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EntityRecognizerArgs {
        /// The ARN for an IAM Role which allows Comprehend to read the training and testing data.
        #[builder(into)]
        pub data_access_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration for the training and testing data.
        /// See the `input_data_config` Configuration Block section below.
        #[builder(into)]
        pub input_data_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::comprehend::EntityRecognizerInputDataConfig,
        >,
        /// Two-letter language code for the language.
        /// One of `en`, `es`, `fr`, `it`, `de`, or `pt`.
        #[builder(into)]
        pub language_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID or ARN of a KMS Key used to encrypt trained Entity Recognizers.
        #[builder(into, default)]
        pub model_kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name for the Entity Recognizer.
        /// Has a maximum length of 63 characters.
        /// Can contain upper- and lower-case letters, numbers, and hypen (`-`).
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` Configuration Block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name for the version of the Entity Recognizer.
        /// Each version must have a unique name within the Entity Recognizer.
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
        /// ID or ARN of a KMS Key used to encrypt storage volumes during job processing.
        #[builder(into, default)]
        pub volume_kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration parameters for VPC to contain Entity Recognizer resources.
        /// See the `vpc_config` Configuration Block section below.
        #[builder(into, default)]
        pub vpc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::comprehend::EntityRecognizerVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct EntityRecognizerResult {
        /// ARN of the Entity Recognizer version.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN for an IAM Role which allows Comprehend to read the training and testing data.
        pub data_access_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration for the training and testing data.
        /// See the `input_data_config` Configuration Block section below.
        pub input_data_config: pulumi_gestalt_rust::Output<
            super::super::types::comprehend::EntityRecognizerInputDataConfig,
        >,
        /// Two-letter language code for the language.
        /// One of `en`, `es`, `fr`, `it`, `de`, or `pt`.
        pub language_code: pulumi_gestalt_rust::Output<String>,
        /// The ID or ARN of a KMS Key used to encrypt trained Entity Recognizers.
        pub model_kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name for the Entity Recognizer.
        /// Has a maximum length of 63 characters.
        /// Can contain upper- and lower-case letters, numbers, and hypen (`-`).
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` Configuration Block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name for the version of the Entity Recognizer.
        /// Each version must have a unique name within the Entity Recognizer.
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
        /// ID or ARN of a KMS Key used to encrypt storage volumes during job processing.
        pub volume_kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration parameters for VPC to contain Entity Recognizer resources.
        /// See the `vpc_config` Configuration Block section below.
        pub vpc_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::comprehend::EntityRecognizerVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EntityRecognizerArgs,
    ) -> EntityRecognizerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let data_access_role_arn_binding = args
            .data_access_role_arn
            .get_output(context)
            .get_inner();
        let input_data_config_binding = args
            .input_data_config
            .get_output(context)
            .get_inner();
        let language_code_binding = args.language_code.get_output(context).get_inner();
        let model_kms_key_id_binding = args
            .model_kms_key_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let version_name_binding = args.version_name.get_output(context).get_inner();
        let version_name_prefix_binding = args
            .version_name_prefix
            .get_output(context)
            .get_inner();
        let volume_kms_key_id_binding = args
            .volume_kms_key_id
            .get_output(context)
            .get_inner();
        let vpc_config_binding = args.vpc_config.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:comprehend/entityRecognizer:EntityRecognizer".into(),
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
                    name: "modelKmsKeyId".into(),
                    value: &model_kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
        EntityRecognizerResult {
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
            model_kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modelKmsKeyId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
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
