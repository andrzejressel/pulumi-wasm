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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
///       Function: aws:iam:getPolicyDocument
///       Arguments:
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
///       Function: aws:iam:getPolicyDocument
///       Arguments:
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
pub mod topic_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicSubscriptionArgs {
        /// Integer indicating number of minutes to wait in retrying mode for fetching subscription arn before marking it as failure. Only applicable for http and https protocols. Default is `1`.
        #[builder(into, default)]
        pub confirmation_timeout_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// JSON String with the delivery policy (retries, backoff, etc.) that will be used in the subscription - this only applies to HTTP/S subscriptions. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/DeliveryPolicies.html) for more details.
        #[builder(into, default)]
        pub delivery_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Endpoint to send data to. The contents vary with the protocol. See details below.
        #[builder(into)]
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Whether the endpoint is capable of [auto confirming subscription](http://docs.aws.amazon.com/sns/latest/dg/SendMessageToHttp.html#SendMessageToHttp.prepare) (e.g., PagerDuty). Default is `false`.
        #[builder(into, default)]
        pub endpoint_auto_confirms: pulumi_wasm_rust::Output<Option<bool>>,
        /// JSON String with the filter policy that will be used in the subscription to filter messages seen by the target resource. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/message-filtering.html) for more details.
        #[builder(into, default)]
        pub filter_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the `filter_policy` applies to `MessageAttributes` (default) or `MessageBody`.
        #[builder(into, default)]
        pub filter_policy_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Protocol to use. Valid values are: `sqs`, `sms`, `lambda`, `firehose`, and `application`. Protocols `email`, `email-json`, `http` and `https` are also valid but partially supported. See details below.
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// Whether to enable raw message delivery (the original message is directly passed, not wrapped in JSON with the original message in the message property). Default is `false`.
        #[builder(into, default)]
        pub raw_message_delivery: pulumi_wasm_rust::Output<Option<bool>>,
        /// JSON String with the redrive policy that will be used in the subscription. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/sns-dead-letter-queues.html#how-messages-moved-into-dead-letter-queue) for more details.
        #[builder(into, default)]
        pub redrive_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// JSON String with the archived message replay policy that will be used in the subscription. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-subscriber.html) for more details.
        #[builder(into, default)]
        pub replay_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the IAM role to publish to Kinesis Data Firehose delivery stream. Refer to [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html).
        #[builder(into, default)]
        pub subscription_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the SNS topic to subscribe to.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub topic: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TopicSubscriptionResult {
        /// ARN of the subscription.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Integer indicating number of minutes to wait in retrying mode for fetching subscription arn before marking it as failure. Only applicable for http and https protocols. Default is `1`.
        pub confirmation_timeout_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Whether the subscription confirmation request was authenticated.
        pub confirmation_was_authenticated: pulumi_wasm_rust::Output<bool>,
        /// JSON String with the delivery policy (retries, backoff, etc.) that will be used in the subscription - this only applies to HTTP/S subscriptions. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/DeliveryPolicies.html) for more details.
        pub delivery_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Endpoint to send data to. The contents vary with the protocol. See details below.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Whether the endpoint is capable of [auto confirming subscription](http://docs.aws.amazon.com/sns/latest/dg/SendMessageToHttp.html#SendMessageToHttp.prepare) (e.g., PagerDuty). Default is `false`.
        pub endpoint_auto_confirms: pulumi_wasm_rust::Output<Option<bool>>,
        /// JSON String with the filter policy that will be used in the subscription to filter messages seen by the target resource. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/message-filtering.html) for more details.
        pub filter_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the `filter_policy` applies to `MessageAttributes` (default) or `MessageBody`.
        pub filter_policy_scope: pulumi_wasm_rust::Output<String>,
        /// AWS account ID of the subscription's owner.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Whether the subscription has not been confirmed.
        pub pending_confirmation: pulumi_wasm_rust::Output<bool>,
        /// Protocol to use. Valid values are: `sqs`, `sms`, `lambda`, `firehose`, and `application`. Protocols `email`, `email-json`, `http` and `https` are also valid but partially supported. See details below.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// Whether to enable raw message delivery (the original message is directly passed, not wrapped in JSON with the original message in the message property). Default is `false`.
        pub raw_message_delivery: pulumi_wasm_rust::Output<Option<bool>>,
        /// JSON String with the redrive policy that will be used in the subscription. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/sns-dead-letter-queues.html#how-messages-moved-into-dead-letter-queue) for more details.
        pub redrive_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// JSON String with the archived message replay policy that will be used in the subscription. Refer to the [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-subscriber.html) for more details.
        pub replay_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the IAM role to publish to Kinesis Data Firehose delivery stream. Refer to [SNS docs](https://docs.aws.amazon.com/sns/latest/dg/sns-firehose-as-subscriber.html).
        pub subscription_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the SNS topic to subscribe to.
        ///
        /// The following arguments are optional:
        pub topic: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TopicSubscriptionArgs) -> TopicSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let confirmation_timeout_in_minutes_binding = args
            .confirmation_timeout_in_minutes
            .get_inner();
        let delivery_policy_binding = args.delivery_policy.get_inner();
        let endpoint_binding = args.endpoint.get_inner();
        let endpoint_auto_confirms_binding = args.endpoint_auto_confirms.get_inner();
        let filter_policy_binding = args.filter_policy.get_inner();
        let filter_policy_scope_binding = args.filter_policy_scope.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let raw_message_delivery_binding = args.raw_message_delivery.get_inner();
        let redrive_policy_binding = args.redrive_policy.get_inner();
        let replay_policy_binding = args.replay_policy.get_inner();
        let subscription_role_arn_binding = args.subscription_role_arn.get_inner();
        let topic_binding = args.topic.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sns/topicSubscription:TopicSubscription".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "confirmationTimeoutInMinutes".into(),
                    value: &confirmation_timeout_in_minutes_binding,
                },
                register_interface::ObjectField {
                    name: "deliveryPolicy".into(),
                    value: &delivery_policy_binding,
                },
                register_interface::ObjectField {
                    name: "endpoint".into(),
                    value: &endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "endpointAutoConfirms".into(),
                    value: &endpoint_auto_confirms_binding,
                },
                register_interface::ObjectField {
                    name: "filterPolicy".into(),
                    value: &filter_policy_binding,
                },
                register_interface::ObjectField {
                    name: "filterPolicyScope".into(),
                    value: &filter_policy_scope_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "rawMessageDelivery".into(),
                    value: &raw_message_delivery_binding,
                },
                register_interface::ObjectField {
                    name: "redrivePolicy".into(),
                    value: &redrive_policy_binding,
                },
                register_interface::ObjectField {
                    name: "replayPolicy".into(),
                    value: &replay_policy_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionRoleArn".into(),
                    value: &subscription_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "topic".into(),
                    value: &topic_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "confirmationTimeoutInMinutes".into(),
                },
                register_interface::ResultField {
                    name: "confirmationWasAuthenticated".into(),
                },
                register_interface::ResultField {
                    name: "deliveryPolicy".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "endpointAutoConfirms".into(),
                },
                register_interface::ResultField {
                    name: "filterPolicy".into(),
                },
                register_interface::ResultField {
                    name: "filterPolicyScope".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "pendingConfirmation".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "rawMessageDelivery".into(),
                },
                register_interface::ResultField {
                    name: "redrivePolicy".into(),
                },
                register_interface::ResultField {
                    name: "replayPolicy".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "topic".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TopicSubscriptionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            confirmation_timeout_in_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("confirmationTimeoutInMinutes").unwrap(),
            ),
            confirmation_was_authenticated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("confirmationWasAuthenticated").unwrap(),
            ),
            delivery_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryPolicy").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            endpoint_auto_confirms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointAutoConfirms").unwrap(),
            ),
            filter_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filterPolicy").unwrap(),
            ),
            filter_policy_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filterPolicyScope").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            pending_confirmation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pendingConfirmation").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            raw_message_delivery: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rawMessageDelivery").unwrap(),
            ),
            redrive_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redrivePolicy").unwrap(),
            ),
            replay_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replayPolicy").unwrap(),
            ),
            subscription_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionRoleArn").unwrap(),
            ),
            topic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topic").unwrap(),
            ),
        }
    }
}