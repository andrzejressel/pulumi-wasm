pub mod get_vocabulary {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVocabularyArgs {
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Returns information on a specific Vocabulary by name
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the Vocabulary.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Returns information on a specific Vocabulary by Vocabulary id
        #[builder(into, default)]
        pub vocabulary_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetVocabularyResult {
        /// The Amazon Resource Name (ARN) of the Vocabulary.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The content of the custom vocabulary in plain-text format with a table of values. Each row in the table represents a word or a phrase, described with Phrase, IPA, SoundsLike, and DisplayAs fields. Separate the fields with TAB characters. For more information, see [Create a custom vocabulary using a table](https://docs.aws.amazon.com/transcribe/latest/dg/custom-vocabulary.html#create-vocabulary-table).
        pub content: pulumi_wasm_rust::Output<String>,
        /// The reason why the custom vocabulary was not created.
        pub failure_reason: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see [What is Amazon Transcribe?](https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html). Valid Values are `ar-AE`, `de-CH`, `de-DE`, `en-AB`, `en-AU`, `en-GB`, `en-IE`, `en-IN`, `en-US`, `en-WL`, `es-ES`, `es-US`, `fr-CA`, `fr-FR`, `hi-IN`, `it-IT`, `ja-JP`, `ko-KR`, `pt-BR`, `pt-PT`, `zh-CN`.
        pub language_code: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the custom vocabulary was last modified.
        pub last_modified_time: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The current state of the custom vocabulary. Valid values are `CREATION_IN_PROGRESS`, `ACTIVE`, `CREATION_FAILED`, `DELETE_IN_PROGRESS`.
        pub state: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the Vocabulary.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The identifier of the custom vocabulary.
        pub vocabulary_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetVocabularyArgs,
    ) -> GetVocabularyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vocabulary_id_binding = args.vocabulary_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getVocabulary:getVocabulary".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
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
                    name: "vocabularyId".into(),
                    value: &vocabulary_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVocabularyResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            content: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            failure_reason: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("failureReason"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            language_code: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("languageCode"),
            ),
            last_modified_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastModifiedTime"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            vocabulary_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vocabularyId"),
            ),
        }
    }
}
