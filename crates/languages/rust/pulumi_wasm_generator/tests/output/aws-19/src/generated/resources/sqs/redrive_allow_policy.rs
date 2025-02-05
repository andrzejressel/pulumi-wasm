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
pub mod redrive_allow_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RedriveAllowPolicyArgs {
        /// The URL of the SQS Queue to which to attach the policy
        #[builder(into)]
        pub queue_url: pulumi_wasm_rust::InputOrOutput<String>,
        /// The JSON redrive allow policy for the SQS queue. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
        #[builder(into)]
        pub redrive_allow_policy: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RedriveAllowPolicyResult {
        /// The URL of the SQS Queue to which to attach the policy
        pub queue_url: pulumi_wasm_rust::Output<String>,
        /// The JSON redrive allow policy for the SQS queue. Learn more in the [Amazon SQS dead-letter queues documentation](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html).
        pub redrive_allow_policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RedriveAllowPolicyArgs,
    ) -> RedriveAllowPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let queue_url_binding = args.queue_url.get_output(context).get_inner();
        let redrive_allow_policy_binding = args
            .redrive_allow_policy
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sqs/redriveAllowPolicy:RedriveAllowPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "queueUrl".into(),
                    value: &queue_url_binding,
                },
                register_interface::ObjectField {
                    name: "redriveAllowPolicy".into(),
                    value: &redrive_allow_policy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RedriveAllowPolicyResult {
            queue_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("queueUrl"),
            ),
            redrive_allow_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("redriveAllowPolicy"),
            ),
        }
    }
}
