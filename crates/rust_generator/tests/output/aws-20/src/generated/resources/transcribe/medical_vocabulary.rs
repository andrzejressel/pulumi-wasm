/// Resource for managing an AWS Transcribe MedicalVocabulary.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example-medical-vocab-123
///       forceDestroy: true
///   object:
///     type: aws:s3:BucketObjectv2
///     properties:
///       bucket: ${example.id}
///       key: transcribe/test1.txt
///       source:
///         fn::FileAsset: test.txt
///   exampleMedicalVocabulary:
///     type: aws:transcribe:MedicalVocabulary
///     name: example
///     properties:
///       vocabularyName: example
///       languageCode: en-US
///       vocabularyFileUri: s3://${example.id}/${object.key}
///       tags:
///         tag1: value1
///         tag2: value3
///     options:
///       dependsOn:
///         - ${object}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Transcribe MedicalVocabulary using the `vocabulary_name`. For example:
///
/// ```sh
/// $ pulumi import aws:transcribe/medicalVocabulary:MedicalVocabulary example example-name
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod medical_vocabulary {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MedicalVocabularyArgs {
        /// The language code you selected for your medical vocabulary. US English (en-US) is the only language supported with Amazon Transcribe Medical.
        #[builder(into)]
        pub language_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the MedicalVocabulary. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon S3 location (URI) of the text file that contains your custom medical vocabulary.
        #[builder(into)]
        pub vocabulary_file_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Medical Vocabulary.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vocabulary_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MedicalVocabularyResult {
        /// ARN of the MedicalVocabulary.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Generated download URI.
        pub download_uri: pulumi_gestalt_rust::Output<String>,
        /// The language code you selected for your medical vocabulary. US English (en-US) is the only language supported with Amazon Transcribe Medical.
        pub language_code: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the MedicalVocabulary. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon S3 location (URI) of the text file that contains your custom medical vocabulary.
        pub vocabulary_file_uri: pulumi_gestalt_rust::Output<String>,
        /// The name of the Medical Vocabulary.
        ///
        /// The following arguments are optional:
        pub vocabulary_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MedicalVocabularyArgs,
    ) -> MedicalVocabularyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let language_code_binding = args.language_code.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vocabulary_file_uri_binding = args
            .vocabulary_file_uri
            .get_output(context)
            .get_inner();
        let vocabulary_name_binding = args
            .vocabulary_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:transcribe/medicalVocabulary:MedicalVocabulary".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "languageCode".into(),
                    value: &language_code_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vocabularyFileUri".into(),
                    value: &vocabulary_file_uri_binding,
                },
                register_interface::ObjectField {
                    name: "vocabularyName".into(),
                    value: &vocabulary_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MedicalVocabularyResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            download_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("downloadUri"),
            ),
            language_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("languageCode"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vocabulary_file_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vocabularyFileUri"),
            ),
            vocabulary_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vocabularyName"),
            ),
        }
    }
}
