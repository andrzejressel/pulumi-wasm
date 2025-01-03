/// Three different resources help you manage your IAM policy for Cloud Storage ManagedFolder. Each of these resources serves a different use case:
///
/// * `gcp.storage.ManagedFolderIamPolicy`: Authoritative. Sets the IAM policy for the managedfolder and replaces any existing policy already attached.
/// * `gcp.storage.ManagedFolderIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the managedfolder are preserved.
/// * `gcp.storage.ManagedFolderIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the managedfolder are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.storage.ManagedFolderIamPolicy`: Retrieves the IAM policy for the managedfolder
///
/// > **Note:** `gcp.storage.ManagedFolderIamPolicy` **cannot** be used in conjunction with `gcp.storage.ManagedFolderIamBinding` and `gcp.storage.ManagedFolderIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.storage.ManagedFolderIamBinding` resources **can be** used in conjunction with `gcp.storage.ManagedFolderIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.storage.ManagedFolderIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:storage:ManagedFolderIamPolicy
///     properties:
///       bucket: ${folder.bucket}
///       managedFolder: ${folder.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/storage.admin
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:storage:ManagedFolderIamPolicy
///     properties:
///       bucket: ${folder.bucket}
///       managedFolder: ${folder.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/storage.admin
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.storage.ManagedFolderIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = managed_folder_iam_binding::create(
///         "binding",
///         ManagedFolderIamBindingArgs::builder()
///             .bucket("${folder.bucket}")
///             .managed_folder("${folder.name}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/storage.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = managed_folder_iam_binding::create(
///         "binding",
///         ManagedFolderIamBindingArgs::builder()
///             .bucket("${folder.bucket}")
///             .condition(
///                 ManagedFolderIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .managed_folder("${folder.name}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/storage.admin")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.storage.ManagedFolderIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = managed_folder_iam_member::create(
///         "member",
///         ManagedFolderIamMemberArgs::builder()
///             .bucket("${folder.bucket}")
///             .managed_folder("${folder.name}")
///             .member("user:jane@example.com")
///             .role("roles/storage.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = managed_folder_iam_member::create(
///         "member",
///         ManagedFolderIamMemberArgs::builder()
///             .bucket("${folder.bucket}")
///             .condition(
///                 ManagedFolderIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .managed_folder("${folder.name}")
///             .member("user:jane@example.com")
///             .role("roles/storage.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## > **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
/// -
///
/// # IAM policy for Cloud Storage ManagedFolder
/// Three different resources help you manage your IAM policy for Cloud Storage ManagedFolder. Each of these resources serves a different use case:
///
/// * `gcp.storage.ManagedFolderIamPolicy`: Authoritative. Sets the IAM policy for the managedfolder and replaces any existing policy already attached.
/// * `gcp.storage.ManagedFolderIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the managedfolder are preserved.
/// * `gcp.storage.ManagedFolderIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the managedfolder are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.storage.ManagedFolderIamPolicy`: Retrieves the IAM policy for the managedfolder
///
/// > **Note:** `gcp.storage.ManagedFolderIamPolicy` **cannot** be used in conjunction with `gcp.storage.ManagedFolderIamBinding` and `gcp.storage.ManagedFolderIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.storage.ManagedFolderIamBinding` resources **can be** used in conjunction with `gcp.storage.ManagedFolderIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.storage.ManagedFolderIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:storage:ManagedFolderIamPolicy
///     properties:
///       bucket: ${folder.bucket}
///       managedFolder: ${folder.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/storage.admin
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:storage:ManagedFolderIamPolicy
///     properties:
///       bucket: ${folder.bucket}
///       managedFolder: ${folder.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/storage.admin
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.storage.ManagedFolderIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = managed_folder_iam_binding::create(
///         "binding",
///         ManagedFolderIamBindingArgs::builder()
///             .bucket("${folder.bucket}")
///             .managed_folder("${folder.name}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/storage.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = managed_folder_iam_binding::create(
///         "binding",
///         ManagedFolderIamBindingArgs::builder()
///             .bucket("${folder.bucket}")
///             .condition(
///                 ManagedFolderIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .managed_folder("${folder.name}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/storage.admin")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.storage.ManagedFolderIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = managed_folder_iam_member::create(
///         "member",
///         ManagedFolderIamMemberArgs::builder()
///             .bucket("${folder.bucket}")
///             .managed_folder("${folder.name}")
///             .member("user:jane@example.com")
///             .role("roles/storage.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = managed_folder_iam_member::create(
///         "member",
///         ManagedFolderIamMemberArgs::builder()
///             .bucket("${folder.bucket}")
///             .condition(
///                 ManagedFolderIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .managed_folder("${folder.name}")
///             .member("user:jane@example.com")
///             .role("roles/storage.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * b/{{bucket}}/managedFolders/{{managed_folder}}
///
/// * {{bucket}}/{{managed_folder}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud Storage managedfolder IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:storage/managedFolderIamMember:ManagedFolderIamMember editor "b/{{bucket}}/managedFolders/{{managed_folder}} roles/storage.objectViewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:storage/managedFolderIamMember:ManagedFolderIamMember editor "b/{{bucket}}/managedFolders/{{managed_folder}} roles/storage.objectViewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:storage/managedFolderIamMember:ManagedFolderIamMember editor b/{{bucket}}/managedFolders/{{managed_folder}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod managed_folder_iam_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedFolderIamMemberArgs {
        /// The name of the bucket that contains the managed folder. Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::ManagedFolderIamMemberCondition>,
        >,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub managed_folder: pulumi_wasm_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        #[builder(into)]
        pub member: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.storage.ManagedFolderIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedFolderIamMemberResult {
        /// The name of the bucket that contains the managed folder. Used to find the parent resource to bind the IAM policy to
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::ManagedFolderIamMemberCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub managed_folder: pulumi_wasm_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        pub member: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.storage.ManagedFolderIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ManagedFolderIamMemberArgs,
    ) -> ManagedFolderIamMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let condition_binding = args.condition.get_inner();
        let managed_folder_binding = args.managed_folder.get_inner();
        let member_binding = args.member.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:storage/managedFolderIamMember:ManagedFolderIamMember".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "managedFolder".into(),
                    value: &managed_folder_binding,
                },
                register_interface::ObjectField {
                    name: "member".into(),
                    value: &member_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "managedFolder".into(),
                },
                register_interface::ResultField {
                    name: "member".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedFolderIamMemberResult {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            managed_folder: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedFolder").unwrap(),
            ),
            member: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("member").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
