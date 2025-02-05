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
pub mod vocabulary {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VocabularyArgs {
        /// The language code you selected for your vocabulary.
        #[builder(into)]
        pub language_code: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of terms to include in the vocabulary. Conflicts with `vocabulary_file_uri`
        #[builder(into, default)]
        pub phrases: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the Vocabulary. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon S3 location (URI) of the text file that contains your custom vocabulary. Conflicts wth `phrases`.
        #[builder(into, default)]
        pub vocabulary_file_uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Vocabulary.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vocabulary_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VocabularyResult {
        /// ARN of the Vocabulary.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Generated download URI.
        pub download_uri: pulumi_wasm_rust::Output<String>,
        /// The language code you selected for your vocabulary.
        pub language_code: pulumi_wasm_rust::Output<String>,
        /// A list of terms to include in the vocabulary. Conflicts with `vocabulary_file_uri`
        pub phrases: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of tags to assign to the Vocabulary. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon S3 location (URI) of the text file that contains your custom vocabulary. Conflicts wth `phrases`.
        pub vocabulary_file_uri: pulumi_wasm_rust::Output<String>,
        /// The name of the Vocabulary.
        ///
        /// The following arguments are optional:
        pub vocabulary_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VocabularyArgs,
    ) -> VocabularyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let language_code_binding = args.language_code.get_output(context).get_inner();
        let phrases_binding = args.phrases.get_output(context).get_inner();
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
            type_: "aws:transcribe/vocabulary:Vocabulary".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "languageCode".into(),
                    value: &language_code_binding,
                },
                register_interface::ObjectField {
                    name: "phrases".into(),
                    value: &phrases_binding,
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
        VocabularyResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            download_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("downloadUri"),
            ),
            language_code: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("languageCode"),
            ),
            phrases: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("phrases"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vocabulary_file_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vocabularyFileUri"),
            ),
            vocabulary_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vocabularyName"),
            ),
        }
    }
}
