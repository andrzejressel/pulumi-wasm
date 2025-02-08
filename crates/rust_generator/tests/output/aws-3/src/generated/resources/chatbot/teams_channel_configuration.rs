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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TeamsChannelConfigurationArgs,
    ) -> TeamsChannelConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let channel_id_binding = args.channel_id.get_output(context).get_inner();
        let channel_name_binding = args.channel_name.get_output(context).get_inner();
        let configuration_name_binding = args
            .configuration_name
            .get_output(context)
            .get_inner();
        let guardrail_policy_arns_binding = args
            .guardrail_policy_arns
            .get_output(context)
            .get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_output(context).get_inner();
        let logging_level_binding = args.logging_level.get_output(context).get_inner();
        let sns_topic_arns_binding = args.sns_topic_arns.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let team_id_binding = args.team_id.get_output(context).get_inner();
        let team_name_binding = args.team_name.get_output(context).get_inner();
        let tenant_id_binding = args.tenant_id.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let user_authorization_required_binding = args
            .user_authorization_required
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chatbot/teamsChannelConfiguration:TeamsChannelConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "channelId".into(),
                    value: &channel_id_binding,
                },
                register_interface::ObjectField {
                    name: "channelName".into(),
                    value: &channel_name_binding,
                },
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
                    name: "snsTopicArns".into(),
                    value: &sns_topic_arns_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "teamId".into(),
                    value: &team_id_binding,
                },
                register_interface::ObjectField {
                    name: "teamName".into(),
                    value: &team_name_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
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
        TeamsChannelConfigurationResult {
            channel_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("channelId"),
            ),
            channel_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("channelName"),
            ),
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
            sns_topic_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snsTopicArns"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            team_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("teamId"),
            ),
            team_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("teamName"),
            ),
            tenant_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tenantId"),
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
