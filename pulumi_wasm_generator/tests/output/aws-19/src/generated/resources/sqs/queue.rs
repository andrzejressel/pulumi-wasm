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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod queue {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueueArgs {
        /// Enables content-based deduplication for FIFO queues. For more information, see the [related documentation](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-exactly-once-processing)
        #[builder(into, default)]
        pub content_based_deduplication: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether message deduplication occurs at the message group or queue level. Valid values are `messageGroup` and `queue` (default).
        #[builder(into, default)]
        pub deduplication_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// The time in seconds that the delivery of all messages in the queue will be delayed. An integer from 0 to 900 (15 minutes). The default for this attribute is 0 seconds.
        #[builder(into, default)]
        pub delay_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Boolean designating a FIFO queue. If not set, it defaults to `false` making it standard.
        #[builder(into, default)]
        pub fifo_queue: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are `perQueue` (default) and `perMessageGroupId`.
        #[builder(into, default)]
        pub fifo_throughput_limit: pulumi_wasm_rust::Output<Option<String>>,
        /// The length of time, in seconds, for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling AWS KMS again. An integer representing seconds, between 60 seconds (1 minute) and 86,400 seconds (24 hours). The default is 300 (5 minutes).
        #[builder(into, default)]
        pub kms_data_key_reuse_period_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of an AWS-managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see [Key Terms](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms).
        #[builder(into, default)]
        pub kms_master_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The limit of how many bytes a message can contain before Amazon SQS rejects it. An integer from 1024 bytes (1 KiB) up to 262144 bytes (256 KiB). The default for this attribute is 262144 (256 KiB).
        #[builder(into, default)]
        pub max_message_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The number of seconds Amazon SQS retains a message. Integer representing seconds, from 60 (1 minute) to 1209600 (14 days). The default for this attribute is 345600 (4 days).
        #[builder(into, default)]
        pub message_retention_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the queue. Queue names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 80 characters long. For a FIFO (first-in-first-out) queue, the name must end with the `.fifo` suffix. If omitted, this provider will assign a random, unique name. Conflicts with `name_prefix`
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The JSON policy for the SQS queue.
        #[builder(into, default)]
        pub policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The time for which a ReceiveMessage call will wait for a message to arrive (long polling) before returning. An integer from 0 to 20 (seconds). The default for this attribute is 0, meaning that the call will return immediately.
        #[builder(into, default)]
        pub receive_wait_time_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The JSON policy to set up the Dead Letter Queue redrive permission, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html).
        #[builder(into, default)]
        pub redrive_allow_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The JSON policy to set up the Dead Letter Queue, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html). **Note:** when specifying `maxReceiveCount`, you must specify it as an integer (`5`), and not a string (`"5"`).
        #[builder(into, default)]
        pub redrive_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean to enable server-side encryption (SSE) of message content with SQS-owned encryption keys. See [Encryption at rest](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html). The provider will only perform drift detection of its value when present in a configuration.
        #[builder(into, default)]
        pub sqs_managed_sse_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of tags to assign to the queue. If configured with a provider `default_tags` configuration block) present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The visibility timeout for the queue. An integer from 0 to 43200 (12 hours). The default for this attribute is 30. For more information about visibility timeout, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/AboutVT.html).
        #[builder(into, default)]
        pub visibility_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct QueueResult {
        /// The ARN of the SQS queue
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Enables content-based deduplication for FIFO queues. For more information, see the [related documentation](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-exactly-once-processing)
        pub content_based_deduplication: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether message deduplication occurs at the message group or queue level. Valid values are `messageGroup` and `queue` (default).
        pub deduplication_scope: pulumi_wasm_rust::Output<String>,
        /// The time in seconds that the delivery of all messages in the queue will be delayed. An integer from 0 to 900 (15 minutes). The default for this attribute is 0 seconds.
        pub delay_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Boolean designating a FIFO queue. If not set, it defaults to `false` making it standard.
        pub fifo_queue: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are `perQueue` (default) and `perMessageGroupId`.
        pub fifo_throughput_limit: pulumi_wasm_rust::Output<String>,
        /// The length of time, in seconds, for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling AWS KMS again. An integer representing seconds, between 60 seconds (1 minute) and 86,400 seconds (24 hours). The default is 300 (5 minutes).
        pub kms_data_key_reuse_period_seconds: pulumi_wasm_rust::Output<i32>,
        /// The ID of an AWS-managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see [Key Terms](http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms).
        pub kms_master_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The limit of how many bytes a message can contain before Amazon SQS rejects it. An integer from 1024 bytes (1 KiB) up to 262144 bytes (256 KiB). The default for this attribute is 262144 (256 KiB).
        pub max_message_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The number of seconds Amazon SQS retains a message. Integer representing seconds, from 60 (1 minute) to 1209600 (14 days). The default for this attribute is 345600 (4 days).
        pub message_retention_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the queue. Queue names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 80 characters long. For a FIFO (first-in-first-out) queue, the name must end with the `.fifo` suffix. If omitted, this provider will assign a random, unique name. Conflicts with `name_prefix`
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// The JSON policy for the SQS queue.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The time for which a ReceiveMessage call will wait for a message to arrive (long polling) before returning. An integer from 0 to 20 (seconds). The default for this attribute is 0, meaning that the call will return immediately.
        pub receive_wait_time_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The JSON policy to set up the Dead Letter Queue redrive permission, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html).
        pub redrive_allow_policy: pulumi_wasm_rust::Output<String>,
        /// The JSON policy to set up the Dead Letter Queue, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/SQSDeadLetterQueue.html). **Note:** when specifying `maxReceiveCount`, you must specify it as an integer (`5`), and not a string (`"5"`).
        pub redrive_policy: pulumi_wasm_rust::Output<String>,
        /// Boolean to enable server-side encryption (SSE) of message content with SQS-owned encryption keys. See [Encryption at rest](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html). The provider will only perform drift detection of its value when present in a configuration.
        pub sqs_managed_sse_enabled: pulumi_wasm_rust::Output<bool>,
        /// A map of tags to assign to the queue. If configured with a provider `default_tags` configuration block) present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Same as `id`: The URL for the created Amazon SQS queue.
        pub url: pulumi_wasm_rust::Output<String>,
        /// The visibility timeout for the queue. An integer from 0 to 43200 (12 hours). The default for this attribute is 30. For more information about visibility timeout, see [AWS docs](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/AboutVT.html).
        pub visibility_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: QueueArgs) -> QueueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_based_deduplication_binding = args
            .content_based_deduplication
            .get_inner();
        let deduplication_scope_binding = args.deduplication_scope.get_inner();
        let delay_seconds_binding = args.delay_seconds.get_inner();
        let fifo_queue_binding = args.fifo_queue.get_inner();
        let fifo_throughput_limit_binding = args.fifo_throughput_limit.get_inner();
        let kms_data_key_reuse_period_seconds_binding = args
            .kms_data_key_reuse_period_seconds
            .get_inner();
        let kms_master_key_id_binding = args.kms_master_key_id.get_inner();
        let max_message_size_binding = args.max_message_size.get_inner();
        let message_retention_seconds_binding = args
            .message_retention_seconds
            .get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let policy_binding = args.policy.get_inner();
        let receive_wait_time_seconds_binding = args
            .receive_wait_time_seconds
            .get_inner();
        let redrive_allow_policy_binding = args.redrive_allow_policy.get_inner();
        let redrive_policy_binding = args.redrive_policy.get_inner();
        let sqs_managed_sse_enabled_binding = args.sqs_managed_sse_enabled.get_inner();
        let tags_binding = args.tags.get_inner();
        let visibility_timeout_seconds_binding = args
            .visibility_timeout_seconds
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sqs/queue:Queue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contentBasedDeduplication".into(),
                    value: &content_based_deduplication_binding,
                },
                register_interface::ObjectField {
                    name: "deduplicationScope".into(),
                    value: &deduplication_scope_binding,
                },
                register_interface::ObjectField {
                    name: "delaySeconds".into(),
                    value: &delay_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "fifoQueue".into(),
                    value: &fifo_queue_binding,
                },
                register_interface::ObjectField {
                    name: "fifoThroughputLimit".into(),
                    value: &fifo_throughput_limit_binding,
                },
                register_interface::ObjectField {
                    name: "kmsDataKeyReusePeriodSeconds".into(),
                    value: &kms_data_key_reuse_period_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "kmsMasterKeyId".into(),
                    value: &kms_master_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "maxMessageSize".into(),
                    value: &max_message_size_binding,
                },
                register_interface::ObjectField {
                    name: "messageRetentionSeconds".into(),
                    value: &message_retention_seconds_binding,
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
                    name: "receiveWaitTimeSeconds".into(),
                    value: &receive_wait_time_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "redriveAllowPolicy".into(),
                    value: &redrive_allow_policy_binding,
                },
                register_interface::ObjectField {
                    name: "redrivePolicy".into(),
                    value: &redrive_policy_binding,
                },
                register_interface::ObjectField {
                    name: "sqsManagedSseEnabled".into(),
                    value: &sqs_managed_sse_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "visibilityTimeoutSeconds".into(),
                    value: &visibility_timeout_seconds_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "contentBasedDeduplication".into(),
                },
                register_interface::ResultField {
                    name: "deduplicationScope".into(),
                },
                register_interface::ResultField {
                    name: "delaySeconds".into(),
                },
                register_interface::ResultField {
                    name: "fifoQueue".into(),
                },
                register_interface::ResultField {
                    name: "fifoThroughputLimit".into(),
                },
                register_interface::ResultField {
                    name: "kmsDataKeyReusePeriodSeconds".into(),
                },
                register_interface::ResultField {
                    name: "kmsMasterKeyId".into(),
                },
                register_interface::ResultField {
                    name: "maxMessageSize".into(),
                },
                register_interface::ResultField {
                    name: "messageRetentionSeconds".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "receiveWaitTimeSeconds".into(),
                },
                register_interface::ResultField {
                    name: "redriveAllowPolicy".into(),
                },
                register_interface::ResultField {
                    name: "redrivePolicy".into(),
                },
                register_interface::ResultField {
                    name: "sqsManagedSseEnabled".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
                register_interface::ResultField {
                    name: "visibilityTimeoutSeconds".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        QueueResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            content_based_deduplication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentBasedDeduplication").unwrap(),
            ),
            deduplication_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deduplicationScope").unwrap(),
            ),
            delay_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delaySeconds").unwrap(),
            ),
            fifo_queue: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fifoQueue").unwrap(),
            ),
            fifo_throughput_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fifoThroughputLimit").unwrap(),
            ),
            kms_data_key_reuse_period_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsDataKeyReusePeriodSeconds").unwrap(),
            ),
            kms_master_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsMasterKeyId").unwrap(),
            ),
            max_message_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxMessageSize").unwrap(),
            ),
            message_retention_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("messageRetentionSeconds").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            receive_wait_time_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("receiveWaitTimeSeconds").unwrap(),
            ),
            redrive_allow_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redriveAllowPolicy").unwrap(),
            ),
            redrive_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redrivePolicy").unwrap(),
            ),
            sqs_managed_sse_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqsManagedSseEnabled").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("url").unwrap(),
            ),
            visibility_timeout_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("visibilityTimeoutSeconds").unwrap(),
            ),
        }
    }
}
