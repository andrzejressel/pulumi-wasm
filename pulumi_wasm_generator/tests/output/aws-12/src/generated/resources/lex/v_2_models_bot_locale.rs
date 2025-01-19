/// Resource for managing an AWS Lex V2 Models Bot Locale.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = v_2_models_bot_locale::create(
///         "example",
///         V2ModelsBotLocaleArgs::builder()
///             .bot_id("${exampleAwsLexv2modelsBot.id}")
///             .bot_version("DRAFT")
///             .locale_id("en_US")
///             .n_lu_intent_confidence_threshold(0.7)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Voice Settings
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = v_2_models_bot_locale::create(
///         "example",
///         V2ModelsBotLocaleArgs::builder()
///             .bot_id("${exampleAwsLexv2modelsBot.id}")
///             .bot_version("DRAFT")
///             .locale_id("en_US")
///             .n_lu_intent_confidence_threshold(0.7)
///             .voice_settings(
///                 V2ModelsBotLocaleVoiceSettings::builder()
///                     .engine("standard")
///                     .voiceId("Kendra")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lex V2 Models Bot Locale using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:lex/v2modelsBotLocale:V2modelsBotLocale example en_US,abcd-12345678,1
/// ```
pub mod v_2_models_bot_locale {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2modelsBotLocaleArgs {
        /// Identifier of the bot to create the locale for.
        #[builder(into)]
        pub bot_id: pulumi_wasm_rust::Output<String>,
        /// Version of the bot to create the locale for. This can only be the draft version of the bot.
        #[builder(into)]
        pub bot_version: pulumi_wasm_rust::Output<String>,
        /// Description of the bot locale. Use this to help identify the bot locale in lists.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the language and locale that the bot will be used in. The string must match one of the supported locales. All of the intents, slot types, and slots used in the bot must have the same locale. For more information, see Supported languages (https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html)
        #[builder(into)]
        pub locale_id: pulumi_wasm_rust::Output<String>,
        /// Determines the threshold where Amazon Lex will insert the AMAZON.FallbackIntent, AMAZON.KendraSearchIntent, or both when returning alternative intents.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub n_lu_intent_confidence_threshold: pulumi_wasm_rust::Output<f64>,
        /// Specified locale name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::lex::V2ModelsBotLocaleTimeouts>,
        >,
        /// Amazon Polly voice ID that Amazon Lex uses for voice interaction with the user. See `voice_settings`.
        #[builder(into, default)]
        pub voice_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::lex::V2ModelsBotLocaleVoiceSettings>,
        >,
    }
    #[allow(dead_code)]
    pub struct V2modelsBotLocaleResult {
        /// Identifier of the bot to create the locale for.
        pub bot_id: pulumi_wasm_rust::Output<String>,
        /// Version of the bot to create the locale for. This can only be the draft version of the bot.
        pub bot_version: pulumi_wasm_rust::Output<String>,
        /// Description of the bot locale. Use this to help identify the bot locale in lists.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the language and locale that the bot will be used in. The string must match one of the supported locales. All of the intents, slot types, and slots used in the bot must have the same locale. For more information, see Supported languages (https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html)
        pub locale_id: pulumi_wasm_rust::Output<String>,
        /// Determines the threshold where Amazon Lex will insert the AMAZON.FallbackIntent, AMAZON.KendraSearchIntent, or both when returning alternative intents.
        ///
        /// The following arguments are optional:
        pub n_lu_intent_confidence_threshold: pulumi_wasm_rust::Output<f64>,
        /// Specified locale name.
        pub name: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::lex::V2ModelsBotLocaleTimeouts>,
        >,
        /// Amazon Polly voice ID that Amazon Lex uses for voice interaction with the user. See `voice_settings`.
        pub voice_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::lex::V2ModelsBotLocaleVoiceSettings>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: V2modelsBotLocaleArgs) -> V2modelsBotLocaleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bot_id_binding = args.bot_id.get_inner();
        let bot_version_binding = args.bot_version.get_inner();
        let description_binding = args.description.get_inner();
        let locale_id_binding = args.locale_id.get_inner();
        let n_lu_intent_confidence_threshold_binding = args
            .n_lu_intent_confidence_threshold
            .get_inner();
        let name_binding = args.name.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let voice_settings_binding = args.voice_settings.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lex/v2modelsBotLocale:V2modelsBotLocale".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "botId".into(),
                    value: &bot_id_binding,
                },
                register_interface::ObjectField {
                    name: "botVersion".into(),
                    value: &bot_version_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "localeId".into(),
                    value: &locale_id_binding,
                },
                register_interface::ObjectField {
                    name: "nLuIntentConfidenceThreshold".into(),
                    value: &n_lu_intent_confidence_threshold_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "voiceSettings".into(),
                    value: &voice_settings_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "botId".into(),
                },
                register_interface::ResultField {
                    name: "botVersion".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "localeId".into(),
                },
                register_interface::ResultField {
                    name: "nLuIntentConfidenceThreshold".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "voiceSettings".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        V2modelsBotLocaleResult {
            bot_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("botId").unwrap(),
            ),
            bot_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("botVersion").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            locale_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localeId").unwrap(),
            ),
            n_lu_intent_confidence_threshold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nLuIntentConfidenceThreshold").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            voice_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("voiceSettings").unwrap(),
            ),
        }
    }
}
