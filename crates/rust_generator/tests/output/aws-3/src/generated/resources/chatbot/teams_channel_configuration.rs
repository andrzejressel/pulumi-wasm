/// Resource for managing an AWS Chatbot Microsoft Teams Channel Configuration.
///
/// > **NOTE:** We provide this resource on a best-effort basis. If you are able to test it and find it useful, we welcome your input at GitHub.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:chatbot:TeamsChannelConfiguration
///     properties:
///       channelId: C07EZ1ABC23
///       configurationName: mitt-lags-kanal
///       iamRoleArn: ${testAwsIamRole.arn}
///       teamId: 74361522-da01-538d-aa2e-ac7918c6bb92
///       tenantId: '1234'
///       tags:
///         Name: mitt-lags-kanal
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Chatbot Microsoft Teams Channel Configuration using the `team_id`. For example:
///
/// ```sh
/// $ pulumi import aws:chatbot/teamsChannelConfiguration:TeamsChannelConfiguration example 5f4f15d2-b958-522a-8333-124aa8bf0925
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod teams_channel_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TeamsChannelConfigurationArgs {
        /// ID of the Microsoft Teams channel.
        #[builder(into)]
        pub channel_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Microsoft Teams channel.
        #[builder(into, default)]
        pub channel_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the Microsoft Teams channel configuration.
        #[builder(into)]
        pub configuration_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of IAM policy ARNs that are applied as channel guardrails. The AWS managed `AdministratorAccess` policy is applied by default if this is not set.
        #[builder(into, default)]
        pub guardrail_policy_arns: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// ARN of the IAM role that defines the permissions for AWS Chatbot. This is a user-defined role that AWS Chatbot will assume. This is not the service-linked role.
        #[builder(into)]
        pub iam_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Logging levels include `ERROR`, `INFO`, or `NONE`.
        #[builder(into, default)]
        pub logging_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARNs of the SNS topics that deliver notifications to AWS Chatbot.
        #[builder(into, default)]
        pub sns_topic_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the Microsoft Team authorized with AWS Chatbot. To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console.
        #[builder(into)]
        pub team_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Microsoft Teams team.
        #[builder(into, default)]
        pub team_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the Microsoft Teams tenant.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::chatbot::TeamsChannelConfigurationTimeouts>,
        >,
        /// Enables use of a user role requirement in your chat configuration.
        #[builder(into, default)]
        pub user_authorization_required: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct TeamsChannelConfigurationResult {
        /// ID of the Microsoft Teams channel.
        pub channel_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the Microsoft Teams channel.
        pub channel_name: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Microsoft Teams channel configuration.
        pub chat_configuration_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the Microsoft Teams channel configuration.
        pub configuration_name: pulumi_gestalt_rust::Output<String>,
        /// List of IAM policy ARNs that are applied as channel guardrails. The AWS managed `AdministratorAccess` policy is applied by default if this is not set.
        pub guardrail_policy_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ARN of the IAM role that defines the permissions for AWS Chatbot. This is a user-defined role that AWS Chatbot will assume. This is not the service-linked role.
        pub iam_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Logging levels include `ERROR`, `INFO`, or `NONE`.
        pub logging_level: pulumi_gestalt_rust::Output<String>,
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
        /// ID of the Microsoft Team authorized with AWS Chatbot. To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console.
        pub team_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the Microsoft Teams team.
        pub team_name: pulumi_gestalt_rust::Output<String>,
        /// ID of the Microsoft Teams tenant.
        ///
        /// The following arguments are optional:
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::chatbot::TeamsChannelConfigurationTimeouts>,
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
        args: TeamsChannelConfigurationArgs,
    ) -> TeamsChannelConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let channel_id_binding = args.channel_id.get_output(context);
        let channel_name_binding = args.channel_name.get_output(context);
        let configuration_name_binding = args.configuration_name.get_output(context);
        let guardrail_policy_arns_binding = args
            .guardrail_policy_arns
            .get_output(context);
        let iam_role_arn_binding = args.iam_role_arn.get_output(context);
        let logging_level_binding = args.logging_level.get_output(context);
        let sns_topic_arns_binding = args.sns_topic_arns.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let team_id_binding = args.team_id.get_output(context);
        let team_name_binding = args.team_name.get_output(context);
        let tenant_id_binding = args.tenant_id.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let user_authorization_required_binding = args
            .user_authorization_required
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:chatbot/teamsChannelConfiguration:TeamsChannelConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "channelId".into(),
                    value: channel_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "channelName".into(),
                    value: channel_name_binding.get_id(),
                },
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
                    name: "snsTopicArns".into(),
                    value: sns_topic_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "teamId".into(),
                    value: team_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "teamName".into(),
                    value: team_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantId".into(),
                    value: tenant_id_binding.get_id(),
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
        TeamsChannelConfigurationResult {
            channel_id: o.get_field("channelId"),
            channel_name: o.get_field("channelName"),
            chat_configuration_arn: o.get_field("chatConfigurationArn"),
            configuration_name: o.get_field("configurationName"),
            guardrail_policy_arns: o.get_field("guardrailPolicyArns"),
            iam_role_arn: o.get_field("iamRoleArn"),
            logging_level: o.get_field("loggingLevel"),
            sns_topic_arns: o.get_field("snsTopicArns"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            team_id: o.get_field("teamId"),
            team_name: o.get_field("teamName"),
            tenant_id: o.get_field("tenantId"),
            timeouts: o.get_field("timeouts"),
            user_authorization_required: o.get_field("userAuthorizationRequired"),
        }
    }
}
