/// Allows you to set a policy of an SQS Queue
/// while referencing ARN of the queue within the policy.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sqs:SendMessage",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("ArnEquals").values(vec!["${example.arn}",])
///                     .variable("aws:SourceArn").build_struct(),]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["*",]). type ("*").build_struct(),])
///                     .resources(vec!["${q.arn}",]).sid("First").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let q = queue::create("q", QueueArgs::builder().name("examplequeue").build_struct());
///     let testQueuePolicy = queue_policy::create(
///         "testQueuePolicy",
///         QueuePolicyArgs::builder()
///             .policy("${test.json}")
///             .queue_url("${q.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SQS Queue Policies using the queue URL. For example:
///
/// ```sh
/// $ pulumi import aws:sqs/queuePolicy:QueuePolicy test https://queue.amazonaws.com/123456789012/myqueue
/// ```
pub mod queue_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueuePolicyArgs {
        /// The JSON policy for the SQS queue.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The URL of the SQS Queue to which to attach the policy
        #[builder(into)]
        pub queue_url: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct QueuePolicyResult {
        /// The JSON policy for the SQS queue.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The URL of the SQS Queue to which to attach the policy
        pub queue_url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: QueuePolicyArgs) -> QueuePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_inner();
        let queue_url_binding = args.queue_url.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sqs/queuePolicy:QueuePolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "queueUrl".into(),
                    value: &queue_url_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "queueUrl".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        QueuePolicyResult {
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            queue_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queueUrl").unwrap(),
            ),
        }
    }
}
