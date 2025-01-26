/// Provides an SNS topic resource
///
/// ## Example Usage
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
/// }
/// ```
///
/// ## Example with Delivery Policy
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let userUpdates = topic::create(
///         "userUpdates",
///         TopicArgs::builder()
///             .delivery_policy(
///                 "{\n  \"http\": {\n    \"defaultHealthyRetryPolicy\": {\n      \"minDelayTarget\": 20,\n      \"maxDelayTarget\": 20,\n      \"numRetries\": 3,\n      \"numMaxDelayRetries\": 0,\n      \"numNoDelayRetries\": 0,\n      \"numMinDelayRetries\": 0,\n      \"backoffFunction\": \"linear\"\n    },\n    \"disableSubscriptionOverrides\": false,\n    \"defaultThrottlePolicy\": {\n      \"maxReceivesPerSecond\": 1\n    }\n  }\n}",
///             )
///             .name("user-updates-topic")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example with Server-side encryption (SSE)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let userUpdates = topic::create(
///         "userUpdates",
///         TopicArgs::builder()
///             .kms_master_key_id("alias/aws/sns")
///             .name("user-updates-topic")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example with First-In-First-Out (FIFO)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let userUpdates = topic::create(
///         "userUpdates",
///         TopicArgs::builder()
///             .content_based_deduplication(true)
///             .fifo_topic(true)
///             .name("user-updates-topic.fifo")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Message Delivery Status Arguments
///
/// The `<endpoint>_success_feedback_role_arn` and `<endpoint>_failure_feedback_role_arn` arguments are used to give Amazon SNS write access to use CloudWatch Logs on your behalf. The `<endpoint>_success_feedback_sample_rate` argument is for specifying the sample rate percentage (0-100) of successfully delivered messages. After you configure the  `<endpoint>_failure_feedback_role_arn` argument, then all failed message deliveries generate CloudWatch Logs.
///
/// ## Import
///
/// Using `pulumi import`, import SNS Topics using the topic `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:sns/topic:Topic user_updates arn:aws:sns:us-west-2:123456789012:my-topic
/// ```
pub mod topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicArgs {
        /// IAM role for failure feedback
        #[builder(into, default)]
        pub application_failure_feedback_role_arn: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        #[builder(into, default)]
        pub application_success_feedback_role_arn: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Percentage of success to sample
        #[builder(into, default)]
        pub application_success_feedback_sample_rate: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The message archive policy for FIFO topics. More details in the [AWS documentation](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-topic-owner.html).
        #[builder(into, default)]
        pub archive_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Enables content-based deduplication for FIFO topics. For more information, see the [related documentation](https://docs.aws.amazon.com/sns/latest/dg/fifo-message-dedup.html)
        #[builder(into, default)]
        pub content_based_deduplication: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The SNS delivery policy. More details in the [AWS documentation](https://docs.aws.amazon.com/sns/latest/dg/DeliveryPolicies.html).
        #[builder(into, default)]
        pub delivery_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The display name for the topic
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Boolean indicating whether or not to create a FIFO (first-in-first-out) topic. FIFO topics can't deliver messages to customer managed endpoints, such as email addresses, mobile apps, SMS, or HTTP(S) endpoints. These endpoint types aren't guaranteed to preserve strict message ordering. Default is `false`.
        #[builder(into, default)]
        pub fifo_topic: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// IAM role for failure feedback
        #[builder(into, default)]
        pub firehose_failure_feedback_role_arn: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        #[builder(into, default)]
        pub firehose_success_feedback_role_arn: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Percentage of success to sample
        #[builder(into, default)]
        pub firehose_success_feedback_sample_rate: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// IAM role for failure feedback
        #[builder(into, default)]
        pub http_failure_feedback_role_arn: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        #[builder(into, default)]
        pub http_success_feedback_role_arn: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Percentage of success to sample
        #[builder(into, default)]
        pub http_success_feedback_sample_rate: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The ID of an AWS-managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see [Key Terms](https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms)
        #[builder(into, default)]
        pub kms_master_key_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// IAM role for failure feedback
        #[builder(into, default)]
        pub lambda_failure_feedback_role_arn: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        #[builder(into, default)]
        pub lambda_success_feedback_role_arn: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Percentage of success to sample
        #[builder(into, default)]
        pub lambda_success_feedback_sample_rate: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The name of the topic. Topic names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters long. For a FIFO (first-in-first-out) topic, the name must end with the `.fifo` suffix. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The fully-formed AWS policy as JSON.
        #[builder(into, default)]
        pub policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If `SignatureVersion` should be [1 (SHA1) or 2 (SHA256)](https://docs.aws.amazon.com/sns/latest/dg/sns-verify-signature-of-message.html). The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS.
        #[builder(into, default)]
        pub signature_version: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// IAM role for failure feedback
        #[builder(into, default)]
        pub sqs_failure_feedback_role_arn: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        #[builder(into, default)]
        pub sqs_success_feedback_role_arn: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Percentage of success to sample
        #[builder(into, default)]
        pub sqs_success_feedback_sample_rate: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Tracing mode of an Amazon SNS topic. Valid values: `"PassThrough"`, `"Active"`.
        #[builder(into, default)]
        pub tracing_config: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TopicResult {
        /// IAM role for failure feedback
        pub application_failure_feedback_role_arn: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The IAM role permitted to receive success feedback for this topic
        pub application_success_feedback_role_arn: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Percentage of success to sample
        pub application_success_feedback_sample_rate: pulumi_wasm_rust::Output<
            Option<i32>,
        >,
        /// The message archive policy for FIFO topics. More details in the [AWS documentation](https://docs.aws.amazon.com/sns/latest/dg/message-archiving-and-replay-topic-owner.html).
        pub archive_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the SNS topic, as a more obvious property (clone of id)
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The oldest timestamp at which a FIFO topic subscriber can start a replay.
        pub beginning_archive_time: pulumi_wasm_rust::Output<String>,
        /// Enables content-based deduplication for FIFO topics. For more information, see the [related documentation](https://docs.aws.amazon.com/sns/latest/dg/fifo-message-dedup.html)
        pub content_based_deduplication: pulumi_wasm_rust::Output<Option<bool>>,
        /// The SNS delivery policy. More details in the [AWS documentation](https://docs.aws.amazon.com/sns/latest/dg/DeliveryPolicies.html).
        pub delivery_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name for the topic
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean indicating whether or not to create a FIFO (first-in-first-out) topic. FIFO topics can't deliver messages to customer managed endpoints, such as email addresses, mobile apps, SMS, or HTTP(S) endpoints. These endpoint types aren't guaranteed to preserve strict message ordering. Default is `false`.
        pub fifo_topic: pulumi_wasm_rust::Output<Option<bool>>,
        /// IAM role for failure feedback
        pub firehose_failure_feedback_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The IAM role permitted to receive success feedback for this topic
        pub firehose_success_feedback_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Percentage of success to sample
        pub firehose_success_feedback_sample_rate: pulumi_wasm_rust::Output<Option<i32>>,
        /// IAM role for failure feedback
        pub http_failure_feedback_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The IAM role permitted to receive success feedback for this topic
        pub http_success_feedback_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Percentage of success to sample
        pub http_success_feedback_sample_rate: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of an AWS-managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see [Key Terms](https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html#sse-key-terms)
        pub kms_master_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// IAM role for failure feedback
        pub lambda_failure_feedback_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The IAM role permitted to receive success feedback for this topic
        pub lambda_success_feedback_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Percentage of success to sample
        pub lambda_success_feedback_sample_rate: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the topic. Topic names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters long. For a FIFO (first-in-first-out) topic, the name must end with the `.fifo` suffix. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// The AWS Account ID of the SNS topic owner
        pub owner: pulumi_wasm_rust::Output<String>,
        /// The fully-formed AWS policy as JSON.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// If `SignatureVersion` should be [1 (SHA1) or 2 (SHA256)](https://docs.aws.amazon.com/sns/latest/dg/sns-verify-signature-of-message.html). The signature version corresponds to the hashing algorithm used while creating the signature of the notifications, subscription confirmations, or unsubscribe confirmation messages sent by Amazon SNS.
        pub signature_version: pulumi_wasm_rust::Output<i32>,
        /// IAM role for failure feedback
        pub sqs_failure_feedback_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The IAM role permitted to receive success feedback for this topic
        pub sqs_success_feedback_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Percentage of success to sample
        pub sqs_success_feedback_sample_rate: pulumi_wasm_rust::Output<Option<i32>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Tracing mode of an Amazon SNS topic. Valid values: `"PassThrough"`, `"Active"`.
        pub tracing_config: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TopicArgs,
    ) -> TopicResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_failure_feedback_role_arn_binding = args
            .application_failure_feedback_role_arn
            .get_output(context)
            .get_inner();
        let application_success_feedback_role_arn_binding = args
            .application_success_feedback_role_arn
            .get_output(context)
            .get_inner();
        let application_success_feedback_sample_rate_binding = args
            .application_success_feedback_sample_rate
            .get_output(context)
            .get_inner();
        let archive_policy_binding = args.archive_policy.get_output(context).get_inner();
        let content_based_deduplication_binding = args
            .content_based_deduplication
            .get_output(context)
            .get_inner();
        let delivery_policy_binding = args
            .delivery_policy
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let fifo_topic_binding = args.fifo_topic.get_output(context).get_inner();
        let firehose_failure_feedback_role_arn_binding = args
            .firehose_failure_feedback_role_arn
            .get_output(context)
            .get_inner();
        let firehose_success_feedback_role_arn_binding = args
            .firehose_success_feedback_role_arn
            .get_output(context)
            .get_inner();
        let firehose_success_feedback_sample_rate_binding = args
            .firehose_success_feedback_sample_rate
            .get_output(context)
            .get_inner();
        let http_failure_feedback_role_arn_binding = args
            .http_failure_feedback_role_arn
            .get_output(context)
            .get_inner();
        let http_success_feedback_role_arn_binding = args
            .http_success_feedback_role_arn
            .get_output(context)
            .get_inner();
        let http_success_feedback_sample_rate_binding = args
            .http_success_feedback_sample_rate
            .get_output(context)
            .get_inner();
        let kms_master_key_id_binding = args
            .kms_master_key_id
            .get_output(context)
            .get_inner();
        let lambda_failure_feedback_role_arn_binding = args
            .lambda_failure_feedback_role_arn
            .get_output(context)
            .get_inner();
        let lambda_success_feedback_role_arn_binding = args
            .lambda_success_feedback_role_arn
            .get_output(context)
            .get_inner();
        let lambda_success_feedback_sample_rate_binding = args
            .lambda_success_feedback_sample_rate
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let signature_version_binding = args
            .signature_version
            .get_output(context)
            .get_inner();
        let sqs_failure_feedback_role_arn_binding = args
            .sqs_failure_feedback_role_arn
            .get_output(context)
            .get_inner();
        let sqs_success_feedback_role_arn_binding = args
            .sqs_success_feedback_role_arn
            .get_output(context)
            .get_inner();
        let sqs_success_feedback_sample_rate_binding = args
            .sqs_success_feedback_sample_rate
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tracing_config_binding = args.tracing_config.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sns/topic:Topic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationFailureFeedbackRoleArn".into(),
                    value: &application_failure_feedback_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "applicationSuccessFeedbackRoleArn".into(),
                    value: &application_success_feedback_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "applicationSuccessFeedbackSampleRate".into(),
                    value: &application_success_feedback_sample_rate_binding,
                },
                register_interface::ObjectField {
                    name: "archivePolicy".into(),
                    value: &archive_policy_binding,
                },
                register_interface::ObjectField {
                    name: "contentBasedDeduplication".into(),
                    value: &content_based_deduplication_binding,
                },
                register_interface::ObjectField {
                    name: "deliveryPolicy".into(),
                    value: &delivery_policy_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "fifoTopic".into(),
                    value: &fifo_topic_binding,
                },
                register_interface::ObjectField {
                    name: "firehoseFailureFeedbackRoleArn".into(),
                    value: &firehose_failure_feedback_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "firehoseSuccessFeedbackRoleArn".into(),
                    value: &firehose_success_feedback_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "firehoseSuccessFeedbackSampleRate".into(),
                    value: &firehose_success_feedback_sample_rate_binding,
                },
                register_interface::ObjectField {
                    name: "httpFailureFeedbackRoleArn".into(),
                    value: &http_failure_feedback_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "httpSuccessFeedbackRoleArn".into(),
                    value: &http_success_feedback_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "httpSuccessFeedbackSampleRate".into(),
                    value: &http_success_feedback_sample_rate_binding,
                },
                register_interface::ObjectField {
                    name: "kmsMasterKeyId".into(),
                    value: &kms_master_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "lambdaFailureFeedbackRoleArn".into(),
                    value: &lambda_failure_feedback_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "lambdaSuccessFeedbackRoleArn".into(),
                    value: &lambda_success_feedback_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "lambdaSuccessFeedbackSampleRate".into(),
                    value: &lambda_success_feedback_sample_rate_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "signatureVersion".into(),
                    value: &signature_version_binding,
                },
                register_interface::ObjectField {
                    name: "sqsFailureFeedbackRoleArn".into(),
                    value: &sqs_failure_feedback_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "sqsSuccessFeedbackRoleArn".into(),
                    value: &sqs_success_feedback_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "sqsSuccessFeedbackSampleRate".into(),
                    value: &sqs_success_feedback_sample_rate_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tracingConfig".into(),
                    value: &tracing_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationFailureFeedbackRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "applicationSuccessFeedbackRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "applicationSuccessFeedbackSampleRate".into(),
                },
                register_interface::ResultField {
                    name: "archivePolicy".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "beginningArchiveTime".into(),
                },
                register_interface::ResultField {
                    name: "contentBasedDeduplication".into(),
                },
                register_interface::ResultField {
                    name: "deliveryPolicy".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "fifoTopic".into(),
                },
                register_interface::ResultField {
                    name: "firehoseFailureFeedbackRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "firehoseSuccessFeedbackRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "firehoseSuccessFeedbackSampleRate".into(),
                },
                register_interface::ResultField {
                    name: "httpFailureFeedbackRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "httpSuccessFeedbackRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "httpSuccessFeedbackSampleRate".into(),
                },
                register_interface::ResultField {
                    name: "kmsMasterKeyId".into(),
                },
                register_interface::ResultField {
                    name: "lambdaFailureFeedbackRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "lambdaSuccessFeedbackRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "lambdaSuccessFeedbackSampleRate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "signatureVersion".into(),
                },
                register_interface::ResultField {
                    name: "sqsFailureFeedbackRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "sqsSuccessFeedbackRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "sqsSuccessFeedbackSampleRate".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "tracingConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TopicResult {
            application_failure_feedback_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationFailureFeedbackRoleArn").unwrap(),
            ),
            application_success_feedback_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationSuccessFeedbackRoleArn").unwrap(),
            ),
            application_success_feedback_sample_rate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationSuccessFeedbackSampleRate").unwrap(),
            ),
            archive_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("archivePolicy").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            beginning_archive_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("beginningArchiveTime").unwrap(),
            ),
            content_based_deduplication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentBasedDeduplication").unwrap(),
            ),
            delivery_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryPolicy").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            fifo_topic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fifoTopic").unwrap(),
            ),
            firehose_failure_feedback_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firehoseFailureFeedbackRoleArn").unwrap(),
            ),
            firehose_success_feedback_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firehoseSuccessFeedbackRoleArn").unwrap(),
            ),
            firehose_success_feedback_sample_rate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firehoseSuccessFeedbackSampleRate").unwrap(),
            ),
            http_failure_feedback_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpFailureFeedbackRoleArn").unwrap(),
            ),
            http_success_feedback_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpSuccessFeedbackRoleArn").unwrap(),
            ),
            http_success_feedback_sample_rate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpSuccessFeedbackSampleRate").unwrap(),
            ),
            kms_master_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsMasterKeyId").unwrap(),
            ),
            lambda_failure_feedback_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lambdaFailureFeedbackRoleArn").unwrap(),
            ),
            lambda_success_feedback_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lambdaSuccessFeedbackRoleArn").unwrap(),
            ),
            lambda_success_feedback_sample_rate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lambdaSuccessFeedbackSampleRate").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            signature_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signatureVersion").unwrap(),
            ),
            sqs_failure_feedback_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqsFailureFeedbackRoleArn").unwrap(),
            ),
            sqs_success_feedback_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqsSuccessFeedbackRoleArn").unwrap(),
            ),
            sqs_success_feedback_sample_rate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqsSuccessFeedbackSampleRate").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            tracing_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tracingConfig").unwrap(),
            ),
        }
    }
}
