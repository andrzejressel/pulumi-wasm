/// Three different resources help you manage your IAM policy for KMS crypto key. Each of these resources serves a different use case:
///
/// * `gcp.kms.CryptoKeyIAMPolicy`: Authoritative. Sets the IAM policy for the crypto key and replaces any existing policy already attached.
/// * `gcp.kms.CryptoKeyIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the crypto key are preserved.
/// * `gcp.kms.CryptoKeyIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the crypto key are preserved.
///
/// > **Note:** `gcp.kms.CryptoKeyIAMPolicy` **cannot** be used in conjunction with `gcp.kms.CryptoKeyIAMBinding` and `gcp.kms.CryptoKeyIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.kms.CryptoKeyIAMBinding` resources **can be** used in conjunction with `gcp.kms.CryptoKeyIAMMember` resources **only if** they do not grant privilege to the same role.
///
/// ```yaml
/// resources:
///   keyring:
///     type: gcp:kms:KeyRing
///     properties:
///       name: keyring-example
///       location: global
///   key:
///     type: gcp:kms:CryptoKey
///     properties:
///       name: crypto-key-example
///       keyRing: ${keyring.id}
///       rotationPeriod: 7776000s
///   cryptoKey:
///     type: gcp:kms:CryptoKeyIAMPolicy
///     name: crypto_key
///     properties:
///       cryptoKeyId: ${key.id}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/cloudkms.cryptoKeyEncrypter
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/cloudkms.cryptoKeyEncrypter
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cryptoKey = crypto_key_iam_binding::create(
///         "cryptoKey",
///         CryptoKeyIamBindingArgs::builder()
///             .crypto_key_id("${key.id}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/cloudkms.cryptoKeyEncrypter")
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
///     let cryptoKey = crypto_key_iam_binding::create(
///         "cryptoKey",
///         CryptoKeyIamBindingArgs::builder()
///             .condition(
///                 CryptoKeyIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .crypto_key_id("${key.id}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/cloudkms.cryptoKeyEncrypter")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cryptoKey = crypto_key_iam_member::create(
///         "cryptoKey",
///         CryptoKeyIamMemberArgs::builder()
///             .crypto_key_id("${key.id}")
///             .member("user:jane@example.com")
///             .role("roles/cloudkms.cryptoKeyEncrypter")
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
///     let cryptoKey = crypto_key_iam_member::create(
///         "cryptoKey",
///         CryptoKeyIamMemberArgs::builder()
///             .condition(
///                 CryptoKeyIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .crypto_key_id("${key.id}")
///             .member("user:jane@example.com")
///             .role("roles/cloudkms.cryptoKeyEncrypter")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing IAM policies
///
/// IAM policy imports use the identifier of the KMS crypto key only. For example:
///
/// * `{{project_id}}/{{location}}/{{key_ring_name}}/{{crypto_key_name}}`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = "{{project_id}}/{{location}}/{{key_ring_name}}/{{crypto_key_name}}"
///
///   to = google_kms_crypto_key_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:kms/cryptoKeyIAMPolicy:CryptoKeyIAMPolicy default {{project_id}}/{{location}}/{{key_ring_name}}/{{crypto_key_name}}
/// ```
///
pub mod crypto_key_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CryptoKeyIAMPolicyArgs {
        /// The crypto key ID, in the form
        /// `{project_id}/{location_name}/{key_ring_name}/{crypto_key_name}` or
        /// `{location_name}/{key_ring_name}/{crypto_key_name}`. In the second form,
        /// the provider's project setting will be used as a fallback.
        #[builder(into)]
        pub crypto_key_id: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CryptoKeyIAMPolicyResult {
        /// The crypto key ID, in the form
        /// `{project_id}/{location_name}/{key_ring_name}/{crypto_key_name}` or
        /// `{location_name}/{key_ring_name}/{crypto_key_name}`. In the second form,
        /// the provider's project setting will be used as a fallback.
        pub crypto_key_id: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the project's IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CryptoKeyIAMPolicyArgs) -> CryptoKeyIAMPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let crypto_key_id_binding = args.crypto_key_id.get_inner();
        let policy_data_binding = args.policy_data.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:kms/cryptoKeyIAMPolicy:CryptoKeyIAMPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cryptoKeyId".into(),
                    value: &crypto_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cryptoKeyId".into(),
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
        CryptoKeyIAMPolicyResult {
            crypto_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cryptoKeyId").unwrap(),
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
