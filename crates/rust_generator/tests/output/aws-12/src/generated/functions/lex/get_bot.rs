#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_bot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBotArgs {
        /// Name of the bot. The name is case sensitive.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version or alias of the bot.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBotResult {
        /// ARN of the bot.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Checksum of the bot used to identify a specific revision of the bot's `$LATEST` version.
        pub checksum: pulumi_gestalt_rust::Output<String>,
        /// If this Amazon Lex Bot is related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA.
        pub child_directed: pulumi_gestalt_rust::Output<bool>,
        /// Date that the bot was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the bot.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// When set to true user utterances are sent to Amazon Comprehend for sentiment analysis.
        pub detect_sentiment: pulumi_gestalt_rust::Output<bool>,
        /// Set to true if natural language understanding improvements are enabled.
        pub enable_model_improvements: pulumi_gestalt_rust::Output<bool>,
        /// If the `status` is `FAILED`, the reason why the bot failed to build.
        pub failure_reason: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The maximum time in seconds that Amazon Lex retains the data gathered in a conversation.
        pub idle_session_ttl_in_seconds: pulumi_gestalt_rust::Output<i32>,
        /// Date that the bot was updated.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// Target locale for the bot. Any intent used in the bot must be compatible with the locale of the bot.
        pub locale: pulumi_gestalt_rust::Output<String>,
        /// Name of the bot, case sensitive.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The threshold where Amazon Lex will insert the AMAZON.FallbackIntent, AMAZON.KendraSearchIntent, or both when returning alternative intents in a PostContent or PostText response. AMAZON.FallbackIntent and AMAZON.KendraSearchIntent are only inserted if they are configured for the bot.
        pub nlu_intent_confidence_threshold: pulumi_gestalt_rust::Output<f64>,
        /// Status of the bot.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Version of the bot. For a new bot, the version is always `$LATEST`.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Polly voice ID that the Amazon Lex Bot uses for voice interactions with the user.
        pub voice_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBotArgs,
    ) -> GetBotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lex/getBot:getBot".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBotResult {
            arn: o.get_field("arn"),
            checksum: o.get_field("checksum"),
            child_directed: o.get_field("childDirected"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            detect_sentiment: o.get_field("detectSentiment"),
            enable_model_improvements: o.get_field("enableModelImprovements"),
            failure_reason: o.get_field("failureReason"),
            id: o.get_field("id"),
            idle_session_ttl_in_seconds: o.get_field("idleSessionTtlInSeconds"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            locale: o.get_field("locale"),
            name: o.get_field("name"),
            nlu_intent_confidence_threshold: o.get_field("nluIntentConfidenceThreshold"),
            status: o.get_field("status"),
            version: o.get_field("version"),
            voice_id: o.get_field("voiceId"),
        }
    }
}
