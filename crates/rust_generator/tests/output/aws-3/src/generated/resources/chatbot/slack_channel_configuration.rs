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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SlackChannelConfigurationArgs,
    ) -> SlackChannelConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_name_binding = args.configuration_name.get_output(context);
        let guardrail_policy_arns_binding = args
            .guardrail_policy_arns
            .get_output(context);
        let iam_role_arn_binding = args.iam_role_arn.get_output(context);
        let logging_level_binding = args.logging_level.get_output(context);
        let slack_channel_id_binding = args.slack_channel_id.get_output(context);
        let slack_team_id_binding = args.slack_team_id.get_output(context);
        let sns_topic_arns_binding = args.sns_topic_arns.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let user_authorization_required_binding = args
            .user_authorization_required
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:chatbot/slackChannelConfiguration:SlackChannelConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationName".into(),
                    value: configuration_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "guardrailPolicyArns".into(),
                    value: guardrail_policy_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamRoleArn".into(),
                    value: iam_role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingLevel".into(),
                    value: logging_level_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "slackChannelId".into(),
                    value: slack_channel_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "slackTeamId".into(),
                    value: slack_team_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snsTopicArns".into(),
                    value: sns_topic_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userAuthorizationRequired".into(),
                    value: user_authorization_required_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SlackChannelConfigurationResult {
            chat_configuration_arn: o.get_field("chatConfigurationArn"),
            configuration_name: o.get_field("configurationName"),
            guardrail_policy_arns: o.get_field("guardrailPolicyArns"),
            iam_role_arn: o.get_field("iamRoleArn"),
            logging_level: o.get_field("loggingLevel"),
            slack_channel_id: o.get_field("slackChannelId"),
            slack_channel_name: o.get_field("slackChannelName"),
            slack_team_id: o.get_field("slackTeamId"),
            slack_team_name: o.get_field("slackTeamName"),
            sns_topic_arns: o.get_field("snsTopicArns"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            user_authorization_required: o.get_field("userAuthorizationRequired"),
        }
    }
}
