/// Three different resources help you manage your IAM policy for Healthcare DICOM store. Each of these resources serves a different use case:
///
/// * `gcp.healthcare.DicomStoreIamPolicy`: Authoritative. Sets the IAM policy for the DICOM store and replaces any existing policy already attached.
/// * `gcp.healthcare.DicomStoreIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the DICOM store are preserved.
/// * `gcp.healthcare.DicomStoreIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the DICOM store are preserved.
///
/// > **Note:** `gcp.healthcare.DicomStoreIamPolicy` **cannot** be used in conjunction with `gcp.healthcare.DicomStoreIamBinding` and `gcp.healthcare.DicomStoreIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.healthcare.DicomStoreIamBinding` resources **can be** used in conjunction with `gcp.healthcare.DicomStoreIamMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.healthcare.DicomStoreIamPolicy
///
/// ```yaml
/// resources:
///   dicomStore:
///     type: gcp:healthcare:DicomStoreIamPolicy
///     name: dicom_store
///     properties:
///       dicomStoreId: your-dicom-store-id
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
/// ## gcp.healthcare.DicomStoreIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dicomStore = dicom_store_iam_binding::create(
///         "dicomStore",
///         DicomStoreIamBindingArgs::builder()
///             .dicom_store_id("your-dicom-store-id")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.DicomStoreIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dicomStore = dicom_store_iam_member::create(
///         "dicomStore",
///         DicomStoreIamMemberArgs::builder()
///             .dicom_store_id("your-dicom-store-id")
///             .member("user:jane@example.com")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.DicomStoreIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dicomStore = dicom_store_iam_binding::create(
///         "dicomStore",
///         DicomStoreIamBindingArgs::builder()
///             .dicom_store_id("your-dicom-store-id")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.DicomStoreIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dicomStore = dicom_store_iam_member::create(
///         "dicomStore",
///         DicomStoreIamMemberArgs::builder()
///             .dicom_store_id("your-dicom-store-id")
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
/// IAM policy imports use the identifier of the Healthcare DICOM store resource. For example:
///
/// * `"{{project_id}}/{{location}}/{{dataset}}/{{dicom_store}}"`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = "{{project_id}}/{{location}}/{{dataset}}/{{dicom_store}}"
///
///   to = google_healthcare_dicom_store_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:healthcare/dicomStoreIamPolicy:DicomStoreIamPolicy default {{project_id}}/{{location}}/{{dataset}}/{{dicom_store}}
/// ```
///
pub mod dicom_store_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DicomStoreIamPolicyArgs {
        /// The DICOM store ID, in the form
        /// `{project_id}/{location_name}/{dataset_name}/{dicom_store_name}` or
        /// `{location_name}/{dataset_name}/{dicom_store_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        #[builder(into)]
        pub dicom_store_id: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DicomStoreIamPolicyResult {
        /// The DICOM store ID, in the form
        /// `{project_id}/{location_name}/{dataset_name}/{dicom_store_name}` or
        /// `{location_name}/{dataset_name}/{dicom_store_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        pub dicom_store_id: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the DICOM store's IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DicomStoreIamPolicyArgs,
    ) -> DicomStoreIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dicom_store_id_binding = args.dicom_store_id.get_inner();
        let policy_data_binding = args.policy_data.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:healthcare/dicomStoreIamPolicy:DicomStoreIamPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dicomStoreId".into(),
                    value: &dicom_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dicomStoreId".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "policyData".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DicomStoreIamPolicyResult {
            dicom_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dicomStoreId").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyData").unwrap(),
            ),
        }
    }
}
