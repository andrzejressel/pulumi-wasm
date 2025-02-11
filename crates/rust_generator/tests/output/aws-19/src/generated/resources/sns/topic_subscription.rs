/// Provides a resource for subscribing to SNS topics. Requires that an SNS topic exist for the subscription to attach to. This resource allows you to automatically place messages sent to SNS topics in SQS queues, send them as HTTP(S) POST requests to a given endpoint, send SMS messages, or notify devices / applications. The most likely use case for provider users will probably be SQS queues.
///
/// > **NOTE:** If the SNS topic and SQS queue are in different AWS regions, the `aws.sns.TopicSubscription` must use an AWS provider that is in the same region as the SNS topic. If the `aws.sns.TopicSubscription` uses a provider with a different region than the SNS topic, this provider will fail to create the subscription.
///
/// > **NOTE:** Setup of cross-account subscriptions from SNS topics to SQS queues requires the provider to have access to BOTH accounts.
///
/// > **NOTE:** If an SNS topic and SQS queue are in different AWS accounts but the same region, the `aws.sns.TopicSubscription` must use the AWS provider for the account with the SQS queue. If `aws.sns.TopicSubscription` uses a Provider with a different account than the SQS queue, this provider creates the subscription but does not keep state and tries to re-create the subscription at every `apply`.
///
/// > **NOTE:** If an SNS topic and SQS queue are in different AWS accounts and different AWS regions, the subscription needs to be initiated from the account with the SQS queue but in the region of the SNS topic.
///
/// > **NOTE:** You cannot unsubscribe to a subscription that is pending confirmation. If you use `email`, `email-json`, or `http`/`https` (without auto-confirmation enabled), until the subscription is confirmed (e.g., outside of this provider), AWS does not allow this provider to delete / unsubscribe the subscription. If you `destroy` an unconfirmed subscription, this provider will remove the subscription from its state but the subscription will still exist in AWS. However, if you delete an SNS topic, SNS [deletes all the subscriptions](https://docs.aws.amazon.com/sns/latest/dg/sns-delete-subscription-topic.html) associated with the topic. Also, you can import a subscription after confirmation and then have the capability to delete it.
///
/// ## Example Usage
///
/// You can directly supply a topic and ARN by hand in the `topic_arn` property along with the queue ARN:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let userUpdatesSqsTarget = topic_subscription::create(
///         "userUpdatesSqsTarget",
///         TopicSubscriptionArgs::builder()
///             .endpoint("arn:aws:sqs:us-west-2:432981146916:queue-too")
///             .protocol("sqs")
///             .topic("arn:aws:sns:us-west-2:432981146916:user-updates-topic")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Alternatively you can use the ARN properties of a managed SNS topic and SQS queue:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let userUpdates = topic::create(
///         "userUpdates",
///         TopicArgs::builder().name("user-updates-topic").build_struct(),
///     );
///     let userUpdatesQueue = queue::create(
///         "userUpdatesQueue",
///         QueueArgs::builder().name("user-updates-queue").build_struct(),
///     );
///     let userUpdatesSqsTarget = topic_subscription::create(
///         "userUpdatesSqsTarget",
///         TopicSubscriptionArgs::builder()
///             .endpoint("${userUpdatesQueue.arn}")
///             .protocol("sqs")
///             .topic("${userUpdates.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// You can subscribe SNS topics to SQS queues in different Amazon accounts and regions:
///
/// ```yaml
/// configuration:
///   sns:
///     type: dynamic
///     default:
///       account-id: '111111111111'
///       displayName: example
///       name: example-sns-topic
///       region: us-west-1
///       role-name: service/service
///   sqs:
///     type: dynamic
///     default:
///       account-id: '222222222222'
///       name: example-sqs-queue
///       region: us-east-1
///       role-name: service/service
/// resources:
///   sns-topic:
///     type: aws:sns:Topic
///     properties:
///       name: ${sns.name}
///       displayName: ${sns.display_name}
///       policy: ${["sns-topic-policy"].json}
///   sqs-queue:
///     type: aws:sqs:Queue
///     properties:
///       name: ${sqs.name}
///       policy: ${["sqs-queue-policy"].json}
///   sns-topicTopicSubscription:
///     type: aws:sns:TopicSubscription
///     name: sns-topic
///     properties:
///       topic: ${["sns-topic"].arn}
///       protocol: sqs
///       endpoint: ${["sqs-queue"].arn}
/// variables:
///   sns-topic-policy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         policyId: __default_policy_ID
///         statements:
///           - actions:
///               - SNS:Subscribe
///               - SNS:SetTopicAttributes
///               - SNS:RemovePermission
///               - SNS:Publish
///               - SNS:ListSubscriptionsByTopic
///               - SNS:GetTopicAttributes
///               - SNS:DeleteTopic
///               - SNS:AddPermission
///             conditions:
///               - test: StringEquals
///                 variable: AWS:SourceOwner
///                 values:
///                   - ${sns"account-id"[%!s(MISSING)]}
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '*'
///             resources:
///               - arn:aws:sns:${sns.region}:${sns"account-id"[%!s(MISSING)]}:${sns.name}
///             sid: __default_statement_ID
///           - actions:
///               - SNS:Subscribe
///               - SNS:Receive
///             conditions:
///               - test: StringLike
///                 variable: SNS:Endpoint
///                 values:
///                   - arn:aws:sqs:${sqs.region}:${sqs"account-id"[%!s(MISSING)]}:${sqs.name}
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '*'
///             resources:
///               - arn:aws:sns:${sns.region}:${sns"account-id"[%!s(MISSING)]}:${sns.name}
///             sid: __console_sub_0
///   sqs-queue-policy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         policyId: arn:aws:sqs:${sqs.region}:${sqs"account-id"[%!s(MISSING)]}:${sqs.name}/SQSDefaultPolicy
///         statements:
///           - sid: example-sns-topic
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '*'
///             actions:
///               - SQS:SendMessage
///             resources:
///               - arn:aws:sqs:${sqs.region}:${sqs"account-id"[%!s(MISSING)]}:${sqs.name}
///             conditions:
///               - test: ArnEquals
///                 variable: aws:SourceArn
///                 values:
///                   - arn:aws:sns:${sns.region}:${sns"account-id"[%!s(MISSING)]}:${sns.name}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SNS Topic Subscriptions using the subscription `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:sns/topicSubscription:TopicSubscription user_updates_sqs_target arn:aws:sns:us-west-2:123456789012:my-topic:8a21d249-4329-4871-acc6-7be709c6ea7f
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod topic_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicSubscriptionArgs {
        /// Integer indicating number of minutes to wait in retrying mode for fetching subscription arn before marking it as failure. Only applicable for http and https protocols. Default is `1`.
        #[builder(into, default)]
        pub confirmation_timeout_in_minutes: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// JSON String with the delivery policy (retries, backoff, etc.) that will be used in the subscription - this only applies to HTTP/S subscriptions. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/DeliveryPolicies.html) for more details.
        #[builder(into, default)]
        pub delivery_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Endpoint to send data to. The contents vary with the protocol. See details below.
        #[builder(into)]
        pub endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the endpoint is capable of [auto confirming subscription](http://docs.aws.amazon.com/sns/latest/dg/SendMessageToHttp.html#SendMessageToHttp.prepare) (e.g., PagerDuty). Default is `false`.
        #[builder(into, default)]
        pub endpoint_auto_confirms: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// JSON String with the filter policy that will be used in the subscription to filter messages seen by the target resource. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/message-filtering.html) for more details.
        #[builder(into, default)]
        pub filter_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the `filter_policy` applies to `MessageAttributes` (default) or `MessageBody`.
        #[builder(into, default)]
        pub filter_policy_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Protocol to use. Valid values are: `sqs`, `sms`, `lambda`, `firehose`, and `application`. Protocols `email`, `email-json`, `http` and `https` are also valid but partially supported. See details below.
        #[builder(into)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to enable raw message delivery (the original message is directly passed, not wrapped in JSON with the original message in the message property). Default is `false`.
        #[builder(into, default)]
        pub raw_message_delivery: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// JSON String with the redrive policy that will be used in the subscription. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/sns-dead-letter-queues.html#how-messages-moved-into-dead-letter-queue) for more details.
        #[builder(into, default)]
        pub redrive_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// JSON String with the archived message replay policy that will be used in the subscription. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-subscriber.html) for more details.
        #[builder(into, default)]
        pub replay_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the IAM role to publish to Kinesis Data Firehose delivery stream. Refer to [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html).
        #[builder(into, default)]
        pub subscription_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the SNS topic to subscribe to.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub topic: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TopicSubscriptionResult {
        /// ARN of the subscription.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Integer indicating number of minutes to wait in retrying mode for fetching subscription arn before marking it as failure. Only applicable for http and https protocols. Default is `1`.
        pub confirmation_timeout_in_minutes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Whether the subscription confirmation request was authenticated.
        pub confirmation_was_authenticated: pulumi_gestalt_rust::Output<bool>,
        /// JSON String with the delivery policy (retries, backoff, etc.) that will be used in the subscription - this only applies to HTTP/S subscriptions. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/DeliveryPolicies.html) for more details.
        pub delivery_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Endpoint to send data to. The contents vary with the protocol. See details below.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Whether the endpoint is capable of [auto confirming subscription](http://docs.aws.amazon.com/sns/latest/dg/SendMessageToHttp.html#SendMessageToHttp.prepare) (e.g., PagerDuty). Default is `false`.
        pub endpoint_auto_confirms: pulumi_gestalt_rust::Output<Option<bool>>,
        /// JSON String with the filter policy that will be used in the subscription to filter messages seen by the target resource. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/message-filtering.html) for more details.
        pub filter_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the `filter_policy` applies to `MessageAttributes` (default) or `MessageBody`.
        pub filter_policy_scope: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID of the subscription's owner.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Whether the subscription has not been confirmed.
        pub pending_confirmation: pulumi_gestalt_rust::Output<bool>,
        /// Protocol to use. Valid values are: `sqs`, `sms`, `lambda`, `firehose`, and `application`. Protocols `email`, `email-json`, `http` and `https` are also valid but partially supported. See details below.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable raw message delivery (the original message is directly passed, not wrapped in JSON with the original message in the message property). Default is `false`.
        pub raw_message_delivery: pulumi_gestalt_rust::Output<Option<bool>>,
        /// JSON String with the redrive policy that will be used in the subscription. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/sns-dead-letter-queues.html#how-messages-moved-into-dead-letter-queue) for more details.
        pub redrive_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// JSON String with the archived message replay policy that will be used in the subscription. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-subscriber.html) for more details.
        pub replay_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the IAM role to publish to Kinesis Data Firehose delivery stream. Refer to [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html).
        pub subscription_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the SNS topic to subscribe to.
        ///
        /// The following arguments are optional:
        pub topic: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TopicSubscriptionArgs,
    ) -> TopicSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let confirmation_timeout_in_minutes_binding = args
            .confirmation_timeout_in_minutes
            .get_output(context);
        let delivery_policy_binding = args.delivery_policy.get_output(context);
        let endpoint_binding = args.endpoint.get_output(context);
        let endpoint_auto_confirms_binding = args
            .endpoint_auto_confirms
            .get_output(context);
        let filter_policy_binding = args.filter_policy.get_output(context);
        let filter_policy_scope_binding = args.filter_policy_scope.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let raw_message_delivery_binding = args.raw_message_delivery.get_output(context);
        let redrive_policy_binding = args.redrive_policy.get_output(context);
        let replay_policy_binding = args.replay_policy.get_output(context);
        let subscription_role_arn_binding = args
            .subscription_role_arn
            .get_output(context);
        let topic_binding = args.topic.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sns/topicSubscription:TopicSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "confirmationTimeoutInMinutes".into(),
                    value: &confirmation_timeout_in_minutes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryPolicy".into(),
                    value: &delivery_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpoint".into(),
                    value: &endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointAutoConfirms".into(),
                    value: &endpoint_auto_confirms_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filterPolicy".into(),
                    value: &filter_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filterPolicyScope".into(),
                    value: &filter_policy_scope_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rawMessageDelivery".into(),
                    value: &raw_message_delivery_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redrivePolicy".into(),
                    value: &redrive_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replayPolicy".into(),
                    value: &replay_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriptionRoleArn".into(),
                    value: &subscription_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topic".into(),
                    value: &topic_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TopicSubscriptionResult {
            arn: o.get_field("arn"),
            confirmation_timeout_in_minutes: o.get_field("confirmationTimeoutInMinutes"),
            confirmation_was_authenticated: o.get_field("confirmationWasAuthenticated"),
            delivery_policy: o.get_field("deliveryPolicy"),
            endpoint: o.get_field("endpoint"),
            endpoint_auto_confirms: o.get_field("endpointAutoConfirms"),
            filter_policy: o.get_field("filterPolicy"),
            filter_policy_scope: o.get_field("filterPolicyScope"),
            owner_id: o.get_field("ownerId"),
            pending_confirmation: o.get_field("pendingConfirmation"),
            protocol: o.get_field("protocol"),
            raw_message_delivery: o.get_field("rawMessageDelivery"),
            redrive_policy: o.get_field("redrivePolicy"),
            replay_policy: o.get_field("replayPolicy"),
            subscription_role_arn: o.get_field("subscriptionRoleArn"),
            topic: o.get_field("topic"),
        }
    }
}
