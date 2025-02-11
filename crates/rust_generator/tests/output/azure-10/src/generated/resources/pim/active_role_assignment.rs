/// Manages a PIM Active Role Assignment.
///
/// ## Example Usage
///
/// ### Subscription)
///
/// ```yaml
/// resources:
///   exampleStatic:
///     type: time:Static
///     name: example
///   exampleActiveRoleAssignment:
///     type: azure:pim:ActiveRoleAssignment
///     name: example
///     properties:
///       scope: ${primary.id}
///       roleDefinitionId: ${primary.id}${exampleGetRoleDefinition.id}
///       principalId: ${example.objectId}
///       schedule:
///         startDateTime: ${exampleStatic.rfc3339}
///         expiration:
///           durationHours: 8
///       justification: Expiration Duration Set
///       ticket:
///         number: '1'
///         system: example ticket system
/// variables:
///   primary:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   exampleGetRoleDefinition:
///     fn::invoke:
///       function: azure:authorization:getRoleDefinition
///       arguments:
///         name: Reader
/// ```
///
///
/// ### Management Group)
///
/// ```yaml
/// resources:
///   exampleGroup:
///     type: azure:management:Group
///     name: example
///     properties:
///       name: Example-Management-Group
///   exampleStatic:
///     type: time:Static
///     name: example
///   exampleActiveRoleAssignment:
///     type: azure:pim:ActiveRoleAssignment
///     name: example
///     properties:
///       scope: ${exampleGroup.id}
///       roleDefinitionId: ${exampleGetRoleDefinition.id}
///       principalId: ${example.objectId}
///       schedule:
///         startDateTime: ${exampleStatic.rfc3339}
///         expiration:
///           durationHours: 8
///       justification: Expiration Duration Set
///       ticket:
///         number: '1'
///         system: example ticket system
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   exampleGetRoleDefinition:
///     fn::invoke:
///       function: azure:authorization:getRoleDefinition
///       arguments:
///         name: Reader
/// ```
///
/// ## Import
///
/// PIM Active Role Assignments can be imported using the following composite resource ID, e.g.
///
/// ```sh
/// $ pulumi import azure:pim/activeRoleAssignment:ActiveRoleAssignment example /subscriptions/00000000-0000-0000-0000-000000000000|/subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Authorization/roleDefinitions/00000000-0000-0000-0000-000000000000|00000000-0000-0000-0000-000000000000
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod active_role_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActiveRoleAssignmentArgs {
        /// The justification for the role assignment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub justification: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Object ID of the principal for this role assignment. Changing this forces a new resource to be created.
        #[builder(into)]
        pub principal_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The role definition ID for this role assignment. Changing this forces a new resource to be created.
        #[builder(into)]
        pub role_definition_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `schedule` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pim::ActiveRoleAssignmentSchedule>,
        >,
        /// The scope for this role assignment, should be a valid resource ID. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `ticket` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub ticket: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pim::ActiveRoleAssignmentTicket>,
        >,
    }
    #[allow(dead_code)]
    pub struct ActiveRoleAssignmentResult {
        /// The justification for the role assignment. Changing this forces a new resource to be created.
        pub justification: pulumi_gestalt_rust::Output<String>,
        /// Object ID of the principal for this role assignment. Changing this forces a new resource to be created.
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        /// Type of principal to which the role will be assigned.
        pub principal_type: pulumi_gestalt_rust::Output<String>,
        /// The role definition ID for this role assignment. Changing this forces a new resource to be created.
        pub role_definition_id: pulumi_gestalt_rust::Output<String>,
        /// A `schedule` block as defined below. Changing this forces a new resource to be created.
        pub schedule: pulumi_gestalt_rust::Output<
            super::super::types::pim::ActiveRoleAssignmentSchedule,
        >,
        /// The scope for this role assignment, should be a valid resource ID. Changing this forces a new resource to be created.
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// A `ticket` block as defined below. Changing this forces a new resource to be created.
        pub ticket: pulumi_gestalt_rust::Output<
            super::super::types::pim::ActiveRoleAssignmentTicket,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ActiveRoleAssignmentArgs,
    ) -> ActiveRoleAssignmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let justification_binding = args.justification.get_output(context);
        let principal_id_binding = args.principal_id.get_output(context);
        let role_definition_id_binding = args.role_definition_id.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let ticket_binding = args.ticket.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:pim/activeRoleAssignment:ActiveRoleAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "justification".into(),
                    value: &justification_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalId".into(),
                    value: &principal_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleDefinitionId".into(),
                    value: &role_definition_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: &scope_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ticket".into(),
                    value: &ticket_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ActiveRoleAssignmentResult {
            justification: o.get_field("justification"),
            principal_id: o.get_field("principalId"),
            principal_type: o.get_field("principalType"),
            role_definition_id: o.get_field("roleDefinitionId"),
            schedule: o.get_field("schedule"),
            scope: o.get_field("scope"),
            ticket: o.get_field("ticket"),
        }
    }
}
