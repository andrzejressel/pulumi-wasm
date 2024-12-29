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
///       dependson:
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
pub mod medical_vocabulary {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MedicalVocabularyArgs {
        /// The language code you selected for your medical vocabulary. US English (en-US) is the only language supported with Amazon Transcribe Medical.
        #[builder(into)]
        pub language_code: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the MedicalVocabulary. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon S3 location (URI) of the text file that contains your custom medical vocabulary.
        #[builder(into)]
        pub vocabulary_file_uri: pulumi_wasm_rust::Output<String>,
        /// The name of the Medical Vocabulary.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vocabulary_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MedicalVocabularyResult {
        /// ARN of the MedicalVocabulary.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Generated download URI.
        pub download_uri: pulumi_wasm_rust::Output<String>,
        /// The language code you selected for your medical vocabulary. US English (en-US) is the only language supported with Amazon Transcribe Medical.
        pub language_code: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the MedicalVocabulary. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon S3 location (URI) of the text file that contains your custom medical vocabulary.
        pub vocabulary_file_uri: pulumi_wasm_rust::Output<String>,
        /// The name of the Medical Vocabulary.
        ///
        /// The following arguments are optional:
        pub vocabulary_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MedicalVocabularyArgs) -> MedicalVocabularyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let language_code_binding = args.language_code.get_inner();
        let tags_binding = args.tags.get_inner();
        let vocabulary_file_uri_binding = args.vocabulary_file_uri.get_inner();
        let vocabulary_name_binding = args.vocabulary_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:transcribe/medicalVocabulary:MedicalVocabulary".into(),
            name: name.to_string(),
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
                    name: "vocabularyFileUri".into(),
                },
                register_interface::ResultField {
                    name: "vocabularyName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MedicalVocabularyResult {
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
            vocabulary_file_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vocabularyFileUri").unwrap(),
            ),
            vocabulary_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vocabularyName").unwrap(),
            ),
        }
    }
}
