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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ActiveRoleAssignmentArgs,
    ) -> ActiveRoleAssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let justification_binding = args.justification.get_output(context).get_inner();
        let principal_id_binding = args.principal_id.get_output(context).get_inner();
        let role_definition_id_binding = args
            .role_definition_id
            .get_output(context)
            .get_inner();
        let schedule_binding = args.schedule.get_output(context).get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let ticket_binding = args.ticket.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:pim/activeRoleAssignment:ActiveRoleAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "justification".into(),
                    value: &justification_binding,
                },
                register_interface::ObjectField {
                    name: "principalId".into(),
                    value: &principal_id_binding,
                },
                register_interface::ObjectField {
                    name: "roleDefinitionId".into(),
                    value: &role_definition_id_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "ticket".into(),
                    value: &ticket_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ActiveRoleAssignmentResult {
            justification: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("justification"),
            ),
            principal_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principalId"),
            ),
            principal_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principalType"),
            ),
            role_definition_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleDefinitionId"),
            ),
            schedule: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schedule"),
            ),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
            ticket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ticket"),
            ),
        }
    }
}
