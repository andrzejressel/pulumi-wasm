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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EntityRecognizerArgs,
    ) -> EntityRecognizerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_access_role_arn_binding = args.data_access_role_arn.get_output(context);
        let input_data_config_binding = args.input_data_config.get_output(context);
        let language_code_binding = args.language_code.get_output(context);
        let model_kms_key_id_binding = args.model_kms_key_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let version_name_binding = args.version_name.get_output(context);
        let version_name_prefix_binding = args.version_name_prefix.get_output(context);
        let volume_kms_key_id_binding = args.volume_kms_key_id.get_output(context);
        let vpc_config_binding = args.vpc_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:comprehend/entityRecognizer:EntityRecognizer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataAccessRoleArn".into(),
                    value: data_access_role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputDataConfig".into(),
                    value: input_data_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "languageCode".into(),
                    value: language_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "modelKmsKeyId".into(),
                    value: model_kms_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionName".into(),
                    value: version_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionNamePrefix".into(),
                    value: version_name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumeKmsKeyId".into(),
                    value: volume_kms_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfig".into(),
                    value: vpc_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EntityRecognizerResult {
            arn: o.get_field("arn"),
            data_access_role_arn: o.get_field("dataAccessRoleArn"),
            input_data_config: o.get_field("inputDataConfig"),
            language_code: o.get_field("languageCode"),
            model_kms_key_id: o.get_field("modelKmsKeyId"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version_name: o.get_field("versionName"),
            version_name_prefix: o.get_field("versionNamePrefix"),
            volume_kms_key_id: o.get_field("volumeKmsKeyId"),
            vpc_config: o.get_field("vpcConfig"),
        }
    }
}
