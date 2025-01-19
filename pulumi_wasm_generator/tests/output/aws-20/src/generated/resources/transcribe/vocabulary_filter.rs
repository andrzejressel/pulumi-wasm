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
pub mod vocabulary_filter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VocabularyFilterArgs {
        /// The language code you selected for your vocabulary filter. Refer to the [supported languages](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html) page for accepted codes.
        #[builder(into)]
        pub language_code: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the VocabularyFilter. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon S3 location (URI) of the text file that contains your custom VocabularyFilter. Conflicts with `words` argument.
        #[builder(into, default)]
        pub vocabulary_filter_file_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the VocabularyFilter.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vocabulary_filter_name: pulumi_wasm_rust::Output<String>,
        /// A list of terms to include in the vocabulary. Conflicts with `vocabulary_filter_file_uri` argument.
        #[builder(into, default)]
        pub words: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct VocabularyFilterResult {
        /// ARN of the VocabularyFilter.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Generated download URI.
        pub download_uri: pulumi_wasm_rust::Output<String>,
        /// The language code you selected for your vocabulary filter. Refer to the [supported languages](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html) page for accepted codes.
        pub language_code: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the VocabularyFilter. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon S3 location (URI) of the text file that contains your custom VocabularyFilter. Conflicts with `words` argument.
        pub vocabulary_filter_file_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the VocabularyFilter.
        ///
        /// The following arguments are optional:
        pub vocabulary_filter_name: pulumi_wasm_rust::Output<String>,
        /// A list of terms to include in the vocabulary. Conflicts with `vocabulary_filter_file_uri` argument.
        pub words: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VocabularyFilterArgs) -> VocabularyFilterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let language_code_binding = args.language_code.get_inner();
        let tags_binding = args.tags.get_inner();
        let vocabulary_filter_file_uri_binding = args
            .vocabulary_filter_file_uri
            .get_inner();
        let vocabulary_filter_name_binding = args.vocabulary_filter_name.get_inner();
        let words_binding = args.words.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:transcribe/vocabularyFilter:VocabularyFilter".into(),
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
                    name: "vocabularyFilterFileUri".into(),
                    value: &vocabulary_filter_file_uri_binding,
                },
                register_interface::ObjectField {
                    name: "vocabularyFilterName".into(),
                    value: &vocabulary_filter_name_binding,
                },
                register_interface::ObjectField {
                    name: "words".into(),
                    value: &words_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "downloadUri".into(),
                },
                register_interface::ResultField {
                    name: "languageCode".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vocabularyFilterFileUri".into(),
                },
                register_interface::ResultField {
                    name: "vocabularyFilterName".into(),
                },
                register_interface::ResultField {
                    name: "words".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VocabularyFilterResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            download_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("downloadUri").unwrap(),
            ),
            language_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("languageCode").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vocabulary_filter_file_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vocabularyFilterFileUri").unwrap(),
            ),
            vocabulary_filter_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vocabularyFilterName").unwrap(),
            ),
            words: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("words").unwrap(),
            ),
        }
    }
}
