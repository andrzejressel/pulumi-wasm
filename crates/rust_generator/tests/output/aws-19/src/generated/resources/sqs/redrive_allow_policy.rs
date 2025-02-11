/// Provides a SQS Queue Redrive Allow Policy resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   src:
///     type: aws:sqs:Queue
///     properties:
///       name: srcqueue
///       redrivePolicy:
///         fn::toJSON:
///           deadLetterTargetArn: ${example.arn}
///           maxReceiveCount: 4
///   example:
///     type: aws:sqs:Queue
///     properties:
///       name: examplequeue
///   exampleRedriveAllowPolicy:
///     type: aws:sqs:RedriveAllowPolicy
///     name: example
///     properties:
///       queueUrl: ${example.id}
///       redriveAllowPolicy:
///         fn::toJSON:
///           redrivePermission: byQueue
///           sourceQueueArns:
///             - ${src.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SQS Queue Redrive Allow Policies using the queue URL. For example:
///
/// ```sh
/// $ pulumi import aws:sqs/redriveAllowPolicy:RedriveAllowPolicy test https://queue.amazonaws.com/123456789012/myqueue
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod redrive_allow_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RedriveAllowPolicyArgs {
        /// The URL of the SQS Queue to which to attach the policy
        #[builder(into)]
        pub queue_url: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The JSON redrive allow policy for the SQS queue. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
        #[builder(into)]
        pub redrive_allow_policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RedriveAllowPolicyResult {
        /// The URL of the SQS Queue to which to attach the policy
        pub queue_url: pulumi_gestalt_rust::Output<String>,
        /// The JSON redrive allow policy for the SQS queue. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
        pub redrive_allow_policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RedriveAllowPolicyArgs,
    ) -> RedriveAllowPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let queue_url_binding = args.queue_url.get_output(context);
        let redrive_allow_policy_binding = args.redrive_allow_policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sqs/redriveAllowPolicy:RedriveAllowPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueUrl".into(),
                    value: &queue_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redriveAllowPolicy".into(),
                    value: &redrive_allow_policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RedriveAllowPolicyResult {
            queue_url: o.get_field("queueUrl"),
            redrive_allow_policy: o.get_field("redriveAllowPolicy"),
        }
    }
}
