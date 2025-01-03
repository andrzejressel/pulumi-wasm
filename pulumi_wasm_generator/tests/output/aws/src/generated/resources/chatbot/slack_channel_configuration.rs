/// Resource for managing an AWS Chatbot Slack Channel Configuration.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:chatbot:SlackChannelConfiguration
///     properties:
///       configurationName: min-slaka-kanal
///       iamRoleArn: ${testAwsIamRole.arn}
///       slackChannelId: C07EZ1ABC23
///       slackTeamId: T07EA123LEP
///       tags:
///         Name: min-slaka-kanal
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Chatbot Slack Channel Configuration using the `chat_configuration_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:chatbot/slackChannelConfiguration:SlackChannelConfiguration example arn:aws:chatbot::123456789012:chat-configuration/slack-channel/min-slaka-kanal
/// ```
pub mod slack_channel_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SlackChannelConfigurationArgs {
        /// Name of the Slack channel configuration.
        #[builder(into)]
        pub configuration_name: pulumi_wasm_rust::Output<String>,
        /// List of IAM policy ARNs that are applied as channel guardrails. The AWS managed `AdministratorAccess` policy is applied by default if this is not set.
        #[builder(into, default)]
        pub guardrail_policy_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// User-defined role that AWS Chatbot assumes. This is not the service-linked role.
        #[builder(into)]
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// Logging levels include `ERROR`, `INFO`, or `NONE`.
        #[builder(into, default)]
        pub logging_level: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the Slack channel. For example, `C07EZ1ABC23`.
        #[builder(into)]
        pub slack_channel_id: pulumi_wasm_rust::Output<String>,
        /// ID of the Slack workspace authorized with AWS Chatbot. For example, `T07EA123LEP`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub slack_team_id: pulumi_wasm_rust::Output<String>,
        /// ARNs of the SNS topics that deliver notifications to AWS Chatbot.
        #[builder(into, default)]
        pub sns_topic_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::chatbot::SlackChannelConfigurationTimeouts>,
        >,
        /// Enables use of a user role requirement in your chat configuration.
        #[builder(into, default)]
        pub user_authorization_required: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SlackChannelConfigurationResult {
        /// ARN of the Slack channel configuration.
        pub chat_configuration_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the Slack channel configuration.
        pub configuration_name: pulumi_wasm_rust::Output<String>,
        /// List of IAM policy ARNs that are applied as channel guardrails. The AWS managed `AdministratorAccess` policy is applied by default if this is not set.
        pub guardrail_policy_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// User-defined role that AWS Chatbot assumes. This is not the service-linked role.
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// Logging levels include `ERROR`, `INFO`, or `NONE`.
        pub logging_level: pulumi_wasm_rust::Output<String>,
        /// ID of the Slack channel. For example, `C07EZ1ABC23`.
        pub slack_channel_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Slack channel.
        pub slack_channel_name: pulumi_wasm_rust::Output<String>,
        /// ID of the Slack workspace authorized with AWS Chatbot. For example, `T07EA123LEP`.
        ///
        /// The following arguments are optional:
        pub slack_team_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Slack team.
        pub slack_team_name: pulumi_wasm_rust::Output<String>,
        /// ARNs of the SNS topics that deliver notifications to AWS Chatbot.
        pub sns_topic_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::chatbot::SlackChannelConfigurationTimeouts>,
        >,
        /// Enables use of a user role requirement in your chat configuration.
        pub user_authorization_required: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SlackChannelConfigurationArgs,
    ) -> SlackChannelConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_name_binding = args.configuration_name.get_inner();
        let guardrail_policy_arns_binding = args.guardrail_policy_arns.get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_inner();
        let logging_level_binding = args.logging_level.get_inner();
        let slack_channel_id_binding = args.slack_channel_id.get_inner();
        let slack_team_id_binding = args.slack_team_id.get_inner();
        let sns_topic_arns_binding = args.sns_topic_arns.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let user_authorization_required_binding = args
            .user_authorization_required
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chatbot/slackChannelConfiguration:SlackChannelConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationName".into(),
                    value: &configuration_name_binding,
                },
                register_interface::ObjectField {
                    name: "guardrailPolicyArns".into(),
                    value: &guardrail_policy_arns_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "loggingLevel".into(),
                    value: &logging_level_binding,
                },
                register_interface::ObjectField {
                    name: "slackChannelId".into(),
                    value: &slack_channel_id_binding,
                },
                register_interface::ObjectField {
                    name: "slackTeamId".into(),
                    value: &slack_team_id_binding,
                },
                register_interface::ObjectField {
                    name: "snsTopicArns".into(),
                    value: &sns_topic_arns_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "userAuthorizationRequired".into(),
                    value: &user_authorization_required_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "chatConfigurationArn".into(),
                },
                register_interface::ResultField {
                    name: "configurationName".into(),
                },
                register_interface::ResultField {
                    name: "guardrailPolicyArns".into(),
                },
                register_interface::ResultField {
                    name: "iamRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "loggingLevel".into(),
                },
                register_interface::ResultField {
                    name: "slackChannelId".into(),
                },
                register_interface::ResultField {
                    name: "slackChannelName".into(),
                },
                register_interface::ResultField {
                    name: "slackTeamId".into(),
                },
                register_interface::ResultField {
                    name: "slackTeamName".into(),
                },
                register_interface::ResultField {
                    name: "snsTopicArns".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "userAuthorizationRequired".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SlackChannelConfigurationResult {
            chat_configuration_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("chatConfigurationArn").unwrap(),
            ),
            configuration_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationName").unwrap(),
            ),
            guardrail_policy_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guardrailPolicyArns").unwrap(),
            ),
            iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoleArn").unwrap(),
            ),
            logging_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingLevel").unwrap(),
            ),
            slack_channel_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("slackChannelId").unwrap(),
            ),
            slack_channel_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("slackChannelName").unwrap(),
            ),
            slack_team_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("slackTeamId").unwrap(),
            ),
            slack_team_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("slackTeamName").unwrap(),
            ),
            sns_topic_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snsTopicArns").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            user_authorization_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userAuthorizationRequired").unwrap(),
            ),
        }
    }
}
