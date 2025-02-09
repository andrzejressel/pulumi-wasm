/// Provides an Amazon Connect Vocabulary resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:connect:Vocabulary
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: example
///       content: |-
///         Phrase	IPA	SoundsLike	DisplayAs
///         Los-Angeles			Los Angeles
///         F.B.I.	ɛ f b i aɪ		FBI
///         Etienne		eh-tee-en	
///       languageCode: en-US
///       tags:
///         Key1: Value1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect Vocabularies using the `instance_id` and `vocabulary_id` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/vocabulary:Vocabulary example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vocabulary {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VocabularyArgs {
        /// The content of the custom vocabulary in plain-text format with a table of values. Each row in the table represents a word or a phrase, described with Phrase, IPA, SoundsLike, and DisplayAs fields. Separate the fields with TAB characters. For more information, see [Create a custom vocabulary using a table](https://docs.aws.amazon.com/transcribe/latest/dg/custom-vocabulary.html#create-vocabulary-table). Minimum length of `1`. Maximum length of `60000`.
        #[builder(into)]
        pub content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see [What is Amazon Transcribe?](https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html). Valid Values are `ar-AE`, `de-CH`, `de-DE`, `en-AB`, `en-AU`, `en-GB`, `en-IE`, `en-IN`, `en-US`, `en-WL`, `es-ES`, `es-US`, `fr-CA`, `fr-FR`, `hi-IN`, `it-IT`, `ja-JP`, `ko-KR`, `pt-BR`, `pt-PT`, `zh-CN`.
        #[builder(into)]
        pub language_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique name of the custom vocabulary. Must not be more than 140 characters.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the vocabulary. If configured with a provider
        /// `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VocabularyResult {
        /// The Amazon Resource Name (ARN) of the vocabulary.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The content of the custom vocabulary in plain-text format with a table of values. Each row in the table represents a word or a phrase, described with Phrase, IPA, SoundsLike, and DisplayAs fields. Separate the fields with TAB characters. For more information, see [Create a custom vocabulary using a table](https://docs.aws.amazon.com/transcribe/latest/dg/custom-vocabulary.html#create-vocabulary-table). Minimum length of `1`. Maximum length of `60000`.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// The reason why the custom vocabulary was not created.
        pub failure_reason: pulumi_gestalt_rust::Output<String>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see [What is Amazon Transcribe?](https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html). Valid Values are `ar-AE`, `de-CH`, `de-DE`, `en-AB`, `en-AU`, `en-GB`, `en-IE`, `en-IN`, `en-US`, `en-WL`, `es-ES`, `es-US`, `fr-CA`, `fr-FR`, `hi-IN`, `it-IT`, `ja-JP`, `ko-KR`, `pt-BR`, `pt-PT`, `zh-CN`.
        pub language_code: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the custom vocabulary was last modified.
        pub last_modified_time: pulumi_gestalt_rust::Output<String>,
        /// A unique name of the custom vocabulary. Must not be more than 140 characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The current state of the custom vocabulary. Valid values are `CREATION_IN_PROGRESS`, `ACTIVE`, `CREATION_FAILED`, `DELETE_IN_PROGRESS`.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the vocabulary. If configured with a provider
        /// `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The identifier of the custom vocabulary.
        pub vocabulary_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VocabularyArgs,
    ) -> VocabularyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let content_binding_1 = args.content.get_output(context);
        let content_binding = content_binding_1.get_inner();
        let instance_id_binding_1 = args.instance_id.get_output(context);
        let instance_id_binding = instance_id_binding_1.get_inner();
        let language_code_binding_1 = args.language_code.get_output(context);
        let language_code_binding = language_code_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:connect/vocabulary:Vocabulary".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "languageCode".into(),
                    value: &language_code_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VocabularyResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            failure_reason: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("failureReason"),
            ),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            language_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("languageCode"),
            ),
            last_modified_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModifiedTime"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vocabulary_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vocabularyId"),
            ),
        }
    }
}
