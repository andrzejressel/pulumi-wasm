/// Three different resources help you manage your IAM policy for Cloud Healthcare ConsentStore. Each of these resources serves a different use case:
///
/// * `gcp.healthcare.ConsentStoreIamPolicy`: Authoritative. Sets the IAM policy for the consentstore and replaces any existing policy already attached.
/// * `gcp.healthcare.ConsentStoreIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the consentstore are preserved.
/// * `gcp.healthcare.ConsentStoreIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the consentstore are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.healthcare.ConsentStoreIamPolicy`: Retrieves the IAM policy for the consentstore
///
/// > **Note:** `gcp.healthcare.ConsentStoreIamPolicy` **cannot** be used in conjunction with `gcp.healthcare.ConsentStoreIamBinding` and `gcp.healthcare.ConsentStoreIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.healthcare.ConsentStoreIamBinding` resources **can be** used in conjunction with `gcp.healthcare.ConsentStoreIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.healthcare.ConsentStoreIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:healthcare:ConsentStoreIamPolicy
///     properties:
///       dataset: ${["my-consent"].dataset}
///       consentStoreId: ${["my-consent"].name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.healthcare.ConsentStoreIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = consent_store_iam_binding::create(
///         "binding",
///         ConsentStoreIamBindingArgs::builder()
///             .consent_store_id("${[\"my-consent\"].name}")
///             .dataset("${[\"my-consent\"].dataset}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.ConsentStoreIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = consent_store_iam_member::create(
///         "member",
///         ConsentStoreIamMemberArgs::builder()
///             .consent_store_id("${[\"my-consent\"].name}")
///             .dataset("${[\"my-consent\"].dataset}")
///             .member("user:jane@example.com")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ## > **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
/// -
///
/// # IAM policy for Cloud Healthcare ConsentStore
/// Three different resources help you manage your IAM policy for Cloud Healthcare ConsentStore. Each of these resources serves a different use case:
///
/// * `gcp.healthcare.ConsentStoreIamPolicy`: Authoritative. Sets the IAM policy for the consentstore and replaces any existing policy already attached.
/// * `gcp.healthcare.ConsentStoreIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the consentstore are preserved.
/// * `gcp.healthcare.ConsentStoreIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the consentstore are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.healthcare.ConsentStoreIamPolicy`: Retrieves the IAM policy for the consentstore
///
/// > **Note:** `gcp.healthcare.ConsentStoreIamPolicy` **cannot** be used in conjunction with `gcp.healthcare.ConsentStoreIamBinding` and `gcp.healthcare.ConsentStoreIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.healthcare.ConsentStoreIamBinding` resources **can be** used in conjunction with `gcp.healthcare.ConsentStoreIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.healthcare.ConsentStoreIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:healthcare:ConsentStoreIamPolicy
///     properties:
///       dataset: ${["my-consent"].dataset}
///       consentStoreId: ${["my-consent"].name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.healthcare.ConsentStoreIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = consent_store_iam_binding::create(
///         "binding",
///         ConsentStoreIamBindingArgs::builder()
///             .consent_store_id("${[\"my-consent\"].name}")
///             .dataset("${[\"my-consent\"].dataset}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.ConsentStoreIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = consent_store_iam_member::create(
///         "member",
///         ConsentStoreIamMemberArgs::builder()
///             .consent_store_id("${[\"my-consent\"].name}")
///             .dataset("${[\"my-consent\"].dataset}")
///             .member("user:jane@example.com")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * {{dataset}}/consentStores/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud Healthcare consentstore IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:healthcare/consentStoreIamBinding:ConsentStoreIamBinding editor "{{dataset}}/consentStores/{{consent_store}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:healthcare/consentStoreIamBinding:ConsentStoreIamBinding editor "{{dataset}}/consentStores/{{consent_store}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:healthcare/consentStoreIamBinding:ConsentStoreIamBinding editor {{dataset}}/consentStores/{{consent_store}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod consent_store_iam_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConsentStoreIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::ConsentStoreIamBindingCondition>,
        >,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub consent_store_id: pulumi_wasm_rust::Output<String>,
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub dataset: pulumi_wasm_rust::Output<String>,
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
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The role that should be applied. Only one
        /// `gcp.healthcare.ConsentStoreIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ConsentStoreIamBindingResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::ConsentStoreIamBindingCondition>,
        >,
        /// Used to find the parent resource to bind the IAM policy to
        pub consent_store_id: pulumi_wasm_rust::Output<String>,
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        /// Used to find the parent resource to bind the IAM policy to
        pub dataset: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
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
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The role that should be applied. Only one
        /// `gcp.healthcare.ConsentStoreIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ConsentStoreIamBindingArgs,
    ) -> ConsentStoreIamBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let consent_store_id_binding = args.consent_store_id.get_inner();
        let dataset_binding = args.dataset.get_inner();
        let members_binding = args.members.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:healthcare/consentStoreIamBinding:ConsentStoreIamBinding".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "consentStoreId".into(),
                    value: &consent_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "dataset".into(),
                    value: &dataset_binding,
                },
                register_interface::ObjectField {
                    name: "members".into(),
                    value: &members_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "consentStoreId".into(),
                },
                register_interface::ResultField {
                    name: "dataset".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "members".into(),
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
        ConsentStoreIamBindingResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            consent_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consentStoreId").unwrap(),
            ),
            dataset: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataset").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("members").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
