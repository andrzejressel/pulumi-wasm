/// Provides an Amazon Lex Bot resource. For more information see
/// [Amazon Lex: How It Works](https://docs.aws.amazon.com/lex/latest/dg/how-it-works.html)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let orderFlowersBot = bot::create(
///         "orderFlowersBot",
///         BotArgs::builder()
///             .abort_statement(
///                 BotAbortStatement::builder()
///                     .messages(
///                         vec![
///                             BotAbortStatementMessage::builder()
///                             .content("Sorry, I am not able to assist at this time")
///                             .contentType("PlainText").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .child_directed(false)
///             .clarification_prompt(
///                 BotClarificationPrompt::builder()
///                     .maxAttempts(2)
///                     .messages(
///                         vec![
///                             BotClarificationPromptMessage::builder()
///                             .content("I didn't understand you, what would you like to do?")
///                             .contentType("PlainText").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .create_version(false)
///             .description("Bot to order flowers on the behalf of a user")
///             .idle_session_ttl_in_seconds(600)
///             .intents(
///                 vec![
///                     BotIntent::builder().intentName("OrderFlowers").intentVersion("1")
///                     .build_struct(),
///                 ],
///             )
///             .locale("en-US")
///             .name("OrderFlowers")
///             .process_behavior("BUILD")
///             .voice_id("Salli")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import bots using their name. For example:
///
/// ```sh
/// $ pulumi import aws:lex/bot:Bot order_flowers_bot OrderFlowers
/// ```
pub mod bot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BotArgs {
        /// The message that Amazon Lex uses to abort a conversation. Attributes are documented under statement.
        #[builder(into)]
        pub abort_statement: pulumi_wasm_rust::Output<
            super::super::types::lex::BotAbortStatement,
        >,
        /// By specifying true, you confirm that your use of Amazon Lex is related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA. For more information see the [Amazon Lex FAQ](https://aws.amazon.com/lex/faqs#data-security) and the [Amazon Lex PutBot API Docs](https://docs.aws.amazon.com/lex/latest/dg/API_PutBot.html#lex-PutBot-request-childDirected).
        #[builder(into)]
        pub child_directed: pulumi_wasm_rust::Output<bool>,
        /// The message that Amazon Lex uses when it doesn't understand the user's request. Attributes are documented under prompt.
        #[builder(into, default)]
        pub clarification_prompt: pulumi_wasm_rust::Output<
            Option<super::super::types::lex::BotClarificationPrompt>,
        >,
        /// Determines if a new bot version is created when the initial resource is created and on each update. Defaults to `false`.
        #[builder(into, default)]
        pub create_version: pulumi_wasm_rust::Output<Option<bool>>,
        /// A description of the bot. Must be less than or equal to 200 characters in length.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// When set to true user utterances are sent to Amazon Comprehend for sentiment analysis. If you don't specify detectSentiment, the default is `false`.
        #[builder(into, default)]
        pub detect_sentiment: pulumi_wasm_rust::Output<Option<bool>>,
        /// Set to `true` to enable access to natural language understanding improvements. When you set the `enable_model_improvements` parameter to true you can use the `nlu_intent_confidence_threshold` parameter to configure confidence scores. For more information, see [Confidence Scores](https://docs.aws.amazon.com/lex/latest/dg/confidence-scores.html). You can only set the `enable_model_improvements` parameter in certain Regions. If you set the parameter to true, your bot has access to accuracy improvements. For more information see the [Amazon Lex Bot PutBot API Docs](https://docs.aws.amazon.com/lex/latest/dg/API_PutBot.html#lex-PutBot-request-enableModelImprovements).
        #[builder(into, default)]
        pub enable_model_improvements: pulumi_wasm_rust::Output<Option<bool>>,
        /// The maximum time in seconds that Amazon Lex retains the data gathered in a conversation. Default is `300`. Must be a number between 60 and 86400 (inclusive).
        #[builder(into, default)]
        pub idle_session_ttl_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// A set of Intent objects. Each intent represents a command that a user can express. Attributes are documented under intent. Can have up to 250 Intent objects.
        #[builder(into)]
        pub intents: pulumi_wasm_rust::Output<Vec<super::super::types::lex::BotIntent>>,
        /// Specifies the target locale for the bot. Any intent used in the bot must be compatible with the locale of the bot. For available locales, see [Amazon Lex Bot PutBot API Docs](https://docs.aws.amazon.com/lex/latest/dg/API_PutBot.html#lex-PutBot-request-locale). Default is `en-US`.
        #[builder(into, default)]
        pub locale: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the bot that you want to create, case sensitive. Must be between 2 and 50 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Determines the threshold where Amazon Lex will insert the AMAZON.FallbackIntent, AMAZON.KendraSearchIntent, or both when returning alternative intents in a PostContent or PostText response. AMAZON.FallbackIntent and AMAZON.KendraSearchIntent are only inserted if they are configured for the bot. For more information see [Amazon Lex Bot PutBot API Docs](https://docs.aws.amazon.com/lex/latest/dg/API_PutBot.html#lex-PutBot-request-nluIntentConfidenceThreshold) This value requires `enable_model_improvements` to be set to `true` and the default is `0`. Must be a float between 0 and 1.
        #[builder(into, default)]
        pub nlu_intent_confidence_threshold: pulumi_wasm_rust::Output<Option<f64>>,
        /// If you set the `process_behavior` element to `BUILD`, Amazon Lex builds the bot so that it can be run. If you set the element to `SAVE` Amazon Lex saves the bot, but doesn't build it. Default is `SAVE`.
        #[builder(into, default)]
        pub process_behavior: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Polly voice ID that you want Amazon Lex to use for voice interactions with the user. The locale configured for the voice must match the locale of the bot. For more information, see [Available Voices](http://docs.aws.amazon.com/polly/latest/dg/voicelist.html) in the Amazon Polly Developer Guide.
        #[builder(into, default)]
        pub voice_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BotResult {
        /// The message that Amazon Lex uses to abort a conversation. Attributes are documented under statement.
        pub abort_statement: pulumi_wasm_rust::Output<
            super::super::types::lex::BotAbortStatement,
        >,
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Checksum identifying the version of the bot that was created. The checksum is not
        /// included as an argument because the resource will add it automatically when updating the bot.
        pub checksum: pulumi_wasm_rust::Output<String>,
        /// By specifying true, you confirm that your use of Amazon Lex is related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA. For more information see the [Amazon Lex FAQ](https://aws.amazon.com/lex/faqs#data-security) and the [Amazon Lex PutBot API Docs](https://docs.aws.amazon.com/lex/latest/dg/API_PutBot.html#lex-PutBot-request-childDirected).
        pub child_directed: pulumi_wasm_rust::Output<bool>,
        /// The message that Amazon Lex uses when it doesn't understand the user's request. Attributes are documented under prompt.
        pub clarification_prompt: pulumi_wasm_rust::Output<
            Option<super::super::types::lex::BotClarificationPrompt>,
        >,
        /// Determines if a new bot version is created when the initial resource is created and on each update. Defaults to `false`.
        pub create_version: pulumi_wasm_rust::Output<Option<bool>>,
        /// The date when the bot version was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// A description of the bot. Must be less than or equal to 200 characters in length.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// When set to true user utterances are sent to Amazon Comprehend for sentiment analysis. If you don't specify detectSentiment, the default is `false`.
        pub detect_sentiment: pulumi_wasm_rust::Output<Option<bool>>,
        /// Set to `true` to enable access to natural language understanding improvements. When you set the `enable_model_improvements` parameter to true you can use the `nlu_intent_confidence_threshold` parameter to configure confidence scores. For more information, see [Confidence Scores](https://docs.aws.amazon.com/lex/latest/dg/confidence-scores.html). You can only set the `enable_model_improvements` parameter in certain Regions. If you set the parameter to true, your bot has access to accuracy improvements. For more information see the [Amazon Lex Bot PutBot API Docs](https://docs.aws.amazon.com/lex/latest/dg/API_PutBot.html#lex-PutBot-request-enableModelImprovements).
        pub enable_model_improvements: pulumi_wasm_rust::Output<Option<bool>>,
        /// If status is FAILED, Amazon Lex provides the reason that it failed to build the bot.
        pub failure_reason: pulumi_wasm_rust::Output<String>,
        /// The maximum time in seconds that Amazon Lex retains the data gathered in a conversation. Default is `300`. Must be a number between 60 and 86400 (inclusive).
        pub idle_session_ttl_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// A set of Intent objects. Each intent represents a command that a user can express. Attributes are documented under intent. Can have up to 250 Intent objects.
        pub intents: pulumi_wasm_rust::Output<Vec<super::super::types::lex::BotIntent>>,
        /// The date when the $LATEST version of this bot was updated.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// Specifies the target locale for the bot. Any intent used in the bot must be compatible with the locale of the bot. For available locales, see [Amazon Lex Bot PutBot API Docs](https://docs.aws.amazon.com/lex/latest/dg/API_PutBot.html#lex-PutBot-request-locale). Default is `en-US`.
        pub locale: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the bot that you want to create, case sensitive. Must be between 2 and 50 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Determines the threshold where Amazon Lex will insert the AMAZON.FallbackIntent, AMAZON.KendraSearchIntent, or both when returning alternative intents in a PostContent or PostText response. AMAZON.FallbackIntent and AMAZON.KendraSearchIntent are only inserted if they are configured for the bot. For more information see [Amazon Lex Bot PutBot API Docs](https://docs.aws.amazon.com/lex/latest/dg/API_PutBot.html#lex-PutBot-request-nluIntentConfidenceThreshold) This value requires `enable_model_improvements` to be set to `true` and the default is `0`. Must be a float between 0 and 1.
        pub nlu_intent_confidence_threshold: pulumi_wasm_rust::Output<Option<f64>>,
        /// If you set the `process_behavior` element to `BUILD`, Amazon Lex builds the bot so that it can be run. If you set the element to `SAVE` Amazon Lex saves the bot, but doesn't build it. Default is `SAVE`.
        pub process_behavior: pulumi_wasm_rust::Output<Option<String>>,
        /// When you send a request to create or update a bot, Amazon Lex sets the status response
        /// element to BUILDING. After Amazon Lex builds the bot, it sets status to READY. If Amazon Lex can't
        /// build the bot, it sets status to FAILED. Amazon Lex returns the reason for the failure in the
        /// failure_reason response element.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The version of the bot.
        pub version: pulumi_wasm_rust::Output<String>,
        /// The Amazon Polly voice ID that you want Amazon Lex to use for voice interactions with the user. The locale configured for the voice must match the locale of the bot. For more information, see [Available Voices](http://docs.aws.amazon.com/polly/latest/dg/voicelist.html) in the Amazon Polly Developer Guide.
        pub voice_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BotArgs) -> BotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let abort_statement_binding = args.abort_statement.get_inner();
        let child_directed_binding = args.child_directed.get_inner();
        let clarification_prompt_binding = args.clarification_prompt.get_inner();
        let create_version_binding = args.create_version.get_inner();
        let description_binding = args.description.get_inner();
        let detect_sentiment_binding = args.detect_sentiment.get_inner();
        let enable_model_improvements_binding = args
            .enable_model_improvements
            .get_inner();
        let idle_session_ttl_in_seconds_binding = args
            .idle_session_ttl_in_seconds
            .get_inner();
        let intents_binding = args.intents.get_inner();
        let locale_binding = args.locale.get_inner();
        let name_binding = args.name.get_inner();
        let nlu_intent_confidence_threshold_binding = args
            .nlu_intent_confidence_threshold
            .get_inner();
        let process_behavior_binding = args.process_behavior.get_inner();
        let voice_id_binding = args.voice_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lex/bot:Bot".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "abortStatement".into(),
                    value: &abort_statement_binding,
                },
                register_interface::ObjectField {
                    name: "childDirected".into(),
                    value: &child_directed_binding,
                },
                register_interface::ObjectField {
                    name: "clarificationPrompt".into(),
                    value: &clarification_prompt_binding,
                },
                register_interface::ObjectField {
                    name: "createVersion".into(),
                    value: &create_version_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "detectSentiment".into(),
                    value: &detect_sentiment_binding,
                },
                register_interface::ObjectField {
                    name: "enableModelImprovements".into(),
                    value: &enable_model_improvements_binding,
                },
                register_interface::ObjectField {
                    name: "idleSessionTtlInSeconds".into(),
                    value: &idle_session_ttl_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "intents".into(),
                    value: &intents_binding,
                },
                register_interface::ObjectField {
                    name: "locale".into(),
                    value: &locale_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nluIntentConfidenceThreshold".into(),
                    value: &nlu_intent_confidence_threshold_binding,
                },
                register_interface::ObjectField {
                    name: "processBehavior".into(),
                    value: &process_behavior_binding,
                },
                register_interface::ObjectField {
                    name: "voiceId".into(),
                    value: &voice_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "abortStatement".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "checksum".into(),
                },
                register_interface::ResultField {
                    name: "childDirected".into(),
                },
                register_interface::ResultField {
                    name: "clarificationPrompt".into(),
                },
                register_interface::ResultField {
                    name: "createVersion".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "detectSentiment".into(),
                },
                register_interface::ResultField {
                    name: "enableModelImprovements".into(),
                },
                register_interface::ResultField {
                    name: "failureReason".into(),
                },
                register_interface::ResultField {
                    name: "idleSessionTtlInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "intents".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedDate".into(),
                },
                register_interface::ResultField {
                    name: "locale".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nluIntentConfidenceThreshold".into(),
                },
                register_interface::ResultField {
                    name: "processBehavior".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "voiceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BotResult {
            abort_statement: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("abortStatement").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            checksum: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checksum").unwrap(),
            ),
            child_directed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("childDirected").unwrap(),
            ),
            clarification_prompt: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clarificationPrompt").unwrap(),
            ),
            create_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createVersion").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            detect_sentiment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("detectSentiment").unwrap(),
            ),
            enable_model_improvements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableModelImprovements").unwrap(),
            ),
            failure_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failureReason").unwrap(),
            ),
            idle_session_ttl_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleSessionTtlInSeconds").unwrap(),
            ),
            intents: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("intents").unwrap(),
            ),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedDate").unwrap(),
            ),
            locale: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locale").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nlu_intent_confidence_threshold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nluIntentConfidenceThreshold").unwrap(),
            ),
            process_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("processBehavior").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            voice_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("voiceId").unwrap(),
            ),
        }
    }
}