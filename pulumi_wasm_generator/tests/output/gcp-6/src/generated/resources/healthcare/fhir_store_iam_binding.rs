/// Three different resources help you manage your IAM policy for Healthcare FHIR store. Each of these resources serves a different use case:
///
/// * `gcp.healthcare.FhirStoreIamPolicy`: Authoritative. Sets the IAM policy for the FHIR store and replaces any existing policy already attached.
/// * `gcp.healthcare.FhirStoreIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the FHIR store are preserved.
/// * `gcp.healthcare.FhirStoreIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the FHIR store are preserved.
///
/// > **Note:** `gcp.healthcare.FhirStoreIamPolicy` **cannot** be used in conjunction with `gcp.healthcare.FhirStoreIamBinding` and `gcp.healthcare.FhirStoreIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.healthcare.FhirStoreIamBinding` resources **can be** used in conjunction with `gcp.healthcare.FhirStoreIamMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.healthcare.FhirStoreIamPolicy
///
/// ```yaml
/// resources:
///   fhirStore:
///     type: gcp:healthcare:FhirStoreIamPolicy
///     name: fhir_store
///     properties:
///       fhirStoreId: your-fhir-store-id
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/editor
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.healthcare.FhirStoreIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let fhirStore = fhir_store_iam_binding::create(
///         "fhirStore",
///         FhirStoreIamBindingArgs::builder()
///             .fhir_store_id("your-fhir-store-id")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.FhirStoreIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let fhirStore = fhir_store_iam_member::create(
///         "fhirStore",
///         FhirStoreIamMemberArgs::builder()
///             .fhir_store_id("your-fhir-store-id")
///             .member("user:jane@example.com")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.FhirStoreIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let fhirStore = fhir_store_iam_binding::create(
///         "fhirStore",
///         FhirStoreIamBindingArgs::builder()
///             .fhir_store_id("your-fhir-store-id")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.FhirStoreIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let fhirStore = fhir_store_iam_member::create(
///         "fhirStore",
///         FhirStoreIamMemberArgs::builder()
///             .fhir_store_id("your-fhir-store-id")
///             .member("user:jane@example.com")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing IAM policies
///
/// IAM policy imports use the identifier of the Healthcare FHIR store resource. For example:
///
/// * `"{{project_id}}/{{location}}/{{dataset}}/{{fhir_store}}"`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = "{{project_id}}/{{location}}/{{dataset}}/{{fhir_store}}"
///
///   to = google_healthcare_fhir_store_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:healthcare/fhirStoreIamBinding:FhirStoreIamBinding default {{project_id}}/{{location}}/{{dataset}}/{{fhir_store}}
/// ```
///
pub mod fhir_store_iam_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FhirStoreIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::healthcare::FhirStoreIamBindingCondition>,
        >,
        /// The FHIR store ID, in the form
        /// `{project_id}/{location_name}/{dataset_name}/{fhir_store_name}` or
        /// `{location_name}/{dataset_name}/{fhir_store_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        #[builder(into)]
        pub fhir_store_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        #[builder(into)]
        pub members: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The role that should be applied. Only one
        /// `gcp.healthcare.FhirStoreIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FhirStoreIamBindingResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::FhirStoreIamBindingCondition>,
        >,
        /// (Computed) The etag of the FHIR store's IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The FHIR store ID, in the form
        /// `{project_id}/{location_name}/{dataset_name}/{fhir_store_name}` or
        /// `{location_name}/{dataset_name}/{fhir_store_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        pub fhir_store_id: pulumi_wasm_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The role that should be applied. Only one
        /// `gcp.healthcare.FhirStoreIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FhirStoreIamBindingArgs,
    ) -> FhirStoreIamBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_output(context).get_inner();
        let fhir_store_id_binding = args.fhir_store_id.get_output(context).get_inner();
        let members_binding = args.members.get_output(context).get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:healthcare/fhirStoreIamBinding:FhirStoreIamBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "fhirStoreId".into(),
                    value: &fhir_store_id_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        FhirStoreIamBindingResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            fhir_store_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fhirStoreId"),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("members"),
            ),
            role: pulumi_wasm_rust::__private::into_domain(o.extract_field("role")),
        }
    }
}
