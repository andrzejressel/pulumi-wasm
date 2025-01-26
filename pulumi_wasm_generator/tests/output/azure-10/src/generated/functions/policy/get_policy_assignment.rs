pub mod get_policy_assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicyAssignmentArgs {
        /// The name of this Policy Assignment. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the scope this Policy Assignment is assigned to. The `scope_id` can be a subscription id, a resource group id, a management group id, or an ID of any resource that is assigned with a policy. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub scope_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPolicyAssignmentResult {
        /// The description of this Policy Assignment.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The display name of this Policy Assignment.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Whether this Policy is enforced or not?
        pub enforce: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::policy::GetPolicyAssignmentIdentity>,
        >,
        /// The Azure Region where the Policy Assignment exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Policy.
        pub metadata: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `non_compliance_message` block as defined below.
        pub non_compliance_messages: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::policy::GetPolicyAssignmentNonComplianceMessage,
            >,
        >,
        /// A `not_scopes` block as defined below.
        pub not_scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// A JSON mapping of any Parameters for this Policy.
        pub parameters: pulumi_wasm_rust::Output<String>,
        /// The ID of the assigned Policy Definition.
        pub policy_definition_id: pulumi_wasm_rust::Output<String>,
        pub scope_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPolicyAssignmentArgs,
    ) -> GetPolicyAssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "enforce".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nonComplianceMessages".into(),
                },
                register_interface::ResultField {
                    name: "notScopes".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "policyDefinitionId".into(),
                },
                register_interface::ResultField {
                    name: "scopeId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPolicyAssignmentResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            enforce: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enforce").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            non_compliance_messages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nonComplianceMessages").unwrap(),
            ),
            not_scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notScopes").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            policy_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDefinitionId").unwrap(),
            ),
            scope_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopeId").unwrap(),
            ),
        }
    }
}
