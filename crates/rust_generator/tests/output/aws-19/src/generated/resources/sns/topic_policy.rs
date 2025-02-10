/// Provides an SNS topic policy resource
///
/// > **NOTE:** If a Principal is specified as just an AWS account ID rather than an ARN, AWS silently converts it to the ARN for the root user, causing future deployments to differ. To avoid this problem, just specify the full ARN, e.g. `arn:aws:iam::123456789012:root`
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:sns:Topic
///     properties:
///       name: my-topic-with-policy
///   default:
///     type: aws:sns:TopicPolicy
///     properties:
///       arn: ${test.arn}
///       policy: ${snsTopicPolicy.json}
/// variables:
///   snsTopicPolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         policyId: __default_policy_ID
///         statements:
///           - actions:
///               - SNS:Subscribe
///               - SNS:SetTopicAttributes
///               - SNS:RemovePermission
///               - SNS:Receive
///               - SNS:Publish
///               - SNS:ListSubscriptionsByTopic
///               - SNS:GetTopicAttributes
///               - SNS:DeleteTopic
///               - SNS:AddPermission
///             conditions:
///               - test: StringEquals
///                 variable: AWS:SourceOwner
///                 values:
///                   - ${["account-id"]}
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '*'
///             resources:
///               - ${test.arn}
///             sid: __default_statement_ID
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SNS Topic Policy using the topic ARN. For example:
///
/// ```sh
/// $ pulumi import aws:sns/topicPolicy:TopicPolicy user_updates arn:aws:sns:us-west-2:123456789012:my-topic
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod topic_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicPolicyArgs {
        /// The ARN of the SNS topic
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The fully-formed AWS policy as JSON.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TopicPolicyResult {
        /// The ARN of the SNS topic
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The AWS Account ID of the SNS topic owner
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// The fully-formed AWS policy as JSON.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TopicPolicyArgs,
    ) -> TopicPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sns/topicPolicy:TopicPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TopicPolicyResult {
            arn: o.get_field("arn"),
            owner: o.get_field("owner"),
            policy: o.get_field("policy"),
        }
    }
}
