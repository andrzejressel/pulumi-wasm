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
pub mod redrive_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RedrivePolicyArgs {
        /// The URL of the SQS Queue to which to attach the policy
        #[builder(into)]
        pub queue_url: pulumi_wasm_rust::InputOrOutput<String>,
        /// The JSON redrive policy for the SQS queue. Accepts two key/val pairs: `deadLetterTargetArn` and `maxReceiveCount`. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
        #[builder(into)]
        pub redrive_policy: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RedrivePolicyResult {
        /// The URL of the SQS Queue to which to attach the policy
        pub queue_url: pulumi_wasm_rust::Output<String>,
        /// The JSON redrive policy for the SQS queue. Accepts two key/val pairs: `deadLetterTargetArn` and `maxReceiveCount`. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
        pub redrive_policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RedrivePolicyArgs,
    ) -> RedrivePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let queue_url_binding = args.queue_url.get_output(context).get_inner();
        let redrive_policy_binding = args.redrive_policy.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sqs/redrivePolicy:RedrivePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "queueUrl".into(),
                    value: &queue_url_binding,
                },
                register_interface::ObjectField {
                    name: "redrivePolicy".into(),
                    value: &redrive_policy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RedrivePolicyResult {
            queue_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("queueUrl"),
            ),
            redrive_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("redrivePolicy"),
            ),
        }
    }
}
