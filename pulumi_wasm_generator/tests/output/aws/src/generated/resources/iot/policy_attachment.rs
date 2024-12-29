/// Provides an IoT policy attachment.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   pubsubPolicy:
///     type: aws:iot:Policy
///     name: pubsub
///     properties:
///       name: PubSubToAnyTopic
///       policy: ${pubsub.json}
///   cert:
///     type: aws:iot:Certificate
///     properties:
///       csr:
///         fn::invoke:
///           Function: std:file
///           Arguments:
///             input: csr.pem
///           Return: result
///       active: true
///   att:
///     type: aws:iot:PolicyAttachment
///     properties:
///       policy: ${pubsubPolicy.name}
///       target: ${cert.arn}
/// variables:
///   pubsub:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - iot:*
///             resources:
///               - '*'
/// ```
pub mod policy_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyAttachmentArgs {
        /// The name of the policy to attach.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The identity to which the policy is attached.
        #[builder(into)]
        pub target: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyAttachmentResult {
        /// The name of the policy to attach.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The identity to which the policy is attached.
        pub target: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PolicyAttachmentArgs) -> PolicyAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_inner();
        let target_binding = args.target.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/policyAttachment:PolicyAttachment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "target".into(),
                    value: &target_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "target".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicyAttachmentResult {
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("target").unwrap(),
            ),
        }
    }
}
