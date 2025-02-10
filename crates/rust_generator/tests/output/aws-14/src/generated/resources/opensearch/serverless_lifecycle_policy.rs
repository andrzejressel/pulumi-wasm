/// Resource for managing an AWS OpenSearch Serverless Lifecycle Policy. See AWS documentation for [lifecycle policies](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless-lifecycle.html).
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessLifecyclePolicy
///     properties:
///       name: example
///       type: retention
///       policy:
///         fn::toJSON:
///           Rules:
///             - ResourceType: index
///               Resource:
///                 - index/autoparts-inventory/*
///               MinIndexRetention: 81d
///             - ResourceType: index
///               Resource:
///                 - index/sales/orders*
///               NoMinIndexRetention: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearch Serverless Lifecycle Policy using the `name` and `type` arguments separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/serverlessLifecyclePolicy:ServerlessLifecyclePolicy example example/retention
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod serverless_lifecycle_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessLifecyclePolicyArgs {
        /// Description of the policy.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// JSON policy document to use as the content for the new policy.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of lifecycle policy. Must be `retention`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServerlessLifecyclePolicyResult {
        /// Description of the policy.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// JSON policy document to use as the content for the new policy.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Version of the policy.
        pub policy_version: pulumi_gestalt_rust::Output<String>,
        /// Type of lifecycle policy. Must be `retention`.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServerlessLifecyclePolicyArgs,
    ) -> ServerlessLifecyclePolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opensearch/serverlessLifecyclePolicy:ServerlessLifecyclePolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServerlessLifecyclePolicyResult {
            description: o.get_field("description"),
            name: o.get_field("name"),
            policy: o.get_field("policy"),
            policy_version: o.get_field("policyVersion"),
            type_: o.get_field("type"),
        }
    }
}
