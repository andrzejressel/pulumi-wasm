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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ManagedHardwareSecurityModuleRoleAssignmentArgs,
    ) -> ManagedHardwareSecurityModuleRoleAssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let managed_hsm_id_binding_1 = args.managed_hsm_id.get_output(context);
        let managed_hsm_id_binding = managed_hsm_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let principal_id_binding_1 = args.principal_id.get_output(context);
        let principal_id_binding = principal_id_binding_1.get_inner();
        let role_definition_id_binding_1 = args.role_definition_id.get_output(context);
        let role_definition_id_binding = role_definition_id_binding_1.get_inner();
        let scope_binding_1 = args.scope.get_output(context);
        let scope_binding = scope_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/managedHardwareSecurityModuleRoleAssignment:ManagedHardwareSecurityModuleRoleAssignment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managedHsmId".into(),
                    value: &managed_hsm_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ManagedHardwareSecurityModuleRoleAssignmentResult {
            managed_hsm_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedHsmId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            principal_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principalId"),
            ),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            role_definition_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleDefinitionId"),
            ),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
        }
    }
}
