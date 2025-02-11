/// Resource for managing an AWS Lex V2 Models Bot Locale.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_2_models_bot_locale {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2modelsBotLocaleArgs {
        /// Identifier of the bot to create the locale for.
        #[builder(into)]
        pub bot_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the bot to create the locale for. This can only be the draft version of the bot.
        #[builder(into)]
        pub bot_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the bot locale. Use this to help identify the bot locale in lists.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the language and locale that the bot will be used in. The string must match one of the supported locales. All of the intents, slot types, and slots used in the bot must have the same locale. For more information, see Supported languages (https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html)
        #[builder(into)]
        pub locale_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Determines the threshold where Amazon Lex will insert the AMAZON.FallbackIntent, AMAZON.KendraSearchIntent, or both when returning alternative intents.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub n_lu_intent_confidence_threshold: pulumi_gestalt_rust::InputOrOutput<f64>,
        /// Specified locale name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsBotLocaleTimeouts>,
        >,
        /// Amazon Polly voice ID that Amazon Lex uses for voice interaction with the user. See `voice_settings`.
        #[builder(into, default)]
        pub voice_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsBotLocaleVoiceSettings>,
        >,
    }
    #[allow(dead_code)]
    pub struct V2modelsBotLocaleResult {
        /// Identifier of the bot to create the locale for.
        pub bot_id: pulumi_gestalt_rust::Output<String>,
        /// Version of the bot to create the locale for. This can only be the draft version of the bot.
        pub bot_version: pulumi_gestalt_rust::Output<String>,
        /// Description of the bot locale. Use this to help identify the bot locale in lists.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of the language and locale that the bot will be used in. The string must match one of the supported locales. All of the intents, slot types, and slots used in the bot must have the same locale. For more information, see Supported languages (https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html)
        pub locale_id: pulumi_gestalt_rust::Output<String>,
        /// Determines the threshold where Amazon Lex will insert the AMAZON.FallbackIntent, AMAZON.KendraSearchIntent, or both when returning alternative intents.
        ///
        /// The following arguments are optional:
        pub n_lu_intent_confidence_threshold: pulumi_gestalt_rust::Output<f64>,
        /// Specified locale name.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsBotLocaleTimeouts>,
        >,
        /// Amazon Polly voice ID that Amazon Lex uses for voice interaction with the user. See `voice_settings`.
        pub voice_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsBotLocaleVoiceSettings>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: V2modelsBotLocaleArgs,
    ) -> V2modelsBotLocaleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bot_id_binding = args.bot_id.get_output(context);
        let bot_version_binding = args.bot_version.get_output(context);
        let description_binding = args.description.get_output(context);
        let locale_id_binding = args.locale_id.get_output(context);
        let n_lu_intent_confidence_threshold_binding = args
            .n_lu_intent_confidence_threshold
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let voice_settings_binding = args.voice_settings.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lex/v2modelsBotLocale:V2modelsBotLocale".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botId".into(),
                    value: &bot_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botVersion".into(),
                    value: &bot_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localeId".into(),
                    value: &locale_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nLuIntentConfidenceThreshold".into(),
                    value: &n_lu_intent_confidence_threshold_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "voiceSettings".into(),
                    value: &voice_settings_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        V2modelsBotLocaleResult {
            bot_id: o.get_field("botId"),
            bot_version: o.get_field("botVersion"),
            description: o.get_field("description"),
            locale_id: o.get_field("localeId"),
            n_lu_intent_confidence_threshold: o
                .get_field("nLuIntentConfidenceThreshold"),
            name: o.get_field("name"),
            timeouts: o.get_field("timeouts"),
            voice_settings: o.get_field("voiceSettings"),
        }
    }
}
