/// Manages a Managed Hardware Security Module Role Assignment.
///
/// ## Import
///
/// Managed Hardware Security Modules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/managedHardwareSecurityModuleRoleAssignment:ManagedHardwareSecurityModuleRoleAssignment example https://0000.managedhsm.azure.net///RoleAssignment/00000000-0000-0000-0000-000000000000
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_hardware_security_module_role_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleRoleAssignmentArgs {
        /// The ID of a Managed Hardware Security Module resource. Changing this forces a new Managed Hardware Security Module to be created.
        /// *
        #[builder(into)]
        pub managed_hsm_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name in GUID notation which should be used for this Managed Hardware Security Module Role Assignment. Changing this forces a new Managed Hardware Security Module to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The principal ID to be assigned to this role. It can point to a user, service principal, or security group. Changing this forces a new Managed Hardware Security Module to be created.
        #[builder(into)]
        pub principal_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the role definition to assign. Changing this forces a new Managed Hardware Security Module to be created.
        #[builder(into)]
        pub role_definition_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the scope to create the role assignment. Changing this forces a new Managed Hardware Security Module to be created.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleRoleAssignmentResult {
        /// The ID of a Managed Hardware Security Module resource. Changing this forces a new Managed Hardware Security Module to be created.
        /// *
        pub managed_hsm_id: pulumi_gestalt_rust::Output<String>,
        /// The name in GUID notation which should be used for this Managed Hardware Security Module Role Assignment. Changing this forces a new Managed Hardware Security Module to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The principal ID to be assigned to this role. It can point to a user, service principal, or security group. Changing this forces a new Managed Hardware Security Module to be created.
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        /// (Deprecated) The resource id of created assignment resource.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the role definition to assign. Changing this forces a new Managed Hardware Security Module to be created.
        pub role_definition_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the scope to create the role assignment. Changing this forces a new Managed Hardware Security Module to be created.
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedHardwareSecurityModuleRoleAssignmentArgs,
    ) -> ManagedHardwareSecurityModuleRoleAssignmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let managed_hsm_id_binding = args.managed_hsm_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let principal_id_binding = args.principal_id.get_output(context);
        let role_definition_id_binding = args.role_definition_id.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:keyvault/managedHardwareSecurityModuleRoleAssignment:ManagedHardwareSecurityModuleRoleAssignment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedHsmId".into(),
                    value: managed_hsm_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalId".into(),
                    value: principal_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleDefinitionId".into(),
                    value: role_definition_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedHardwareSecurityModuleRoleAssignmentResult {
            managed_hsm_id: o.get_field("managedHsmId"),
            name: o.get_field("name"),
            principal_id: o.get_field("principalId"),
            resource_id: o.get_field("resourceId"),
            role_definition_id: o.get_field("roleDefinitionId"),
            scope: o.get_field("scope"),
        }
    }
}
