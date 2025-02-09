#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_auth_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAuthPolicyArgs {
        /// The auth policy. The policy string in JSON must not contain newlines or blank lines.
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID or Amazon Resource Name (ARN) of the service network or service for which the policy is created.
        #[builder(into)]
        pub resource_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The state of the auth policy. The auth policy is only active when the auth type is set to AWS_IAM. If you provide a policy, then authentication and authorization decisions are made based on this policy and the client's IAM policy. If the Auth type is NONE, then, any auth policy you provide will remain inactive.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAuthPolicyResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The auth policy. The policy string in JSON must not contain newlines or blank lines.
        pub policy: pulumi_gestalt_rust::Output<Option<String>>,
        pub resource_identifier: pulumi_gestalt_rust::Output<String>,
        /// The state of the auth policy. The auth policy is only active when the auth type is set to AWS_IAM. If you provide a policy, then authentication and authorization decisions are made based on this policy and the client's IAM policy. If the Auth type is NONE, then, any auth policy you provide will remain inactive.
        pub state: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAuthPolicyArgs,
    ) -> GetAuthPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let policy_binding_1 = args.policy.get_output(context);
        let policy_binding = policy_binding_1.get_inner();
        let resource_identifier_binding_1 = args.resource_identifier.get_output(context);
        let resource_identifier_binding = resource_identifier_binding_1.get_inner();
        let state_binding_1 = args.state.get_output(context);
        let state_binding = state_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:vpclattice/getAuthPolicy:getAuthPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "resourceIdentifier".into(),
                    value: &resource_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAuthPolicyResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
            resource_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceIdentifier"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
        }
    }
}
