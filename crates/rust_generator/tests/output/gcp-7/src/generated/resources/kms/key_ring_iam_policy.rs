/// Three different resources help you manage your IAM policy for KMS key ring. Each of these resources serves a different use case:
///
/// * `gcp.kms.KeyRingIAMPolicy`: Authoritative. Sets the IAM policy for the key ring and replaces any existing policy already attached.
/// * `gcp.kms.KeyRingIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the key ring are preserved.
/// * `gcp.kms.KeyRingIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the key ring are preserved.
///
/// > **Note:** `gcp.kms.KeyRingIAMPolicy` **cannot** be used in conjunction with `gcp.kms.KeyRingIAMBinding` and `gcp.kms.KeyRingIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.kms.KeyRingIAMBinding` resources **can be** used in conjunction with `gcp.kms.KeyRingIAMMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.kms.KeyRingIAMPolicy
///
/// ```yaml
/// resources:
///   keyring:
///     type: gcp:kms:KeyRing
///     properties:
///       name: keyring-example
///       location: global
///   keyRing:
///     type: gcp:kms:KeyRingIAMPolicy
///     name: key_ring
///     properties:
///       keyRingId: ${keyring.id}
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
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   keyring:
///     type: gcp:kms:KeyRing
///     properties:
///       name: keyring-example
///       location: global
///   keyRing:
///     type: gcp:kms:KeyRingIAMPolicy
///     name: key_ring
///     properties:
///       keyRingId: ${keyring.id}
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
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
/// ## gcp.kms.KeyRingIAMBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let keyRing = key_ring_iam_binding::create(
///         "keyRing",
///         KeyRingIamBindingArgs::builder()
///             .key_ring_id("your-key-ring-id")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/cloudkms.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let keyRing = key_ring_iam_binding::create(
///         "keyRing",
///         KeyRingIamBindingArgs::builder()
///             .condition(
///                 KeyRingIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .key_ring_id("your-key-ring-id")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/cloudkms.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.kms.KeyRingIAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let keyRing = key_ring_iam_member::create(
///         "keyRing",
///         KeyRingIamMemberArgs::builder()
///             .key_ring_id("your-key-ring-id")
///             .member("user:jane@example.com")
///             .role("roles/cloudkms.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let keyRing = key_ring_iam_member::create(
///         "keyRing",
///         KeyRingIamMemberArgs::builder()
///             .condition(
///                 KeyRingIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .key_ring_id("your-key-ring-id")
///             .member("user:jane@example.com")
///             .role("roles/cloudkms.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.kms.KeyRingIAMBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let keyRing = key_ring_iam_binding::create(
///         "keyRing",
///         KeyRingIamBindingArgs::builder()
///             .key_ring_id("your-key-ring-id")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/cloudkms.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let keyRing = key_ring_iam_binding::create(
///         "keyRing",
///         KeyRingIamBindingArgs::builder()
///             .condition(
///                 KeyRingIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .key_ring_id("your-key-ring-id")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/cloudkms.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.kms.KeyRingIAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let keyRing = key_ring_iam_member::create(
///         "keyRing",
///         KeyRingIamMemberArgs::builder()
///             .key_ring_id("your-key-ring-id")
///             .member("user:jane@example.com")
///             .role("roles/cloudkms.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let keyRing = key_ring_iam_member::create(
///         "keyRing",
///         KeyRingIamMemberArgs::builder()
///             .condition(
///                 KeyRingIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .key_ring_id("your-key-ring-id")
///             .member("user:jane@example.com")
///             .role("roles/cloudkms.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing IAM policies
///
/// IAM policy imports use the identifier of the Cloud KMS key ring only. For example:
///
/// * `{{project_id}}/{{location}}/{{key_ring_name}}`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = "{{project_id}}/{{location}}/{{key_ring_name}}"
///
///   to = google_kms_key_ring_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:kms/keyRingIAMPolicy:KeyRingIAMPolicy default {{project_id}}/{{location}}/{{key_ring_name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod key_ring_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyRingIAMPolicyArgs {
        /// The key ring ID, in the form
        /// `{project_id}/{location_name}/{key_ring_name}` or
        /// `{location_name}/{key_ring_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        #[builder(into)]
        pub key_ring_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct KeyRingIAMPolicyResult {
        /// (Computed) The etag of the key ring's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The key ring ID, in the form
        /// `{project_id}/{location_name}/{key_ring_name}` or
        /// `{location_name}/{key_ring_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        pub key_ring_id: pulumi_gestalt_rust::Output<String>,
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
        args: KeyRingIAMPolicyArgs,
    ) -> KeyRingIAMPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_ring_id_binding = args.key_ring_id.get_output(context).get_inner();
        let policy_data_binding = args.policy_data.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:kms/keyRingIAMPolicy:KeyRingIAMPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyRingId".into(),
                    value: &key_ring_id_binding,
                },
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        KeyRingIAMPolicyResult {
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            key_ring_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyRingId"),
            ),
            policy_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
        }
    }
}
