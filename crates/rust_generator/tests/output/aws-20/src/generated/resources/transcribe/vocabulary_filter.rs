/// Resource for managing an AWS Transcribe VocabularyFilter.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:transcribe:VocabularyFilter
///     properties:
///       vocabularyFilterName: example
///       languageCode: en-US
///       words:
///         - cars
///         - bucket
///       tags:
///         tag1: value1
///         tag2: value3
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Transcribe VocabularyFilter using the `vocabulary_filter_name`. For example:
///
/// ```sh
/// $ pulumi import aws:transcribe/vocabularyFilter:VocabularyFilter example example-name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vocabulary_filter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VocabularyFilterArgs {
        /// The language code you selected for your vocabulary filter. Refer to the [supported languages](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html) page for accepted codes.
        #[builder(into)]
        pub language_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the VocabularyFilter. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon S3 location (URI) of the text file that contains your custom VocabularyFilter. Conflicts with `words` argument.
        #[builder(into, default)]
        pub vocabulary_filter_file_uri: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the VocabularyFilter.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vocabulary_filter_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of terms to include in the vocabulary. Conflicts with `vocabulary_filter_file_uri` argument.
        #[builder(into, default)]
        pub words: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct VocabularyFilterResult {
        /// ARN of the VocabularyFilter.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Generated download URI.
        pub download_uri: pulumi_gestalt_rust::Output<String>,
        /// The language code you selected for your vocabulary filter. Refer to the [supported languages](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html) page for accepted codes.
        pub language_code: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the VocabularyFilter. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon S3 location (URI) of the text file that contains your custom VocabularyFilter. Conflicts with `words` argument.
        pub vocabulary_filter_file_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the VocabularyFilter.
        ///
        /// The following arguments are optional:
        pub vocabulary_filter_name: pulumi_gestalt_rust::Output<String>,
        /// A list of terms to include in the vocabulary. Conflicts with `vocabulary_filter_file_uri` argument.
        pub words: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VocabularyFilterArgs,
    ) -> VocabularyFilterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let language_code_binding = args.language_code.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vocabulary_filter_file_uri_binding = args
            .vocabulary_filter_file_uri
            .get_output(context);
        let vocabulary_filter_name_binding = args
            .vocabulary_filter_name
            .get_output(context);
        let words_binding = args.words.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:transcribe/vocabularyFilter:VocabularyFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "languageCode".into(),
                    value: language_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vocabularyFilterFileUri".into(),
                    value: vocabulary_filter_file_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vocabularyFilterName".into(),
                    value: vocabulary_filter_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "words".into(),
                    value: words_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VocabularyFilterResult {
            arn: o.get_field("arn"),
            download_uri: o.get_field("downloadUri"),
            language_code: o.get_field("languageCode"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vocabulary_filter_file_uri: o.get_field("vocabularyFilterFileUri"),
            vocabulary_filter_name: o.get_field("vocabularyFilterName"),
            words: o.get_field("words"),
        }
    }
}
