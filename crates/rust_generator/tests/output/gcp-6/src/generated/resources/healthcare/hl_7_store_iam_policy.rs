/// Three different resources help you manage your IAM policy for Healthcare HL7v2 store. Each of these resources serves a different use case:
///
/// * `gcp.healthcare.Hl7StoreIamPolicy`: Authoritative. Sets the IAM policy for the HL7v2 store and replaces any existing policy already attached.
/// * `gcp.healthcare.Hl7StoreIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the HL7v2 store are preserved.
/// * `gcp.healthcare.Hl7StoreIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the HL7v2 store are preserved.
///
/// > **Note:** `gcp.healthcare.Hl7StoreIamPolicy` **cannot** be used in conjunction with `gcp.healthcare.Hl7StoreIamBinding` and `gcp.healthcare.Hl7StoreIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.healthcare.Hl7StoreIamBinding` resources **can be** used in conjunction with `gcp.healthcare.Hl7StoreIamMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.healthcare.Hl7StoreIamPolicy
///
/// ```yaml
/// resources:
///   hl7V2Store:
///     type: gcp:healthcare:Hl7StoreIamPolicy
///     name: hl7_v2_store
///     properties:
///       hl7V2StoreId: your-hl7-v2-store-id
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
/// ## gcp.healthcare.Hl7StoreIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let hl7V2Store = hl_7_store_iam_binding::create(
///         "hl7V2Store",
///         Hl7StoreIamBindingArgs::builder()
///             .hl_7_v_2_store_id("your-hl7-v2-store-id")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.Hl7StoreIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let hl7V2Store = hl_7_store_iam_member::create(
///         "hl7V2Store",
///         Hl7StoreIamMemberArgs::builder()
///             .hl_7_v_2_store_id("your-hl7-v2-store-id")
///             .member("user:jane@example.com")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.Hl7StoreIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let hl7V2Store = hl_7_store_iam_binding::create(
///         "hl7V2Store",
///         Hl7StoreIamBindingArgs::builder()
///             .hl_7_v_2_store_id("your-hl7-v2-store-id")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.Hl7StoreIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let hl7V2Store = hl_7_store_iam_member::create(
///         "hl7V2Store",
///         Hl7StoreIamMemberArgs::builder()
///             .hl_7_v_2_store_id("your-hl7-v2-store-id")
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
/// IAM policy imports use the identifier of the Google Cloud Healthcare HL7v2 store resource. For example:
///
/// * `"{{project_id}}/{{location}}/{{dataset}}/{{hl7_v2_store}}"`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = "{{project_id}}/{{location}}/{{dataset}}/{{hl7_v2_store}}"
///
///   to = google_healthcare_hl7_v2_store_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:healthcare/hl7StoreIamPolicy:Hl7StoreIamPolicy default {{project_id}}/{{location}}/{{dataset}}/{{hl7_v2_store}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hl_7_store_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Hl7StoreIamPolicyArgs {
        /// The HL7v2 store ID, in the form
        /// `{project_id}/{location_name}/{dataset_name}/{hl7_v2_store_name}` or
        /// `{location_name}/{dataset_name}/{hl7_v2_store_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        #[builder(into)]
        pub hl7_v2_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct Hl7StoreIamPolicyResult {
        /// (Computed) The etag of the HL7v2 store's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The HL7v2 store ID, in the form
        /// `{project_id}/{location_name}/{dataset_name}/{hl7_v2_store_name}` or
        /// `{location_name}/{dataset_name}/{hl7_v2_store_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        pub hl7_v2_store_id: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: Hl7StoreIamPolicyArgs,
    ) -> Hl7StoreIamPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let hl7_v2_store_id_binding_1 = args.hl7_v2_store_id.get_output(context);
        let hl7_v2_store_id_binding = hl7_v2_store_id_binding_1.get_inner();
        let policy_data_binding_1 = args.policy_data.get_output(context);
        let policy_data_binding = policy_data_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:healthcare/hl7StoreIamPolicy:Hl7StoreIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hl7V2StoreId".into(),
                    value: &hl7_v2_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        Hl7StoreIamPolicyResult {
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            hl7_v2_store_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hl7V2StoreId"),
            ),
            policy_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
        }
    }
}
