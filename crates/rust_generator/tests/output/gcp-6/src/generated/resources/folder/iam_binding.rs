/// Allows creation and management of a single binding within IAM policy for
/// an existing Google Cloud Platform folder.
///
/// > **Note:** This resource _must not_ be used in conjunction with
///    `gcp.folder.IAMPolicy` or they will fight over what your policy
///    should be.
///
/// > **Note:** On create, this resource will overwrite members of any existing roles.
///     Use `pulumi import` and inspect the output to ensure
///     your existing members are preserved.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let admin = iam_binding::create(
///         "admin",
///         IamBindingArgs::builder()
///             .folder("${department1.name}")
///             .members(vec!["user:alice@gmail.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
///     let department1 = folder::create(
///         "department1",
///         FolderArgs::builder()
///             .display_name("Department 1")
///             .parent("organizations/1234567")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// IAM binding imports use space-delimited identifiers; first the resource in question and then the role.  These bindings can be imported using the `folder` and role, e.g.
///
/// ```sh
/// $ pulumi import gcp:folder/iAMBinding:IAMBinding viewer "folder-name roles/viewer"
/// ```
///
/// -> **Custom Roles**: If you're importing a IAM binding with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IAMBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::folder::IamBindingCondition>,
        >,
        /// The resource name of the folder the policy is attached to. Its format is folders/{folder_id}.
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An array of identities that will be granted the privilege in the `role`.
        /// Each entry can have one of the following values:
        /// * **user:{emailid}**: An email address that is associated with a specific Google account. For example, alice@gmail.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * For more details on format and restrictions see https://cloud.google.com/billing/reference/rest/v1/Policy#Binding
        #[builder(into)]
        pub members: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The role that should be applied. Only one
        /// `gcp.folder.IAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IAMBindingResult {
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::folder::IamBindingCondition>,
        >,
        /// (Computed) The etag of the folder's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the folder the policy is attached to. Its format is folders/{folder_id}.
        pub folder: pulumi_gestalt_rust::Output<String>,
        /// An array of identities that will be granted the privilege in the `role`.
        /// Each entry can have one of the following values:
        /// * **user:{emailid}**: An email address that is associated with a specific Google account. For example, alice@gmail.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * For more details on format and restrictions see https://cloud.google.com/billing/reference/rest/v1/Policy#Binding
        pub members: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The role that should be applied. Only one
        /// `gcp.folder.IAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IAMBindingArgs,
    ) -> IAMBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let folder_binding = args.folder.get_output(context);
        let members_binding = args.members.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:folder/iAMBinding:IAMBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: &folder_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "members".into(),
                    value: &members_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        IAMBindingResult {
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            folder: o.get_field("folder"),
            members: o.get_field("members"),
            role: o.get_field("role"),
        }
    }
}
