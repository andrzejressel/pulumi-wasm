/// Manage a role policy for an Azure Management Group, Subscription, Resource Group or resource.
///
/// ## Example Usage
///
/// ### Resource Group
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: East US
///   exampleRoleManagementPolicy:
///     type: azure:pim:RoleManagementPolicy
///     name: example
///     properties:
///       scope: ${test.id}
///       roleDefinitionId: ${contributor.id}
///       activeAssignmentRules:
///         expireAfter: P365D
///       eligibleAssignmentRules:
///         expirationRequired: false
///       activationRules:
///         maximumDuration: PT1H
///         requireApproval: true
///         approvalStage:
///           primaryApprovers:
///             - objectId: ${approvers.objectId}
///               type: Group
///       notificationRules:
///         eligibleAssignments:
///           approverNotifications:
///             notificationLevel: Critical
///             defaultRecipients: false
///             additionalRecipients:
///               - someone@example.com
///         eligibleActivations:
///           assigneeNotifications:
///             notificationLevel: All
///             defaultRecipients: true
///             additionalRecipients:
///               - someone.else@example.com
/// variables:
///   rgContributor:
///     fn::invoke:
///       function: azure:authorization:getRoleDefinition
///       arguments:
///         name: Contributor
///         scope: ${example.id}
///   approvers:
///     fn::invoke:
///       function: azuread:getGroup
///       arguments:
///         displayName: Example Approver Group
/// ```
///
/// ### Management Group
///
/// ```yaml
/// resources:
///   example:
///     type: azure:management:Group
///     properties:
///       name: example-group
///   exampleRoleManagementPolicy:
///     type: azure:pim:RoleManagementPolicy
///     name: example
///     properties:
///       scope: ${example.id}
///       roleDefinitionId: ${mgContributor.id}
///       eligibleAssignmentRules:
///         expirationRequired: false
///       activeAssignmentRules:
///         expireAfter: P90D
///       activationRules:
///         maximumDuration: PT1H
///         requireApproval: true
///       notificationRules:
///         activeAssignments:
///           adminNotifications:
///             notificationLevel: Critical
///             defaultRecipients: false
///             additionalRecipients:
///               - someone@example.com
/// variables:
///   mgContributor:
///     fn::invoke:
///       function: azure:authorization:getRoleDefinition
///       arguments:
///         name: Contributor
///         scope: ${example.id}
/// ```
///
/// ## Import
///
/// Because these policies are created automatically by Azure, they will auto-import on first use. They can be imported using the `resource id` of the role definition, combined with the scope id, e.g.
///
/// ```sh
/// $ pulumi import azure:pim/roleManagementPolicy:RoleManagementPolicy example "/subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Authorization/roleDefinitions/00000000-0000-0000-0000-000000000000|<scope>"
/// ```
///
pub mod role_management_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoleManagementPolicyArgs {
        /// An `activation_rules` block as defined below.
        #[builder(into, default)]
        pub activation_rules: pulumi_wasm_rust::Output<
            Option<super::super::types::pim::RoleManagementPolicyActivationRules>,
        >,
        /// An `active_assignment_rules` block as defined below.
        #[builder(into, default)]
        pub active_assignment_rules: pulumi_wasm_rust::Output<
            Option<super::super::types::pim::RoleManagementPolicyActiveAssignmentRules>,
        >,
        /// An `eligible_assignment_rules` block as defined below.
        #[builder(into, default)]
        pub eligible_assignment_rules: pulumi_wasm_rust::Output<
            Option<super::super::types::pim::RoleManagementPolicyEligibleAssignmentRules>,
        >,
        /// A `notification_rules` block as defined below.
        #[builder(into, default)]
        pub notification_rules: pulumi_wasm_rust::Output<
            Option<super::super::types::pim::RoleManagementPolicyNotificationRules>,
        >,
        /// The scoped Role Definition ID of the role for which this policy will apply. Changing this forces a new resource to be created.
        #[builder(into)]
        pub role_definition_id: pulumi_wasm_rust::Output<String>,
        /// The scope to which this Role Management Policy will apply. Can refer to a management group, a subscription, a resource group or a resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RoleManagementPolicyResult {
        /// An `activation_rules` block as defined below.
        pub activation_rules: pulumi_wasm_rust::Output<
            super::super::types::pim::RoleManagementPolicyActivationRules,
        >,
        /// An `active_assignment_rules` block as defined below.
        pub active_assignment_rules: pulumi_wasm_rust::Output<
            super::super::types::pim::RoleManagementPolicyActiveAssignmentRules,
        >,
        /// (String) The description of this policy.
        pub description: pulumi_wasm_rust::Output<String>,
        /// An `eligible_assignment_rules` block as defined below.
        pub eligible_assignment_rules: pulumi_wasm_rust::Output<
            super::super::types::pim::RoleManagementPolicyEligibleAssignmentRules,
        >,
        /// (String) The name of this policy, which is typically a UUID and may change over time.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `notification_rules` block as defined below.
        pub notification_rules: pulumi_wasm_rust::Output<
            super::super::types::pim::RoleManagementPolicyNotificationRules,
        >,
        /// The scoped Role Definition ID of the role for which this policy will apply. Changing this forces a new resource to be created.
        pub role_definition_id: pulumi_wasm_rust::Output<String>,
        /// The scope to which this Role Management Policy will apply. Can refer to a management group, a subscription, a resource group or a resource. Changing this forces a new resource to be created.
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RoleManagementPolicyArgs,
    ) -> RoleManagementPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let activation_rules_binding = args.activation_rules.get_inner();
        let active_assignment_rules_binding = args.active_assignment_rules.get_inner();
        let eligible_assignment_rules_binding = args
            .eligible_assignment_rules
            .get_inner();
        let notification_rules_binding = args.notification_rules.get_inner();
        let role_definition_id_binding = args.role_definition_id.get_inner();
        let scope_binding = args.scope.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:pim/roleManagementPolicy:RoleManagementPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activationRules".into(),
                    value: &activation_rules_binding,
                },
                register_interface::ObjectField {
                    name: "activeAssignmentRules".into(),
                    value: &active_assignment_rules_binding,
                },
                register_interface::ObjectField {
                    name: "eligibleAssignmentRules".into(),
                    value: &eligible_assignment_rules_binding,
                },
                register_interface::ObjectField {
                    name: "notificationRules".into(),
                    value: &notification_rules_binding,
                },
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RoleManagementPolicyResult {
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