pub mod get_voices {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVoicesArgs {
        /// Engine used by Amazon Polly when processing input text for speech synthesis. Valid values are `standard`, `neural`, and `long-form`.
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to return any bilingual voices that use the specified language as an additional language.
        #[builder(into, default)]
        pub include_additional_language_codes: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Language identification tag for filtering the list of voices returned. If not specified, all available voices are returned.
        #[builder(into, default)]
        pub language_code: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of voices with their properties. See `voices` Attribute Reference below.
        #[builder(into, default)]
        pub voices: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::polly::GetVoicesVoice>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVoicesResult {
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Polly assigned voice ID.
        pub id: pulumi_wasm_rust::Output<String>,
        pub include_additional_language_codes: pulumi_wasm_rust::Output<Option<bool>>,
        /// Language code of the voice.
        pub language_code: pulumi_wasm_rust::Output<Option<String>>,
        /// List of voices with their properties. See `voices` Attribute Reference below.
        pub voices: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::polly::GetVoicesVoice>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetVoicesArgs,
    ) -> GetVoicesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let engine_binding = args.engine.get_output(context).get_inner();
        let include_additional_language_codes_binding = args
            .include_additional_language_codes
            .get_output(context)
            .get_inner();
        let language_code_binding = args.language_code.get_output(context).get_inner();
        let voices_binding = args.voices.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:polly/getVoices:getVoices".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "includeAdditionalLanguageCodes".into(),
                    value: &include_additional_language_codes_binding,
                },
                register_interface::ObjectField {
                    name: "languageCode".into(),
                    value: &language_code_binding,
                },
                register_interface::ObjectField {
                    name: "voices".into(),
                    value: &voices_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVoicesResult {
            engine: pulumi_wasm_rust::__private::into_domain(o.extract_field("engine")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            include_additional_language_codes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("includeAdditionalLanguageCodes"),
            ),
            language_code: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("languageCode"),
            ),
            voices: pulumi_wasm_rust::__private::into_domain(o.extract_field("voices")),
        }
    }
}
