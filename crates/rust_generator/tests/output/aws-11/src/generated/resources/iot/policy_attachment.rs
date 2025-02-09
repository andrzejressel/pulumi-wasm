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
///           function: std:file
///           arguments:
///             input: csr.pem
///           return: result
///       active: true
///   att:
///     type: aws:iot:PolicyAttachment
///     properties:
///       policy: ${pubsubPolicy.name}
///       target: ${cert.arn}
/// variables:
///   pubsub:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - iot:*
///             resources:
///               - '*'
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyAttachmentArgs {
        /// The name of the policy to attach.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The identity to which the policy is attached.
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyAttachmentResult {
        /// The name of the policy to attach.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The identity to which the policy is attached.
        pub target: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyAttachmentArgs,
    ) -> PolicyAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_binding = args.policy.get_output(context);
        let target_binding = args.target.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iot/policyAttachment:PolicyAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: target_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyAttachmentResult {
            policy: o.get_field("policy"),
            target: o.get_field("target"),
        }
    }
}
