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
/// ```yaml
/// resources:
///   topicTopic:
///     type: aws:sns:Topic
///     name: topic
///     properties:
///       name: s3-event-notification-topic
///       policy: ${topic.json}
///   bucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: your-bucket-name
///   bucketNotification:
///     type: aws:s3:BucketNotification
///     name: bucket_notification
///     properties:
///       bucket: ${bucket.id}
///       topics:
///         - topicArn: ${topicTopic.arn}
///           events:
///             - s3:ObjectCreated:*
///           filterSuffix: .log
/// variables:
///   topic:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - s3.amazonaws.com
///             actions:
///               - SNS:Publish
///             resources:
///               - arn:aws:sns:*:*:s3-event-notification-topic
///             conditions:
///               - test: ArnLike
///                 variable: aws:SourceArn
///                 values:
///                   - ${bucket.arn}
/// ```
///
/// ### Add notification configuration to SQS Queue
///
/// ```yaml
/// resources:
///   queueQueue:
///     type: aws:sqs:Queue
///     name: queue
///     properties:
///       name: s3-event-notification-queue
///       policy: ${queue.json}
///   bucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: your-bucket-name
///   bucketNotification:
///     type: aws:s3:BucketNotification
///     name: bucket_notification
///     properties:
///       bucket: ${bucket.id}
///       queues:
///         - queueArn: ${queueQueue.arn}
///           events:
///             - s3:ObjectCreated:*
///           filterSuffix: .log
/// variables:
///   queue:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: '*'
///                 identifiers:
///                   - '*'
///             actions:
///               - sqs:SendMessage
///             resources:
///               - arn:aws:sqs:*:*:s3-event-notification-queue
///             conditions:
///               - test: ArnEquals
///                 variable: aws:SourceArn
///                 values:
///                   - ${bucket.arn}
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
///       dependsOn:
///         - ${allowBucket}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
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
///       dependsOn:
///         - ${allowBucket1}
///         - ${allowBucket2}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
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
/// ```yaml
/// resources:
///   queueQueue:
///     type: aws:sqs:Queue
///     name: queue
///     properties:
///       name: s3-event-notification-queue
///       policy: ${queue.json}
///   bucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: your-bucket-name
///   bucketNotification:
///     type: aws:s3:BucketNotification
///     name: bucket_notification
///     properties:
///       bucket: ${bucket.id}
///       queues:
///         - id: image-upload-event
///           queueArn: ${queueQueue.arn}
///           events:
///             - s3:ObjectCreated:*
///           filterPrefix: images/
///         - id: video-upload-event
///           queueArn: ${queueQueue.arn}
///           events:
///             - s3:ObjectCreated:*
///           filterPrefix: videos/
/// variables:
///   queue:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: '*'
///                 identifiers:
///                   - '*'
///             actions:
///               - sqs:SendMessage
///             resources:
///               - arn:aws:sqs:*:*:s3-event-notification-queue
///             conditions:
///               - test: ArnEquals
///                 variable: aws:SourceArn
///                 values:
///                   - ${bucket.arn}
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_notification {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketNotificationArgs {
        /// Name of the bucket for notification configuration.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to enable Amazon EventBridge notifications. Defaults to `false`.
        #[builder(into, default)]
        pub eventbridge: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Used to configure notifications to a Lambda Function. See below.
        #[builder(into, default)]
        pub lambda_functions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketNotificationLambdaFunction>>,
        >,
        /// Notification configuration to SQS Queue. See below.
        #[builder(into, default)]
        pub queues: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketNotificationQueue>>,
        >,
        /// Notification configuration to SNS Topic. See below.
        #[builder(into, default)]
        pub topics: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketNotificationTopic>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketNotificationResult {
        /// Name of the bucket for notification configuration.
        ///
        /// The following arguments are optional:
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable Amazon EventBridge notifications. Defaults to `false`.
        pub eventbridge: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Used to configure notifications to a Lambda Function. See below.
        pub lambda_functions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::s3::BucketNotificationLambdaFunction>>,
        >,
        /// Notification configuration to SQS Queue. See below.
        pub queues: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::s3::BucketNotificationQueue>>,
        >,
        /// Notification configuration to SNS Topic. See below.
        pub topics: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::s3::BucketNotificationTopic>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketNotificationArgs,
    ) -> BucketNotificationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let eventbridge_binding = args.eventbridge.get_output(context);
        let lambda_functions_binding = args.lambda_functions.get_output(context);
        let queues_binding = args.queues.get_output(context);
        let topics_binding = args.topics.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/bucketNotification:BucketNotification".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventbridge".into(),
                    value: &eventbridge_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lambdaFunctions".into(),
                    value: &lambda_functions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queues".into(),
                    value: &queues_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topics".into(),
                    value: &topics_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketNotificationResult {
            bucket: o.get_field("bucket"),
            eventbridge: o.get_field("eventbridge"),
            lambda_functions: o.get_field("lambdaFunctions"),
            queues: o.get_field("queues"),
            topics: o.get_field("topics"),
        }
    }
}
