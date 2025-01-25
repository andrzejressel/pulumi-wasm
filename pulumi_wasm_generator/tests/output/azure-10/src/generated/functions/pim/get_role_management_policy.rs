pub mod get_role_management_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRoleManagementPolicyArgs {
        /// The scoped Role Definition ID of the role for which this policy applies.
        #[builder(into)]
        pub role_definition_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The scope to which this Role Management Policy applies. Can refer to a management group, a subscription, a resource group or a resource.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRoleManagementPolicyResult {
        /// An `activation_rules` block as defined below.
        pub activation_rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pim::GetRoleManagementPolicyActivationRule>,
        >,
        /// An `active_assignment_rules` block as defined below.
        pub active_assignment_rules: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::pim::GetRoleManagementPolicyActiveAssignmentRule,
            >,
        >,
        /// (String) The description of this policy.
        pub description: pulumi_wasm_rust::Output<String>,
        /// An `eligible_assignment_rules` block as defined below.
        pub eligible_assignment_rules: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::pim::GetRoleManagementPolicyEligibleAssignmentRule,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// (String) The name of this policy, which is typically a UUID and may change over time.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `notification_rules` block as defined below.
        pub notification_rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pim::GetRoleManagementPolicyNotificationRule>,
        >,
        pub role_definition_id: pulumi_wasm_rust::Output<String>,
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRoleManagementPolicyArgs,
    ) -> GetRoleManagementPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let role_definition_id_binding = args
            .role_definition_id
            .get_output(context)
            .get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:pim/getRoleManagementPolicy:getRoleManagementPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "roleDefinitionId".into(),
                    value: &role_definition_id_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activationRules".into(),
                },
                register_interface::ResultField {
                    name: "activeAssignmentRules".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "eligibleAssignmentRules".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationRules".into(),
                },
                register_interface::ResultField {
                    name: "roleDefinitionId".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRoleManagementPolicyResult {
            activation_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activationRules").unwrap(),
            ),
            active_assignment_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activeAssignmentRules").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            eligible_assignment_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eligibleAssignmentRules").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationRules").unwrap(),
            ),
            role_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleDefinitionId").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
        }
    }
}
