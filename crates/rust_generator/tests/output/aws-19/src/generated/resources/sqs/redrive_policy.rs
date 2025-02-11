/// Allows you to set a redrive policy of an SQS Queue
/// while referencing ARN of the dead letter queue inside the redrive policy.
///
/// This is useful when you want to set a dedicated
/// dead letter queue for a standard or FIFO queue, but need
/// the dead letter queue to exist before setting the redrive policy.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   q:
///     type: aws:sqs:Queue
///     properties:
///       name: examplequeue
///   ddl:
///     type: aws:sqs:Queue
///     properties:
///       name: examplequeue-ddl
///       redriveAllowPolicy:
///         fn::toJSON:
///           redrivePermission: byQueue
///           sourceQueueArns:
///             - ${q.arn}
///   qRedrivePolicy:
///     type: aws:sqs:RedrivePolicy
///     name: q
///     properties:
///       queueUrl: ${q.id}
///       redrivePolicy:
///         fn::toJSON:
///           deadLetterTargetArn: ${ddl.arn}
///           maxReceiveCount: 4
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SQS Queue Redrive Policies using the queue URL. For example:
///
/// ```sh
/// $ pulumi import aws:sqs/redrivePolicy:RedrivePolicy test https://queue.amazonaws.com/123456789012/myqueue
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod redrive_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RedrivePolicyArgs {
        /// The URL of the SQS Queue to which to attach the policy
        #[builder(into)]
        pub queue_url: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The JSON redrive policy for the SQS queue. Accepts two key/val pairs: `deadLetterTargetArn` and `maxReceiveCount`. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
        #[builder(into)]
        pub redrive_policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RedrivePolicyResult {
        /// The URL of the SQS Queue to which to attach the policy
        pub queue_url: pulumi_gestalt_rust::Output<String>,
        /// The JSON redrive policy for the SQS queue. Accepts two key/val pairs: `deadLetterTargetArn` and `maxReceiveCount`. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
        pub redrive_policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RedrivePolicyArgs,
    ) -> RedrivePolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let queue_url_binding = args.queue_url.get_output(context);
        let redrive_policy_binding = args.redrive_policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sqs/redrivePolicy:RedrivePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueUrl".into(),
                    value: &queue_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redrivePolicy".into(),
                    value: &redrive_policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RedrivePolicyResult {
            queue_url: o.get_field("queueUrl"),
            redrive_policy: o.get_field("redrivePolicy"),
        }
    }
}
