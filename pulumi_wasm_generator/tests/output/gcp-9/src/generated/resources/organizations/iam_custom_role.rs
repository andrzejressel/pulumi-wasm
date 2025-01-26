/// Allows management of a customized Cloud IAM organization role. For more information see
/// [the official documentation](https://cloud.google.com/iam/docs/understanding-custom-roles)
/// and
/// [API](https://cloud.google.com/iam/reference/rest/v1/organizations.roles).
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
/// This snippet creates a customized IAM organization role.
///
/// ```yaml
/// resources:
///   my-custom-role:
///     type: gcp:organizations:IAMCustomRole
///     properties:
///       roleId: myCustomRole
///       orgId: '123456789'
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
/// Customized IAM organization role can be imported using their URI, e.g.
///
/// ```sh
/// $ pulumi import gcp:organizations/iAMCustomRole:IAMCustomRole my-custom-role organizations/123456789/roles/myCustomRole
/// ```
pub mod iam_custom_role {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IAMCustomRoleArgs {
        /// A human-readable description for the role.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The numeric ID of the organization in which you want to create a custom role.
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The names of the permissions this role grants when bound in an IAM policy. At least one permission must be specified.
        #[builder(into)]
        pub permissions: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The role id to use for this role.
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
        /// The name of the role in the format `organizations/{{org_id}}/roles/{{role_id}}`. Like `id`, this field can be used as a reference in other resources such as IAM role bindings.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The numeric ID of the organization in which you want to create a custom role.
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// The names of the permissions this role grants when bound in an IAM policy. At least one permission must be specified.
        pub permissions: pulumi_wasm_rust::Output<Vec<String>>,
        /// The role id to use for this role.
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
        let org_id_binding = args.org_id.get_output(context).get_inner();
        let permissions_binding = args.permissions.get_output(context).get_inner();
        let role_id_binding = args.role_id.get_output(context).get_inner();
        let stage_binding = args.stage.get_output(context).get_inner();
        let title_binding = args.title.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:organizations/iAMCustomRole:IAMCustomRole".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
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
            org_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("orgId")),
            permissions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            role_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("roleId")),
            stage: pulumi_wasm_rust::__private::into_domain(o.extract_field("stage")),
            title: pulumi_wasm_rust::__private::into_domain(o.extract_field("title")),
        }
    }
}
