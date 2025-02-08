#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_policy_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicyAssignmentArgs {
        /// The name of this Policy Assignment. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the scope this Policy Assignment is assigned to. The `scope_id` can be a subscription id, a resource group id, a management group id, or an ID of any resource that is assigned with a policy. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub scope_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPolicyAssignmentResult {
        /// The description of this Policy Assignment.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The display name of this Policy Assignment.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Whether this Policy is enforced or not?
        pub enforce: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::policy::GetPolicyAssignmentIdentity>,
        >,
        /// The Azure Region where the Policy Assignment exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Policy.
        pub metadata: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `non_compliance_message` block as defined below.
        pub non_compliance_messages: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::policy::GetPolicyAssignmentNonComplianceMessage,
            >,
        >,
        /// A `not_scopes` block as defined below.
        pub not_scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A JSON mapping of any Parameters for this Policy.
        pub parameters: pulumi_gestalt_rust::Output<String>,
        /// The ID of the assigned Policy Definition.
        pub policy_definition_id: pulumi_gestalt_rust::Output<String>,
        pub scope_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetPolicyAssignmentArgs,
    ) -> GetPolicyAssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let scope_id_binding = args.scope_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:policy/getPolicyAssignment:getPolicyAssignment".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scopeId".into(),
                    value: &scope_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPolicyAssignmentResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            enforce: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enforce"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            non_compliance_messages: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nonComplianceMessages"),
            ),
            not_scopes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notScopes"),
            ),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            policy_definition_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyDefinitionId"),
            ),
            scope_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scopeId"),
            ),
        }
    }
}
