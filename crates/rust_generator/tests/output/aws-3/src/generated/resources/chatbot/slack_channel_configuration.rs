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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod slack_channel_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SlackChannelConfigurationArgs {
        /// Name of the Slack channel configuration.
        #[builder(into)]
        pub configuration_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of IAM policy ARNs that are applied as channel guardrails. The AWS managed `AdministratorAccess` policy is applied by default if this is not set.
        #[builder(into, default)]
        pub guardrail_policy_arns: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// User-defined role that AWS Chatbot assumes. This is not the service-linked role.
        #[builder(into)]
        pub iam_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Logging levels include `ERROR`, `INFO`, or `NONE`.
        #[builder(into, default)]
        pub logging_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the Slack channel. For example, `C07EZ1ABC23`.
        #[builder(into)]
        pub slack_channel_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the Slack workspace authorized with AWS Chatbot. For example, `T07EA123LEP`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub slack_team_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARNs of the SNS topics that deliver notifications to AWS Chatbot.
        #[builder(into, default)]
        pub sns_topic_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::chatbot::SlackChannelConfigurationTimeouts>,
        >,
        /// Enables use of a user role requirement in your chat configuration.
        #[builder(into, default)]
        pub user_authorization_required: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct SlackChannelConfigurationResult {
        /// ARN of the Slack channel configuration.
        pub chat_configuration_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the Slack channel configuration.
        pub configuration_name: pulumi_gestalt_rust::Output<String>,
        /// List of IAM policy ARNs that are applied as channel guardrails. The AWS managed `AdministratorAccess` policy is applied by default if this is not set.
        pub guardrail_policy_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// User-defined role that AWS Chatbot assumes. This is not the service-linked role.
        pub iam_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Logging levels include `ERROR`, `INFO`, or `NONE`.
        pub logging_level: pulumi_gestalt_rust::Output<String>,
        /// ID of the Slack channel. For example, `C07EZ1ABC23`.
        pub slack_channel_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the Slack channel.
        pub slack_channel_name: pulumi_gestalt_rust::Output<String>,
        /// ID of the Slack workspace authorized with AWS Chatbot. For example, `T07EA123LEP`.
        ///
        /// The following arguments are optional:
        pub slack_team_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the Slack team.
        pub slack_team_name: pulumi_gestalt_rust::Output<String>,
        /// ARNs of the SNS topics that deliver notifications to AWS Chatbot.
        pub sns_topic_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::chatbot::SlackChannelConfigurationTimeouts>,
        >,
        /// Enables use of a user role requirement in your chat configuration.
        pub user_authorization_required: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SlackChannelConfigurationArgs,
    ) -> SlackChannelConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let configuration_name_binding_1 = args.configuration_name.get_output(context);
        let configuration_name_binding = configuration_name_binding_1.get_inner();
        let guardrail_policy_arns_binding_1 = args
            .guardrail_policy_arns
            .get_output(context);
        let guardrail_policy_arns_binding = guardrail_policy_arns_binding_1.get_inner();
        let iam_role_arn_binding_1 = args.iam_role_arn.get_output(context);
        let iam_role_arn_binding = iam_role_arn_binding_1.get_inner();
        let logging_level_binding_1 = args.logging_level.get_output(context);
        let logging_level_binding = logging_level_binding_1.get_inner();
        let slack_channel_id_binding_1 = args.slack_channel_id.get_output(context);
        let slack_channel_id_binding = slack_channel_id_binding_1.get_inner();
        let slack_team_id_binding_1 = args.slack_team_id.get_output(context);
        let slack_team_id_binding = slack_team_id_binding_1.get_inner();
        let sns_topic_arns_binding_1 = args.sns_topic_arns.get_output(context);
        let sns_topic_arns_binding = sns_topic_arns_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
        let user_authorization_required_binding_1 = args
            .user_authorization_required
            .get_output(context);
        let user_authorization_required_binding = user_authorization_required_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chatbot/slackChannelConfiguration:SlackChannelConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        SlackChannelConfigurationResult {
            chat_configuration_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("chatConfigurationArn"),
            ),
            configuration_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationName"),
            ),
            guardrail_policy_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("guardrailPolicyArns"),
            ),
            iam_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iamRoleArn"),
            ),
            logging_level: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingLevel"),
            ),
            slack_channel_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("slackChannelId"),
            ),
            slack_channel_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("slackChannelName"),
            ),
            slack_team_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("slackTeamId"),
            ),
            slack_team_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("slackTeamName"),
            ),
            sns_topic_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snsTopicArns"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            user_authorization_required: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userAuthorizationRequired"),
            ),
        }
    }
}
