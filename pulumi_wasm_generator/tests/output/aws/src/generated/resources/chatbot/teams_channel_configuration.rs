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
pub mod teams_channel_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TeamsChannelConfigurationArgs {
        /// ID of the Microsoft Teams channel.
        #[builder(into)]
        pub channel_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Microsoft Teams channel.
        #[builder(into, default)]
        pub channel_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the Microsoft Teams channel configuration.
        #[builder(into)]
        pub configuration_name: pulumi_wasm_rust::Output<String>,
        /// List of IAM policy ARNs that are applied as channel guardrails. The AWS managed `AdministratorAccess` policy is applied by default if this is not set.
        #[builder(into, default)]
        pub guardrail_policy_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// ARN of the IAM role that defines the permissions for AWS Chatbot. This is a user-defined role that AWS Chatbot will assume. This is not the service-linked role.
        #[builder(into)]
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// Logging levels include `ERROR`, `INFO`, or `NONE`.
        #[builder(into, default)]
        pub logging_level: pulumi_wasm_rust::Output<Option<String>>,
        /// ARNs of the SNS topics that deliver notifications to AWS Chatbot.
        #[builder(into, default)]
        pub sns_topic_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the Microsoft Team authorized with AWS Chatbot. To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console.
        #[builder(into)]
        pub team_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Microsoft Teams team.
        #[builder(into, default)]
        pub team_name: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the Microsoft Teams tenant.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub tenant_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::chatbot::TeamsChannelConfigurationTimeouts>,
        >,
        /// Enables use of a user role requirement in your chat configuration.
        #[builder(into, default)]
        pub user_authorization_required: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct TeamsChannelConfigurationResult {
        /// ID of the Microsoft Teams channel.
        pub channel_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Microsoft Teams channel.
        pub channel_name: pulumi_wasm_rust::Output<String>,
        /// ARN of the Microsoft Teams channel configuration.
        pub chat_configuration_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the Microsoft Teams channel configuration.
        pub configuration_name: pulumi_wasm_rust::Output<String>,
        /// List of IAM policy ARNs that are applied as channel guardrails. The AWS managed `AdministratorAccess` policy is applied by default if this is not set.
        pub guardrail_policy_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// ARN of the IAM role that defines the permissions for AWS Chatbot. This is a user-defined role that AWS Chatbot will assume. This is not the service-linked role.
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// Logging levels include `ERROR`, `INFO`, or `NONE`.
        pub logging_level: pulumi_wasm_rust::Output<String>,
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
        /// ID of the Microsoft Team authorized with AWS Chatbot. To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console.
        pub team_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Microsoft Teams team.
        pub team_name: pulumi_wasm_rust::Output<String>,
        /// ID of the Microsoft Teams tenant.
        ///
        /// The following arguments are optional:
        pub tenant_id: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::chatbot::TeamsChannelConfigurationTimeouts>,
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
        args: TeamsChannelConfigurationArgs,
    ) -> TeamsChannelConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let channel_id_binding = args.channel_id.get_inner();
        let channel_name_binding = args.channel_name.get_inner();
        let configuration_name_binding = args.configuration_name.get_inner();
        let guardrail_policy_arns_binding = args.guardrail_policy_arns.get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_inner();
        let logging_level_binding = args.logging_level.get_inner();
        let sns_topic_arns_binding = args.sns_topic_arns.get_inner();
        let tags_binding = args.tags.get_inner();
        let team_id_binding = args.team_id.get_inner();
        let team_name_binding = args.team_name.get_inner();
        let tenant_id_binding = args.tenant_id.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let user_authorization_required_binding = args
            .user_authorization_required
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chatbot/teamsChannelConfiguration:TeamsChannelConfiguration"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "channelId".into(),
                },
                register_interface::ResultField {
                    name: "channelName".into(),
                },
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
                    name: "snsTopicArns".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "teamId".into(),
                },
                register_interface::ResultField {
                    name: "teamName".into(),
                },
                register_interface::ResultField {
                    name: "tenantId".into(),
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
        TeamsChannelConfigurationResult {
            channel_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("channelId").unwrap(),
            ),
            channel_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("channelName").unwrap(),
            ),
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
            sns_topic_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snsTopicArns").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            team_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("teamId").unwrap(),
            ),
            team_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("teamName").unwrap(),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantId").unwrap(),
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