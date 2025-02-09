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
/// $ pulumi import gcp:healthcare/hl7StoreIamBinding:Hl7StoreIamBinding default {{project_id}}/{{location}}/{{dataset}}/{{hl7_v2_store}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hl_7_store_iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Hl7StoreIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::healthcare::Hl7StoreIamBindingCondition>,
        >,
        /// The HL7v2 store ID, in the form
        /// `{project_id}/{location_name}/{dataset_name}/{hl7_v2_store_name}` or
        /// `{location_name}/{dataset_name}/{hl7_v2_store_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        #[builder(into)]
        pub hl7_v2_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        #[builder(into)]
        pub members: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The role that should be applied. Only one
        /// `gcp.healthcare.Hl7StoreIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct Hl7StoreIamBindingResult {
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::healthcare::Hl7StoreIamBindingCondition>,
        >,
        /// (Computed) The etag of the HL7v2 store's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The HL7v2 store ID, in the form
        /// `{project_id}/{location_name}/{dataset_name}/{hl7_v2_store_name}` or
        /// `{location_name}/{dataset_name}/{hl7_v2_store_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        pub hl7_v2_store_id: pulumi_gestalt_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        pub members: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The role that should be applied. Only one
        /// `gcp.healthcare.Hl7StoreIamBinding` can be used per role. Note that custom roles must be of the format
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
        args: Hl7StoreIamBindingArgs,
    ) -> Hl7StoreIamBindingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let hl7_v2_store_id_binding = args.hl7_v2_store_id.get_output(context);
        let members_binding = args.members.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:healthcare/hl7StoreIamBinding:Hl7StoreIamBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: condition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hl7V2StoreId".into(),
                    value: hl7_v2_store_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "members".into(),
                    value: members_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        Hl7StoreIamBindingResult {
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            hl7_v2_store_id: o.get_field("hl7V2StoreId"),
            members: o.get_field("members"),
            role: o.get_field("role"),
        }
    }
}
