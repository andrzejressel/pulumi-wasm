/// Resource for managing an AWS Transcribe Vocabulary.
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
///       bucket: example-vocab-123
///       forceDestroy: true
///   object:
///     type: aws:s3:BucketObjectv2
///     properties:
///       bucket: ${example.id}
///       key: transcribe/test1.txt
///       source:
///         fn::FileAsset: test.txt
///   exampleVocabulary:
///     type: aws:transcribe:Vocabulary
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
/// Using `pulumi import`, import Transcribe Vocabulary using the `vocabulary_name`. For example:
///
/// ```sh
/// $ pulumi import aws:transcribe/vocabulary:Vocabulary example example-name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vocabulary {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VocabularyArgs {
        /// The language code you selected for your vocabulary.
        #[builder(into)]
        pub language_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of terms to include in the vocabulary. Conflicts with `vocabulary_file_uri`
        #[builder(into, default)]
        pub phrases: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the Vocabulary. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon S3 location (URI) of the text file that contains your custom vocabulary. Conflicts wth `phrases`.
        #[builder(into, default)]
        pub vocabulary_file_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Vocabulary.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vocabulary_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VocabularyResult {
        /// ARN of the Vocabulary.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Generated download URI.
        pub download_uri: pulumi_gestalt_rust::Output<String>,
        /// The language code you selected for your vocabulary.
        pub language_code: pulumi_gestalt_rust::Output<String>,
        /// A list of terms to include in the vocabulary. Conflicts with `vocabulary_file_uri`
        pub phrases: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A map of tags to assign to the Vocabulary. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon S3 location (URI) of the text file that contains your custom vocabulary. Conflicts wth `phrases`.
        pub vocabulary_file_uri: pulumi_gestalt_rust::Output<String>,
        /// The name of the Vocabulary.
        ///
        /// The following arguments are optional:
        pub vocabulary_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VocabularyArgs,
    ) -> VocabularyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let language_code_binding = args.language_code.get_output(context);
        let phrases_binding = args.phrases.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vocabulary_file_uri_binding = args.vocabulary_file_uri.get_output(context);
        let vocabulary_name_binding = args.vocabulary_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:transcribe/vocabulary:Vocabulary".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "languageCode".into(),
                    value: language_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "phrases".into(),
                    value: phrases_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vocabularyFileUri".into(),
                    value: vocabulary_file_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vocabularyName".into(),
                    value: vocabulary_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VocabularyResult {
            arn: o.get_field("arn"),
            download_uri: o.get_field("downloadUri"),
            language_code: o.get_field("languageCode"),
            phrases: o.get_field("phrases"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vocabulary_file_uri: o.get_field("vocabularyFileUri"),
            vocabulary_name: o.get_field("vocabularyName"),
        }
    }
}
