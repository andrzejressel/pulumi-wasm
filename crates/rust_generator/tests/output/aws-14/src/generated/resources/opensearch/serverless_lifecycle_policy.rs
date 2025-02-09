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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServerlessLifecyclePolicyArgs,
    ) -> ServerlessLifecyclePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let policy_binding_1 = args.policy.get_output(context);
        let policy_binding = policy_binding_1.get_inner();
        let type__binding_1 = args.type_.get_output(context);
        let type__binding = type__binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/serverlessLifecyclePolicy:ServerlessLifecyclePolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServerlessLifecyclePolicyResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
            policy_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyVersion"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
