/// Manages a Dev Center Project Environment Type.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleDevCenter:
///     type: azure:devcenter:DevCenter
///     name: example
///     properties:
///       name: example-dc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       identity:
///         type: SystemAssigned
///   exampleEnvironmentType:
///     type: azure:devcenter:EnvironmentType
///     name: example
///     properties:
///       name: example-et
///       devCenterId: ${exampleDevCenter.id}
///   exampleProject:
///     type: azure:devcenter:Project
///     name: example
///     properties:
///       name: example-dcp
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       devCenterId: ${exampleDevCenter.id}
///     options:
///       dependsOn:
///         - ${exampleEnvironmentType}
///   exampleProjectEnvironmentType:
///     type: azure:devcenter:ProjectEnvironmentType
///     name: example
///     properties:
///       name: example-et
///       location: ${example.location}
///       devCenterProjectId: ${exampleProject.id}
///       deploymentTargetId: /subscriptions/${current.subscriptionId}
///       identity:
///         type: SystemAssigned
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// An existing Dev Center Project Environment Type can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devcenter/projectEnvironmentType:ProjectEnvironmentType example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevCenter/projects/project1/environmentTypes/et1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod project_environment_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectEnvironmentTypeArgs {
        /// A list of roles to assign to the environment creator.
        #[builder(into, default)]
        pub creator_role_assignment_roles: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The ID of the subscription that the Environment Type will be mapped to. The environment's resources will be deployed into this subscription.
        #[builder(into)]
        pub deployment_target_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the associated Dev Center Project. Changing this forces a new resource to be created.
        #[builder(into)]
        pub dev_center_project_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `identity` block as defined below.
        #[builder(into)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::devcenter::ProjectEnvironmentTypeIdentity,
        >,
        /// The Azure Region where the Dev Center Project Environment Type should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Dev Center Project Environment Type. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Dev Center Project Environment Type.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `user_role_assignment` block as defined below.
        #[builder(into, default)]
        pub user_role_assignments: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::devcenter::ProjectEnvironmentTypeUserRoleAssignment,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ProjectEnvironmentTypeResult {
        /// A list of roles to assign to the environment creator.
        pub creator_role_assignment_roles: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The ID of the subscription that the Environment Type will be mapped to. The environment's resources will be deployed into this subscription.
        pub deployment_target_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the associated Dev Center Project. Changing this forces a new resource to be created.
        pub dev_center_project_id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            super::super::types::devcenter::ProjectEnvironmentTypeIdentity,
        >,
        /// The Azure Region where the Dev Center Project Environment Type should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Dev Center Project Environment Type. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Dev Center Project Environment Type.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `user_role_assignment` block as defined below.
        pub user_role_assignments: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::devcenter::ProjectEnvironmentTypeUserRoleAssignment,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectEnvironmentTypeArgs,
    ) -> ProjectEnvironmentTypeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let creator_role_assignment_roles_binding = args
            .creator_role_assignment_roles
            .get_output(context);
        let deployment_target_id_binding = args.deployment_target_id.get_output(context);
        let dev_center_project_id_binding = args
            .dev_center_project_id
            .get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_role_assignments_binding = args
            .user_role_assignments
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devcenter/projectEnvironmentType:ProjectEnvironmentType"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "creatorRoleAssignmentRoles".into(),
                    value: creator_role_assignment_roles_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentTargetId".into(),
                    value: deployment_target_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "devCenterProjectId".into(),
                    value: dev_center_project_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userRoleAssignments".into(),
                    value: user_role_assignments_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProjectEnvironmentTypeResult {
            creator_role_assignment_roles: o.get_field("creatorRoleAssignmentRoles"),
            deployment_target_id: o.get_field("deploymentTargetId"),
            dev_center_project_id: o.get_field("devCenterProjectId"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            user_role_assignments: o.get_field("userRoleAssignments"),
        }
    }
}
