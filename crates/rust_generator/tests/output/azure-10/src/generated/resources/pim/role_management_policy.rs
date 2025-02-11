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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod role_management_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoleManagementPolicyArgs {
        /// An `activation_rules` block as defined below.
        #[builder(into, default)]
        pub activation_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pim::RoleManagementPolicyActivationRules>,
        >,
        /// An `active_assignment_rules` block as defined below.
        #[builder(into, default)]
        pub active_assignment_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pim::RoleManagementPolicyActiveAssignmentRules>,
        >,
        /// An `eligible_assignment_rules` block as defined below.
        #[builder(into, default)]
        pub eligible_assignment_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pim::RoleManagementPolicyEligibleAssignmentRules>,
        >,
        /// A `notification_rules` block as defined below.
        #[builder(into, default)]
        pub notification_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pim::RoleManagementPolicyNotificationRules>,
        >,
        /// The scoped Role Definition ID of the role for which this policy will apply. Changing this forces a new resource to be created.
        #[builder(into)]
        pub role_definition_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The scope to which this Role Management Policy will apply. Can refer to a management group, a subscription, a resource group or a resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RoleManagementPolicyResult {
        /// An `activation_rules` block as defined below.
        pub activation_rules: pulumi_gestalt_rust::Output<
            super::super::types::pim::RoleManagementPolicyActivationRules,
        >,
        /// An `active_assignment_rules` block as defined below.
        pub active_assignment_rules: pulumi_gestalt_rust::Output<
            super::super::types::pim::RoleManagementPolicyActiveAssignmentRules,
        >,
        /// (String) The description of this policy.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// An `eligible_assignment_rules` block as defined below.
        pub eligible_assignment_rules: pulumi_gestalt_rust::Output<
            super::super::types::pim::RoleManagementPolicyEligibleAssignmentRules,
        >,
        /// (String) The name of this policy, which is typically a UUID and may change over time.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `notification_rules` block as defined below.
        pub notification_rules: pulumi_gestalt_rust::Output<
            super::super::types::pim::RoleManagementPolicyNotificationRules,
        >,
        /// The scoped Role Definition ID of the role for which this policy will apply. Changing this forces a new resource to be created.
        pub role_definition_id: pulumi_gestalt_rust::Output<String>,
        /// The scope to which this Role Management Policy will apply. Can refer to a management group, a subscription, a resource group or a resource. Changing this forces a new resource to be created.
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RoleManagementPolicyArgs,
    ) -> RoleManagementPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let activation_rules_binding = args.activation_rules.get_output(context);
        let active_assignment_rules_binding = args
            .active_assignment_rules
            .get_output(context);
        let eligible_assignment_rules_binding = args
            .eligible_assignment_rules
            .get_output(context);
        let notification_rules_binding = args.notification_rules.get_output(context);
        let role_definition_id_binding = args.role_definition_id.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:pim/roleManagementPolicy:RoleManagementPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activationRules".into(),
                    value: &activation_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activeAssignmentRules".into(),
                    value: &active_assignment_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eligibleAssignmentRules".into(),
                    value: &eligible_assignment_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationRules".into(),
                    value: &notification_rules_binding.drop_type(),
                },
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
        let o = context.register_resource(request);
        RoleManagementPolicyResult {
            activation_rules: o.get_field("activationRules"),
            active_assignment_rules: o.get_field("activeAssignmentRules"),
            description: o.get_field("description"),
            eligible_assignment_rules: o.get_field("eligibleAssignmentRules"),
            name: o.get_field("name"),
            notification_rules: o.get_field("notificationRules"),
            role_definition_id: o.get_field("roleDefinitionId"),
            scope: o.get_field("scope"),
        }
    }
}
