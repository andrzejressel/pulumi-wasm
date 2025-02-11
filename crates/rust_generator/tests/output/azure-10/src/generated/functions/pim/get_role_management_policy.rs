#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_role_management_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRoleManagementPolicyArgs {
        /// The scoped Role Definition ID of the role for which this policy applies.
        #[builder(into)]
        pub role_definition_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The scope to which this Role Management Policy applies. Can refer to a management group, a subscription, a resource group or a resource.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRoleManagementPolicyResult {
        /// An `activation_rules` block as defined below.
        pub activation_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::pim::GetRoleManagementPolicyActivationRule>,
        >,
        /// An `active_assignment_rules` block as defined below.
        pub active_assignment_rules: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::pim::GetRoleManagementPolicyActiveAssignmentRule,
            >,
        >,
        /// (String) The description of this policy.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// An `eligible_assignment_rules` block as defined below.
        pub eligible_assignment_rules: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::pim::GetRoleManagementPolicyEligibleAssignmentRule,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// (String) The name of this policy, which is typically a UUID and may change over time.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `notification_rules` block as defined below.
        pub notification_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::pim::GetRoleManagementPolicyNotificationRule>,
        >,
        pub role_definition_id: pulumi_gestalt_rust::Output<String>,
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRoleManagementPolicyArgs,
    ) -> GetRoleManagementPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let role_definition_id_binding = args.role_definition_id.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:pim/getRoleManagementPolicy:getRoleManagementPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleDefinitionId".into(),
                    value: &role_definition_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: &scope_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRoleManagementPolicyResult {
            activation_rules: o.get_field("activationRules"),
            active_assignment_rules: o.get_field("activeAssignmentRules"),
            description: o.get_field("description"),
            eligible_assignment_rules: o.get_field("eligibleAssignmentRules"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            notification_rules: o.get_field("notificationRules"),
            role_definition_id: o.get_field("roleDefinitionId"),
            scope: o.get_field("scope"),
        }
    }
}
