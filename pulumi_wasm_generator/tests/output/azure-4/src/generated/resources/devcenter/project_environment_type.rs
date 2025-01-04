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
pub mod project_environment_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectEnvironmentTypeArgs {
        /// A list of roles to assign to the environment creator.
        #[builder(into, default)]
        pub creator_role_assignment_roles: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the subscription that the Environment Type will be mapped to. The environment's resources will be deployed into this subscription.
        #[builder(into)]
        pub deployment_target_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the associated Dev Center Project. Changing this forces a new resource to be created.
        #[builder(into)]
        pub dev_center_project_id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        #[builder(into)]
        pub identity: pulumi_wasm_rust::Output<
            super::super::types::devcenter::ProjectEnvironmentTypeIdentity,
        >,
        /// The Azure Region where the Dev Center Project Environment Type should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of this Dev Center Project Environment Type. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Dev Center Project Environment Type.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `user_role_assignment` block as defined below.
        #[builder(into, default)]
        pub user_role_assignments: pulumi_wasm_rust::Output<
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
        pub creator_role_assignment_roles: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the subscription that the Environment Type will be mapped to. The environment's resources will be deployed into this subscription.
        pub deployment_target_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the associated Dev Center Project. Changing this forces a new resource to be created.
        pub dev_center_project_id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            super::super::types::devcenter::ProjectEnvironmentTypeIdentity,
        >,
        /// The Azure Region where the Dev Center Project Environment Type should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Dev Center Project Environment Type. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Dev Center Project Environment Type.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `user_role_assignment` block as defined below.
        pub user_role_assignments: pulumi_wasm_rust::Output<
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
        name: &str,
        args: ProjectEnvironmentTypeArgs,
    ) -> ProjectEnvironmentTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let creator_role_assignment_roles_binding = args
            .creator_role_assignment_roles
            .get_inner();
        let deployment_target_id_binding = args.deployment_target_id.get_inner();
        let dev_center_project_id_binding = args.dev_center_project_id.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let user_role_assignments_binding = args.user_role_assignments.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:devcenter/projectEnvironmentType:ProjectEnvironmentType"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "creatorRoleAssignmentRoles".into(),
                    value: &creator_role_assignment_roles_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentTargetId".into(),
                    value: &deployment_target_id_binding,
                },
                register_interface::ObjectField {
                    name: "devCenterProjectId".into(),
                    value: &dev_center_project_id_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userRoleAssignments".into(),
                    value: &user_role_assignments_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "creatorRoleAssignmentRoles".into(),
                },
                register_interface::ResultField {
                    name: "deploymentTargetId".into(),
                },
                register_interface::ResultField {
                    name: "devCenterProjectId".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "userRoleAssignments".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProjectEnvironmentTypeResult {
            creator_role_assignment_roles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creatorRoleAssignmentRoles").unwrap(),
            ),
            deployment_target_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentTargetId").unwrap(),
            ),
            dev_center_project_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("devCenterProjectId").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            user_role_assignments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userRoleAssignments").unwrap(),
            ),
        }
    }
}
