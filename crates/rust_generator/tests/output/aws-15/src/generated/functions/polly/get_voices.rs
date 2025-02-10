#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_voices {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVoicesArgs {
        /// Engine used by Amazon Polly when processing input text for speech synthesis. Valid values are `standard`, `neural`, and `long-form`.
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to return any bilingual voices that use the specified language as an additional language.
        #[builder(into, default)]
        pub include_additional_language_codes: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Language identification tag for filtering the list of voices returned. If not specified, all available voices are returned.
        #[builder(into, default)]
        pub language_code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of voices with their properties. See `voices` Attribute Reference below.
        #[builder(into, default)]
        pub voices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::polly::GetVoicesVoice>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVoicesResult {
        pub engine: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Polly assigned voice ID.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub include_additional_language_codes: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Language code of the voice.
        pub language_code: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of voices with their properties. See `voices` Attribute Reference below.
        pub voices: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::polly::GetVoicesVoice>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVoicesArgs,
    ) -> GetVoicesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let engine_binding = args.engine.get_output(context);
        let include_additional_language_codes_binding = args
            .include_additional_language_codes
            .get_output(context);
        let language_code_binding = args.language_code.get_output(context);
        let voices_binding = args.voices.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:polly/getVoices:getVoices".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: engine_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeAdditionalLanguageCodes".into(),
                    value: include_additional_language_codes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "languageCode".into(),
                    value: language_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "voices".into(),
                    value: voices_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVoicesResult {
            engine: o.get_field("engine"),
            id: o.get_field("id"),
            include_additional_language_codes: o
                .get_field("includeAdditionalLanguageCodes"),
            language_code: o.get_field("languageCode"),
            voices: o.get_field("voices"),
        }
    }
}
