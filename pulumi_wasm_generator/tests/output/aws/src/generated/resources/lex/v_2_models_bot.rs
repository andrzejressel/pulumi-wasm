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
///               Sid:
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
pub mod v_2_models_bot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2modelsBotArgs {
        /// Provides information on additional privacy protections Amazon Lex should use with the bot's data. See `data_privacy`
        #[builder(into, default)]
        pub data_privacies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsBotDataPrivacy>>,
        >,
        /// Description of the bot. It appears in lists to help you identify a particular bot.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Time, in seconds, that Amazon Lex should keep information about a user's conversation with the bot. You can specify between 60 (1 minute) and 86,400 (24 hours) seconds.
        #[builder(into)]
        pub idle_session_ttl_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// List of bot members in a network to be created. See `bot_members`.
        #[builder(into, default)]
        pub members: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsBotMember>>,
        >,
        /// Name of the bot. The bot name must be unique in the account that creates the bot. Type String. Length Constraints: Minimum length of 1. Maximum length of 100.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of an IAM role that has permission to access the bot.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// List of tags to add to the bot. You can only add tags when you create a bot.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags to add to the test alias for a bot. You can only add tags when you create a bot.
        #[builder(into, default)]
        pub test_bot_alias_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::lex::V2ModelsBotTimeouts>,
        >,
        /// Type of a bot to create. Possible values are `"Bot"` and `"BotNetwork"`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct V2modelsBotResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Provides information on additional privacy protections Amazon Lex should use with the bot's data. See `data_privacy`
        pub data_privacies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsBotDataPrivacy>>,
        >,
        /// Description of the bot. It appears in lists to help you identify a particular bot.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Time, in seconds, that Amazon Lex should keep information about a user's conversation with the bot. You can specify between 60 (1 minute) and 86,400 (24 hours) seconds.
        pub idle_session_ttl_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// List of bot members in a network to be created. See `bot_members`.
        pub members: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::lex::V2ModelsBotMember>>,
        >,
        /// Name of the bot. The bot name must be unique in the account that creates the bot. Type String. Length Constraints: Minimum length of 1. Maximum length of 100.
        pub name: pulumi_wasm_rust::Output<String>,
        /// ARN of an IAM role that has permission to access the bot.
        ///
        /// The following arguments are optional:
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// List of tags to add to the bot. You can only add tags when you create a bot.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of tags to add to the test alias for a bot. You can only add tags when you create a bot.
        pub test_bot_alias_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::lex::V2ModelsBotTimeouts>,
        >,
        /// Type of a bot to create. Possible values are `"Bot"` and `"BotNetwork"`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: V2modelsBotArgs) -> V2modelsBotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_privacies_binding = args.data_privacies.get_inner();
        let description_binding = args.description.get_inner();
        let idle_session_ttl_in_seconds_binding = args
            .idle_session_ttl_in_seconds
            .get_inner();
        let members_binding = args.members.get_inner();
        let name_binding = args.name.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let test_bot_alias_tags_binding = args.test_bot_alias_tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lex/v2modelsBot:V2modelsBot".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataPrivacies".into(),
                    value: &data_privacies_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "idleSessionTtlInSeconds".into(),
                    value: &idle_session_ttl_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "members".into(),
                    value: &members_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "testBotAliasTags".into(),
                    value: &test_bot_alias_tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "dataPrivacies".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "idleSessionTtlInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "members".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "testBotAliasTags".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        V2modelsBotResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            data_privacies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataPrivacies").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            idle_session_ttl_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleSessionTtlInSeconds").unwrap(),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("members").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            test_bot_alias_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("testBotAliasTags").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}