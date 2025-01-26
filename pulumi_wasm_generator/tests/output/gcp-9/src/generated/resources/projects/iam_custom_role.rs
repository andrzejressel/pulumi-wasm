/// Allows management of a customized Cloud IAM project role. For more information see
/// [the official documentation](https://cloud.google.com/iam/docs/understanding-custom-roles)
/// and
/// [API](https://cloud.google.com/iam/reference/rest/v1/projects.roles).
///
/// > **Warning:** Note that custom roles in GCP have the concept of a soft-delete. There are two issues that may arise
///  from this and how roles are propagated. 1) creating a role may involve undeleting and then updating a role with the
///  same name, possibly causing confusing behavior between undelete and update. 2) A deleted role is permanently deleted
///  after 7 days, but it can take up to 30 more days (i.e. between 7 and 37 days after deletion) before the role name is
///  made available again. This means a deleted role that has been deleted for more than 7 days cannot be changed at all
///  by the provider, and new roles cannot share that name.
///
/// ## Example Usage
///
/// This snippet creates a customized IAM role.
///
/// ```yaml
/// resources:
///   my-custom-role:
///     type: gcp:projects:IAMCustomRole
///     properties:
///       roleId: myCustomRole
///       title: My Custom Role
///       description: A description
///       permissions:
///         - iam.roles.list
///         - iam.roles.create
///         - iam.roles.delete
/// ```
///
/// ## Import
///
/// Custom Roles can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/roles/{{role_id}}`
///
/// * `{{project}}/{{role_id}}`
///
/// * `{{role_id}}`
///
/// When using the `pulumi import` command, Custom Roles can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:projects/iAMCustomRole:IAMCustomRole default projects/{{project}}/roles/{{role_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:projects/iAMCustomRole:IAMCustomRole default {{project}}/{{role_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:projects/iAMCustomRole:IAMCustomRole default {{role_id}}
/// ```
///
pub mod iam_custom_role {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IAMCustomRoleArgs {
        /// A human-readable description for the role.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The names of the permissions this role grants when bound in an IAM policy. At least one permission must be specified.
        #[builder(into)]
        pub permissions: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The project that the custom role will be created in.
        /// Defaults to the provider project configuration.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The camel case role id to use for this role. Cannot contain `-` characters.
        #[builder(into, default)]
        pub role_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The current launch stage of the role.
        /// Defaults to `GA`.
        /// List of possible stages is [here](https://cloud.google.com/iam/reference/rest/v1/organizations.roles#Role.RoleLaunchStage).
        #[builder(into, default)]
        pub stage: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A human-readable title for the role.
        #[builder(into)]
        pub title: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IAMCustomRoleResult {
        /// (Optional) The current deleted state of the role.
        pub deleted: pulumi_wasm_rust::Output<bool>,
        /// A human-readable description for the role.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the role in the format `projects/{{project}}/roles/{{role_id}}`. Like `id`, this field can be used as a reference in other resources such as IAM role bindings.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The names of the permissions this role grants when bound in an IAM policy. At least one permission must be specified.
        pub permissions: pulumi_wasm_rust::Output<Vec<String>>,
        /// The project that the custom role will be created in.
        /// Defaults to the provider project configuration.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The camel case role id to use for this role. Cannot contain `-` characters.
        pub role_id: pulumi_wasm_rust::Output<String>,
        /// The current launch stage of the role.
        /// Defaults to `GA`.
        /// List of possible stages is [here](https://cloud.google.com/iam/reference/rest/v1/organizations.roles#Role.RoleLaunchStage).
        pub stage: pulumi_wasm_rust::Output<Option<String>>,
        /// A human-readable title for the role.
        pub title: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IAMCustomRoleArgs,
    ) -> IAMCustomRoleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let permissions_binding = args.permissions.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let role_id_binding = args.role_id.get_output(context).get_inner();
        let stage_binding = args.stage.get_output(context).get_inner();
        let title_binding = args.title.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:projects/iAMCustomRole:IAMCustomRole".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "roleId".into(),
                    value: &role_id_binding,
                },
                register_interface::ObjectField {
                    name: "stage".into(),
                    value: &stage_binding,
                },
                register_interface::ObjectField {
                    name: "title".into(),
                    value: &title_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IAMCustomRoleResult {
            deleted: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deleted"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            permissions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            role_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("roleId")),
            stage: pulumi_wasm_rust::__private::into_domain(o.extract_field("stage")),
            title: pulumi_wasm_rust::__private::into_domain(o.extract_field("title")),
        }
    }
}
