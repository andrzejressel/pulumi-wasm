/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleContainer:
///     type: aws:mediastore:Container
///     name: example
///     properties:
///       name: example
///   exampleContainerPolicy:
///     type: aws:mediastore:ContainerPolicy
///     name: example
///     properties:
///       containerName: ${exampleContainer.name}
///       policy: ${example.json}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   currentGetCallerIdentity:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: MediaStoreFullAccess
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - arn:aws:iam::${currentGetCallerIdentity.accountId}:root
///             actions:
///               - mediastore:*
///             resources:
///               - arn:aws:mediastore:${current.name}:${currentGetCallerIdentity.accountId}:container/${exampleContainer.name}/*
///             conditions:
///               - test: Bool
///                 variable: aws:SecureTransport
///                 values:
///                   - 'true'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MediaStore Container Policy using the MediaStore Container Name. For example:
///
/// ```sh
/// $ pulumi import aws:mediastore/containerPolicy:ContainerPolicy example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod container_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerPolicyArgs {
        /// The name of the container.
        #[builder(into)]
        pub container_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The contents of the policy.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ContainerPolicyResult {
        /// The name of the container.
        pub container_name: pulumi_gestalt_rust::Output<String>,
        /// The contents of the policy.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContainerPolicyArgs,
    ) -> ContainerPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_name_binding = args.container_name.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:mediastore/containerPolicy:ContainerPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerName".into(),
                    value: container_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ContainerPolicyResult {
            container_name: o.get_field("containerName"),
            policy: o.get_field("policy"),
        }
    }
}
