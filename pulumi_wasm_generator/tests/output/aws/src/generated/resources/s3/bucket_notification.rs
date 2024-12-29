/// Manages a S3 Bucket Notification Configuration. For additional information, see the [Configuring S3 Event Notifications section in the Amazon S3 Developer Guide](https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html).
///
/// > **NOTE:** S3 Buckets only support a single notification configuration resource. Declaring multiple `aws.s3.BucketNotification` resources to the same S3 Bucket will cause a perpetual difference in configuration. This resource will overwrite any existing event notifications configured for the S3 bucket it's associated with. See the example "Trigger multiple Lambda functions" for an option of how to configure multiple triggers within this resource.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### Add notification configuration to SNS Topic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let topic = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder().actions(vec!["SNS:Publish",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("ArnLike").values(vec!["${bucket.arn}",])
///                     .variable("aws:SourceArn").build_struct(),]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["s3.amazonaws.com",]). type ("Service")
///                     .build_struct(),])
///                     .resources(vec!["arn:aws:sns:*:*:s3-event-notification-topic",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let bucket = bucket_v_2::create(
///         "bucket",
///         BucketV2Args::builder().bucket("your-bucket-name").build_struct(),
///     );
///     let bucketNotification = bucket_notification::create(
///         "bucketNotification",
///         BucketNotificationArgs::builder()
///             .bucket("${bucket.id}")
///             .topics(
///                 vec![
///                     BucketNotificationTopic::builder()
///                     .events(vec!["s3:ObjectCreated:*",]).filterSuffix(".log")
///                     .topicArn("${topicTopic.arn}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let topicTopic = topic::create(
///         "topicTopic",
///         TopicArgs::builder()
///             .name("s3-event-notification-topic")
///             .policy("${topic.json}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Add notification configuration to SQS Queue
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let queue = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sqs:SendMessage",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("ArnEquals").values(vec!["${bucket.arn}",])
///                     .variable("aws:SourceArn").build_struct(),]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["*",]). type ("*").build_struct(),])
///                     .resources(vec!["arn:aws:sqs:*:*:s3-event-notification-queue",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let bucket = bucket_v_2::create(
///         "bucket",
///         BucketV2Args::builder().bucket("your-bucket-name").build_struct(),
///     );
///     let bucketNotification = bucket_notification::create(
///         "bucketNotification",
///         BucketNotificationArgs::builder()
///             .bucket("${bucket.id}")
///             .queues(
///                 vec![
///                     BucketNotificationQueue::builder()
///                     .events(vec!["s3:ObjectCreated:*",]).filterSuffix(".log")
///                     .queueArn("${queueQueue.arn}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let queueQueue = queue::create(
///         "queueQueue",
///         QueueArgs::builder()
///             .name("s3-event-notification-queue")
///             .policy("${queue.json}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Add notification configuration to Lambda Function
///
/// ```yaml
/// resources:
///   iamForLambda:
///     type: aws:iam:Role
///     name: iam_for_lambda
///     properties:
///       name: iam_for_lambda
///       assumeRolePolicy: ${assumeRole.json}
///   allowBucket:
///     type: aws:lambda:Permission
///     name: allow_bucket
///     properties:
///       statementId: AllowExecutionFromS3Bucket
///       action: lambda:InvokeFunction
///       function: ${func.arn}
///       principal: s3.amazonaws.com
///       sourceArn: ${bucket.arn}
///   func:
///     type: aws:lambda:Function
///     properties:
///       code:
///         fn::FileArchive: your-function.zip
///       name: example_lambda_name
///       role: ${iamForLambda.arn}
///       handler: exports.example
///       runtime: nodejs20.x
///   bucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: your-bucket-name
///   bucketNotification:
///     type: aws:s3:BucketNotification
///     name: bucket_notification
///     properties:
///       bucket: ${bucket.id}
///       lambdaFunctions:
///         - lambdaFunctionArn: ${func.arn}
///           events:
///             - s3:ObjectCreated:*
///           filterPrefix: AWSLogs/
///           filterSuffix: .log
///     options:
///       dependson:
///         - ${allowBucket}
/// variables:
///   assumeRole:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - lambda.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ### Trigger multiple Lambda functions
///
/// ```yaml
/// resources:
///   iamForLambda:
///     type: aws:iam:Role
///     name: iam_for_lambda
///     properties:
///       name: iam_for_lambda
///       assumeRolePolicy: ${assumeRole.json}
///   allowBucket1:
///     type: aws:lambda:Permission
///     name: allow_bucket1
///     properties:
///       statementId: AllowExecutionFromS3Bucket1
///       action: lambda:InvokeFunction
///       function: ${func1.arn}
///       principal: s3.amazonaws.com
///       sourceArn: ${bucket.arn}
///   func1:
///     type: aws:lambda:Function
///     properties:
///       code:
///         fn::FileArchive: your-function1.zip
///       name: example_lambda_name1
///       role: ${iamForLambda.arn}
///       handler: exports.example
///       runtime: nodejs20.x
///   allowBucket2:
///     type: aws:lambda:Permission
///     name: allow_bucket2
///     properties:
///       statementId: AllowExecutionFromS3Bucket2
///       action: lambda:InvokeFunction
///       function: ${func2.arn}
///       principal: s3.amazonaws.com
///       sourceArn: ${bucket.arn}
///   func2:
///     type: aws:lambda:Function
///     properties:
///       code:
///         fn::FileArchive: your-function2.zip
///       name: example_lambda_name2
///       role: ${iamForLambda.arn}
///       handler: exports.example
///   bucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: your-bucket-name
///   bucketNotification:
///     type: aws:s3:BucketNotification
///     name: bucket_notification
///     properties:
///       bucket: ${bucket.id}
///       lambdaFunctions:
///         - lambdaFunctionArn: ${func1.arn}
///           events:
///             - s3:ObjectCreated:*
///           filterPrefix: AWSLogs/
///           filterSuffix: .log
///         - lambdaFunctionArn: ${func2.arn}
///           events:
///             - s3:ObjectCreated:*
///           filterPrefix: OtherLogs/
///           filterSuffix: .log
///     options:
///       dependson:
///         - ${allowBucket1}
///         - ${allowBucket2}
/// variables:
///   assumeRole:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - lambda.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ### Add multiple notification configurations to SQS Queue
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let queue = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sqs:SendMessage",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("ArnEquals").values(vec!["${bucket.arn}",])
///                     .variable("aws:SourceArn").build_struct(),]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["*",]). type ("*").build_struct(),])
///                     .resources(vec!["arn:aws:sqs:*:*:s3-event-notification-queue",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let bucket = bucket_v_2::create(
///         "bucket",
///         BucketV2Args::builder().bucket("your-bucket-name").build_struct(),
///     );
///     let bucketNotification = bucket_notification::create(
///         "bucketNotification",
///         BucketNotificationArgs::builder()
///             .bucket("${bucket.id}")
///             .queues(
///                 vec![
///                     BucketNotificationQueue::builder()
///                     .events(vec!["s3:ObjectCreated:*",]).filterPrefix("images/")
///                     .id("image-upload-event").queueArn("${queueQueue.arn}")
///                     .build_struct(), BucketNotificationQueue::builder()
///                     .events(vec!["s3:ObjectCreated:*",]).filterPrefix("videos/")
///                     .id("video-upload-event").queueArn("${queueQueue.arn}")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let queueQueue = queue::create(
///         "queueQueue",
///         QueueArgs::builder()
///             .name("s3-event-notification-queue")
///             .policy("${queue.json}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// For JSON syntax, use an array instead of defining the `queue` key twice.
///
/// ```json
/// {
/// 	"bucket": "${aws_s3_bucket.bucket.id}",
/// 	"queue": [
/// 		{
/// 			"id": "image-upload-event",
/// 			"queue_arn": "${aws_sqs_queue.queue.arn}",
/// 			"events": ["s3:ObjectCreated:*"],
/// 			"filter_prefix": "images/"
/// 		},
/// 		{
/// 			"id": "video-upload-event",
/// 			"queue_arn": "${aws_sqs_queue.queue.arn}",
/// 			"events": ["s3:ObjectCreated:*"],
/// 			"filter_prefix": "videos/"
/// 		}
/// 	]
/// }
/// ```
///
/// ### Emit events to EventBridge
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bucket = bucket_v_2::create(
///         "bucket",
///         BucketV2Args::builder().bucket("your-bucket-name").build_struct(),
///     );
///     let bucketNotification = bucket_notification::create(
///         "bucketNotification",
///         BucketNotificationArgs::builder()
///             .bucket("${bucket.id}")
///             .eventbridge(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 bucket notification using the `bucket`. For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketNotification:BucketNotification bucket_notification bucket-name
/// ```
pub mod bucket_notification {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketNotificationArgs {
        /// Name of the bucket for notification configuration.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Whether to enable Amazon EventBridge notifications. Defaults to `false`.
        #[builder(into, default)]
        pub eventbridge: pulumi_wasm_rust::Output<Option<bool>>,
        /// Used to configure notifications to a Lambda Function. See below.
        #[builder(into, default)]
        pub lambda_functions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::s3::BucketNotificationLambdaFunction>>,
        >,
        /// Notification configuration to SQS Queue. See below.
        #[builder(into, default)]
        pub queues: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::s3::BucketNotificationQueue>>,
        >,
        /// Notification configuration to SNS Topic. See below.
        #[builder(into, default)]
        pub topics: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::s3::BucketNotificationTopic>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketNotificationResult {
        /// Name of the bucket for notification configuration.
        ///
        /// The following arguments are optional:
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Whether to enable Amazon EventBridge notifications. Defaults to `false`.
        pub eventbridge: pulumi_wasm_rust::Output<Option<bool>>,
        /// Used to configure notifications to a Lambda Function. See below.
        pub lambda_functions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::s3::BucketNotificationLambdaFunction>>,
        >,
        /// Notification configuration to SQS Queue. See below.
        pub queues: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::s3::BucketNotificationQueue>>,
        >,
        /// Notification configuration to SNS Topic. See below.
        pub topics: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::s3::BucketNotificationTopic>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BucketNotificationArgs) -> BucketNotificationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let eventbridge_binding = args.eventbridge.get_inner();
        let lambda_functions_binding = args.lambda_functions.get_inner();
        let queues_binding = args.queues.get_inner();
        let topics_binding = args.topics.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketNotification:BucketNotification".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "eventbridge".into(),
                    value: &eventbridge_binding,
                },
                register_interface::ObjectField {
                    name: "lambdaFunctions".into(),
                    value: &lambda_functions_binding,
                },
                register_interface::ObjectField {
                    name: "queues".into(),
                    value: &queues_binding,
                },
                register_interface::ObjectField {
                    name: "topics".into(),
                    value: &topics_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "eventbridge".into(),
                },
                register_interface::ResultField {
                    name: "lambdaFunctions".into(),
                },
                register_interface::ResultField {
                    name: "queues".into(),
                },
                register_interface::ResultField {
                    name: "topics".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketNotificationResult {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            eventbridge: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventbridge").unwrap(),
            ),
            lambda_functions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lambdaFunctions").unwrap(),
            ),
            queues: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queues").unwrap(),
            ),
            topics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topics").unwrap(),
            ),
        }
    }
}
