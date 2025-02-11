/// Resource for managing an AWS Lex V2 Models Bot.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:lex:V2modelsBot
///     properties:
///       name: example
///       description: Example description
///       dataPrivacies:
///         - childDirected: false
///       idleSessionTtlInSeconds: 60
///       roleArn: ${exampleRole.arn}
///       type: Bot
///       tags:
///         foo: bar
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Sid: ""
///               Principal:
///                 Service: lexv2.amazonaws.com
///       tags:
///         created_by: aws
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lex V2 Models Bot using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:lex/v2modelsBot:V2modelsBot example bot-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_2_models_bot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2modelsBotArgs {
        /// Provides information on additional privacy protections Amazon Lex should use with the bot's data. See `data_privacy`
        #[builder(into, default)]
        pub data_privacies: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lex::V2ModelsBotDataPrivacy>>,
        >,
        /// Description of the bot. It appears in lists to help you identify a particular bot.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Time, in seconds, that Amazon Lex should keep information about a user's conversation with the bot. You can specify between 60 (1 minute) and 86,400 (24 hours) seconds.
        #[builder(into)]
        pub idle_session_ttl_in_seconds: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// List of bot members in a network to be created. See `bot_members`.
        #[builder(into, default)]
        pub members: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lex::V2ModelsBotMember>>,
        >,
        /// Name of the bot. The bot name must be unique in the account that creates the bot. Type String. Length Constraints: Minimum length of 1. Maximum length of 100.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of an IAM role that has permission to access the bot.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of tags to add to the bot. You can only add tags when you create a bot.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags to add to the test alias for a bot. You can only add tags when you create a bot.
        #[builder(into, default)]
        pub test_bot_alias_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsBotTimeouts>,
        >,
        /// Type of a bot to create. Possible values are `"Bot"` and `"BotNetwork"`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct V2modelsBotResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Provides information on additional privacy protections Amazon Lex should use with the bot's data. See `data_privacy`
        pub data_privacies: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsBotDataPrivacy>>,
        >,
        /// Description of the bot. It appears in lists to help you identify a particular bot.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Time, in seconds, that Amazon Lex should keep information about a user's conversation with the bot. You can specify between 60 (1 minute) and 86,400 (24 hours) seconds.
        pub idle_session_ttl_in_seconds: pulumi_gestalt_rust::Output<i32>,
        /// List of bot members in a network to be created. See `bot_members`.
        pub members: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsBotMember>>,
        >,
        /// Name of the bot. The bot name must be unique in the account that creates the bot. Type String. Length Constraints: Minimum length of 1. Maximum length of 100.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ARN of an IAM role that has permission to access the bot.
        ///
        /// The following arguments are optional:
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// List of tags to add to the bot. You can only add tags when you create a bot.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of tags to add to the test alias for a bot. You can only add tags when you create a bot.
        pub test_bot_alias_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsBotTimeouts>,
        >,
        /// Type of a bot to create. Possible values are `"Bot"` and `"BotNetwork"`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: V2modelsBotArgs,
    ) -> V2modelsBotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_privacies_binding = args.data_privacies.get_output(context);
        let description_binding = args.description.get_output(context);
        let idle_session_ttl_in_seconds_binding = args
            .idle_session_ttl_in_seconds
            .get_output(context);
        let members_binding = args.members.get_output(context);
        let name_binding = args.name.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let test_bot_alias_tags_binding = args.test_bot_alias_tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lex/v2modelsBot:V2modelsBot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataPrivacies".into(),
                    value: &data_privacies_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idleSessionTtlInSeconds".into(),
                    value: &idle_session_ttl_in_seconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "members".into(),
                    value: &members_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "testBotAliasTags".into(),
                    value: &test_bot_alias_tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        V2modelsBotResult {
            arn: o.get_field("arn"),
            data_privacies: o.get_field("dataPrivacies"),
            description: o.get_field("description"),
            idle_session_ttl_in_seconds: o.get_field("idleSessionTtlInSeconds"),
            members: o.get_field("members"),
            name: o.get_field("name"),
            role_arn: o.get_field("roleArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            test_bot_alias_tags: o.get_field("testBotAliasTags"),
            timeouts: o.get_field("timeouts"),
            type_: o.get_field("type"),
        }
    }
}
