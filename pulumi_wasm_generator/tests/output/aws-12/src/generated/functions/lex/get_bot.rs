pub mod get_bot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBotArgs {
        /// Name of the bot. The name is case sensitive.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Version or alias of the bot.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBotResult {
        /// ARN of the bot.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Checksum of the bot used to identify a specific revision of the bot's `$LATEST` version.
        pub checksum: pulumi_wasm_rust::Output<String>,
        /// If this Amazon Lex Bot is related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA.
        pub child_directed: pulumi_wasm_rust::Output<bool>,
        /// Date that the bot was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Description of the bot.
        pub description: pulumi_wasm_rust::Output<String>,
        /// When set to true user utterances are sent to Amazon Comprehend for sentiment analysis.
        pub detect_sentiment: pulumi_wasm_rust::Output<bool>,
        /// Set to true if natural language understanding improvements are enabled.
        pub enable_model_improvements: pulumi_wasm_rust::Output<bool>,
        /// If the `status` is `FAILED`, the reason why the bot failed to build.
        pub failure_reason: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The maximum time in seconds that Amazon Lex retains the data gathered in a conversation.
        pub idle_session_ttl_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// Date that the bot was updated.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// Target locale for the bot. Any intent used in the bot must be compatible with the locale of the bot.
        pub locale: pulumi_wasm_rust::Output<String>,
        /// Name of the bot, case sensitive.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The threshold where Amazon Lex will insert the AMAZON.FallbackIntent, AMAZON.KendraSearchIntent, or both when returning alternative intents in a PostContent or PostText response. AMAZON.FallbackIntent and AMAZON.KendraSearchIntent are only inserted if they are configured for the bot.
        pub nlu_intent_confidence_threshold: pulumi_wasm_rust::Output<f64>,
        /// Status of the bot.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Version of the bot. For a new bot, the version is always `$LATEST`.
        pub version: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Polly voice ID that the Amazon Lex Bot uses for voice interactions with the user.
        pub voice_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetBotArgs,
    ) -> GetBotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lex/getBot:getBot".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "idleSessionTtlInSeconds".into(),
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
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBotResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            checksum: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checksum").unwrap(),
            ),
            child_directed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("childDirected").unwrap(),
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            idle_session_ttl_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleSessionTtlInSeconds").unwrap(),
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
