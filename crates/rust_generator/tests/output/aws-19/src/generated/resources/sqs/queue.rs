/// ## Example Usage
///
/// ```yaml
/// resources:
///   queue:
///     type: aws:sqs:Queue
///     properties:
///       name: example-queue
///       delaySeconds: 90
///       maxMessageSize: 2048
///       messageRetentionSeconds: 86400
///       receiveWaitTimeSeconds: 10
///       redrivePolicy:
///         fn::toJSON:
///           deadLetterTargetArn: ${queueDeadletter.arn}
///           maxReceiveCount: 4
///       tags:
///         Environment: production
/// ```
///
/// ## FIFO queue
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let queue = queue::create(
///         "queue",
///         QueueArgs::builder()
///             .content_based_deduplication(true)
///             .fifo_queue(true)
///             .name("example-queue.fifo")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## High-throughput FIFO queue
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let queue = queue::create(
///         "queue",
///         QueueArgs::builder()
///             .deduplication_scope("messageGroup")
///             .fifo_queue(true)
///             .fifo_throughput_limit("perMessageGroupId")
///             .name("pulumi-example-queue.fifo")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Dead-letter queue
///
/// ```yaml
/// resources:
///   queue:
///     type: aws:sqs:Queue
///     properties:
///       name: pulumi-example-queue
///       redrivePolicy:
///         fn::toJSON:
///           deadLetterTargetArn: ${queueDeadletter.arn}
///           maxReceiveCount: 4
///   exampleQueueDeadletter:
///     type: aws:sqs:Queue
///     name: example_queue_deadletter
///     properties:
///       name: pulumi-example-deadletter-queue
///   exampleQueueRedriveAllowPolicy:
///     type: aws:sqs:RedriveAllowPolicy
///     name: example_queue_redrive_allow_policy
///     properties:
///       queueUrl: ${exampleQueueDeadletter.id}
///       redriveAllowPolicy:
///         fn::toJSON:
///           redrivePermission: byQueue
///           sourceQueueArns:
///             - ${exampleQueue.arn}
/// ```
///
/// ## Server-side encryption (SSE)
///
/// Using [SSE-SQS](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sqs-sse-queue.html):
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let queue = queue::create(
///         "queue",
///         QueueArgs::builder()
///             .name("pulumi-example-queue")
///             .sqs_managed_sse_enabled(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// Using [SSE-KMS](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-sse-existing-queue.html):
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let queue = queue::create(
///         "queue",
///         QueueArgs::builder()
///             .kms_data_key_reuse_period_seconds(300)
///             .kms_master_key_id("alias/aws/sqs")
///             .name("example-queue")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SQS Queues using the queue `url`. For example:
///
/// ```sh
/// $ pulumi import aws:sqs/queue:Queue public_queue https://queue.amazonaws.com/80398EXAMPLE/MyQueue
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod queue {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueueArgs {
        /// Enables content-based deduplication for FIFO queues. For more information, see the [related documentation](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-exactly-once-processing)
        #[builder(into, default)]
        pub content_based_deduplication: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies whether message deduplication occurs at the message group or queue level. Valid values are `messageGroup` and `queue` (default).
        #[builder(into, default)]
        pub deduplication_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The time in seconds that the delivery of all messages in the queue will be delayed. An integer from 0 to 900 (15 minutes). The default for this attribute is 0 seconds.
        #[builder(into, default)]
        pub delay_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Boolean designating a FIFO queue. If not set, it defaults to `false` making it standard.
        #[builder(into, default)]
        pub fifo_queue: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are `perQueue` (default) and `perMessageGroupId`.
        #[builder(into, default)]
        pub fifo_throughput_limit: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The length of time, in seconds, for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling AWS KMS again. An integer representing seconds, between 60 seconds (1 minute) and 86,400 seconds (24 hours). The default is 300 (5 minutes).
        #[builder(into, default)]
        pub kms_data_key_reuse_period_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The ID of an AWS-managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see [Key Terms](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms).
        #[builder(into, default)]
        pub kms_master_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The limit of how many bytes a message can contain before Amazon SQS rejects it. An integer from 1024 bytes (1 KiB) up to 262144 bytes (256 KiB). The default for this attribute is 262144 (256 KiB).
        #[builder(into, default)]
        pub max_message_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The number of seconds Amazon SQS retains a message. Integer representing seconds, from 60 (1 minute) to 1209600 (14 days). The default for this attribute is 345600 (4 days).
        #[builder(into, default)]
        pub message_retention_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the queue. Queue names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 80 characters long. For a FIFO (first-in-first-out) queue, the name must end with the `.fifo` suffix. If omitted, this provider will assign a random, unique name. Conflicts with `name_prefix`
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The JSON policy for the SQS queue.
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The time for which a ReceiveMessage call will wait for a message to arrive (long polling) before returning. An integer from 0 to 20 (seconds). The default for this attribute is 0, meaning that the call will return immediately.
        #[builder(into, default)]
        pub receive_wait_time_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The JSON policy to set up the Dead Letter Queue redrive permission, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html).
        #[builder(into, default)]
        pub redrive_allow_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The JSON policy to set up the Dead Letter Queue, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html). **Note:** when specifying `maxReceiveCount`, you must specify it as an integer (`5`), and not a string (`"5"`).
        #[builder(into, default)]
        pub redrive_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean to enable server-side encryption (SSE) of message content with SQS-owned encryption keys. See [Encryption at rest](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html). The provider will only perform drift detection of its value when present in a configuration.
        #[builder(into, default)]
        pub sqs_managed_sse_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the queue. If configured with a provider `default_tags` configuration block) present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The visibility timeout for the queue. An integer from 0 to 43200 (12 hours). The default for this attribute is 30. For more information about visibility timeout, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/AboutVT.html).
        #[builder(into, default)]
        pub visibility_timeout_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct QueueResult {
        /// The ARN of the SQS queue
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Enables content-based deduplication for FIFO queues. For more information, see the [related documentation](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-exactly-once-processing)
        pub content_based_deduplication: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether message deduplication occurs at the message group or queue level. Valid values are `messageGroup` and `queue` (default).
        pub deduplication_scope: pulumi_gestalt_rust::Output<String>,
        /// The time in seconds that the delivery of all messages in the queue will be delayed. An integer from 0 to 900 (15 minutes). The default for this attribute is 0 seconds.
        pub delay_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Boolean designating a FIFO queue. If not set, it defaults to `false` making it standard.
        pub fifo_queue: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are `perQueue` (default) and `perMessageGroupId`.
        pub fifo_throughput_limit: pulumi_gestalt_rust::Output<String>,
        /// The length of time, in seconds, for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling AWS KMS again. An integer representing seconds, between 60 seconds (1 minute) and 86,400 seconds (24 hours). The default is 300 (5 minutes).
        pub kms_data_key_reuse_period_seconds: pulumi_gestalt_rust::Output<i32>,
        /// The ID of an AWS-managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see [Key Terms](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms).
        pub kms_master_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The limit of how many bytes a message can contain before Amazon SQS rejects it. An integer from 1024 bytes (1 KiB) up to 262144 bytes (256 KiB). The default for this attribute is 262144 (256 KiB).
        pub max_message_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The number of seconds Amazon SQS retains a message. Integer representing seconds, from 60 (1 minute) to 1209600 (14 days). The default for this attribute is 345600 (4 days).
        pub message_retention_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the queue. Queue names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 80 characters long. For a FIFO (first-in-first-out) queue, the name must end with the `.fifo` suffix. If omitted, this provider will assign a random, unique name. Conflicts with `name_prefix`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The JSON policy for the SQS queue.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The time for which a ReceiveMessage call will wait for a message to arrive (long polling) before returning. An integer from 0 to 20 (seconds). The default for this attribute is 0, meaning that the call will return immediately.
        pub receive_wait_time_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The JSON policy to set up the Dead Letter Queue redrive permission, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html).
        pub redrive_allow_policy: pulumi_gestalt_rust::Output<String>,
        /// The JSON policy to set up the Dead Letter Queue, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html). **Note:** when specifying `maxReceiveCount`, you must specify it as an integer (`5`), and not a string (`"5"`).
        pub redrive_policy: pulumi_gestalt_rust::Output<String>,
        /// Boolean to enable server-side encryption (SSE) of message content with SQS-owned encryption keys. See [Encryption at rest](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html). The provider will only perform drift detection of its value when present in a configuration.
        pub sqs_managed_sse_enabled: pulumi_gestalt_rust::Output<bool>,
        /// A map of tags to assign to the queue. If configured with a provider `default_tags` configuration block) present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Same as `id`: The URL for the created Amazon SQS queue.
        pub url: pulumi_gestalt_rust::Output<String>,
        /// The visibility timeout for the queue. An integer from 0 to 43200 (12 hours). The default for this attribute is 30. For more information about visibility timeout, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/AboutVT.html).
        pub visibility_timeout_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QueueArgs,
    ) -> QueueResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let content_based_deduplication_binding = args
            .content_based_deduplication
            .get_output(context);
        let deduplication_scope_binding = args.deduplication_scope.get_output(context);
        let delay_seconds_binding = args.delay_seconds.get_output(context);
        let fifo_queue_binding = args.fifo_queue.get_output(context);
        let fifo_throughput_limit_binding = args
            .fifo_throughput_limit
            .get_output(context);
        let kms_data_key_reuse_period_seconds_binding = args
            .kms_data_key_reuse_period_seconds
            .get_output(context);
        let kms_master_key_id_binding = args.kms_master_key_id.get_output(context);
        let max_message_size_binding = args.max_message_size.get_output(context);
        let message_retention_seconds_binding = args
            .message_retention_seconds
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let receive_wait_time_seconds_binding = args
            .receive_wait_time_seconds
            .get_output(context);
        let redrive_allow_policy_binding = args.redrive_allow_policy.get_output(context);
        let redrive_policy_binding = args.redrive_policy.get_output(context);
        let sqs_managed_sse_enabled_binding = args
            .sqs_managed_sse_enabled
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let visibility_timeout_seconds_binding = args
            .visibility_timeout_seconds
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sqs/queue:Queue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentBasedDeduplication".into(),
                    value: content_based_deduplication_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deduplicationScope".into(),
                    value: deduplication_scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delaySeconds".into(),
                    value: delay_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fifoQueue".into(),
                    value: fifo_queue_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fifoThroughputLimit".into(),
                    value: fifo_throughput_limit_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsDataKeyReusePeriodSeconds".into(),
                    value: kms_data_key_reuse_period_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsMasterKeyId".into(),
                    value: kms_master_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxMessageSize".into(),
                    value: max_message_size_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "messageRetentionSeconds".into(),
                    value: message_retention_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "receiveWaitTimeSeconds".into(),
                    value: receive_wait_time_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redriveAllowPolicy".into(),
                    value: redrive_allow_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redrivePolicy".into(),
                    value: redrive_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqsManagedSseEnabled".into(),
                    value: sqs_managed_sse_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "visibilityTimeoutSeconds".into(),
                    value: visibility_timeout_seconds_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        QueueResult {
            arn: o.get_field("arn"),
            content_based_deduplication: o.get_field("contentBasedDeduplication"),
            deduplication_scope: o.get_field("deduplicationScope"),
            delay_seconds: o.get_field("delaySeconds"),
            fifo_queue: o.get_field("fifoQueue"),
            fifo_throughput_limit: o.get_field("fifoThroughputLimit"),
            kms_data_key_reuse_period_seconds: o
                .get_field("kmsDataKeyReusePeriodSeconds"),
            kms_master_key_id: o.get_field("kmsMasterKeyId"),
            max_message_size: o.get_field("maxMessageSize"),
            message_retention_seconds: o.get_field("messageRetentionSeconds"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            policy: o.get_field("policy"),
            receive_wait_time_seconds: o.get_field("receiveWaitTimeSeconds"),
            redrive_allow_policy: o.get_field("redriveAllowPolicy"),
            redrive_policy: o.get_field("redrivePolicy"),
            sqs_managed_sse_enabled: o.get_field("sqsManagedSseEnabled"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            url: o.get_field("url"),
            visibility_timeout_seconds: o.get_field("visibilityTimeoutSeconds"),
        }
    }
}
