/// When managing IAM roles, you can treat a service account either as a resource or as an identity. This resource is to add iam policy bindings to a service account resource, such as allowing the members to run operations as or modify the service account. To configure permissions for a service account on other GCP resources, use the google_project_iam set of resources.
///
/// Three different resources help you manage your IAM policy for a service account. Each of these resources serves a different use case:
///
/// * `gcp.serviceaccount.IAMPolicy`: Authoritative. Sets the IAM policy for the service account and replaces any existing policy already attached.
/// * `gcp.serviceaccount.IAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the service account are preserved.
/// * `gcp.serviceaccount.IAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the service account are preserved.
///
/// > **Note:** `gcp.serviceaccount.IAMPolicy` **cannot** be used in conjunction with `gcp.serviceaccount.IAMBinding` and `gcp.serviceaccount.IAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.serviceaccount.IAMBinding` resources **can be** used in conjunction with `gcp.serviceaccount.IAMMember` resources **only if** they do not grant privilege to the same role.
///
/// ## Example Usage
///
/// ### Service Account IAM Policy
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that only Jane can interact with
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMPolicy
///     properties:
///       serviceAccountId: ${sa.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iam.serviceAccountUser
///             members:
///               - user:jane@example.com
/// ```
///
/// ### Service Account IAM Binding
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that only Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMBinding
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       members:
///         - user:jane@example.com
/// ```
///
/// ### Service Account IAM Binding With IAM Conditions:
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that only Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMBinding
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       members:
///         - user:jane@example.com
///       condition:
///         title: expires_after_2019_12_31
///         description: Expiring at midnight of 2019-12-31
///         expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
/// ### Service Account IAM Member
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMMember
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       member: user:jane@example.com
///   # Allow SA service account use the default GCE account
///   gce-default-account-iam:
///     type: gcp:serviceaccount:IAMMember
///     properties:
///       serviceAccountId: ${default.name}
///       role: roles/iam.serviceAccountUser
///       member: serviceAccount:${sa.email}
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:compute:getDefaultServiceAccount
///       arguments: {}
/// ```
///
/// ### Service Account IAM Member With IAM Conditions:
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMMember
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       member: user:jane@example.com
///       condition:
///         title: expires_after_2019_12_31
///         description: Expiring at midnight of 2019-12-31
///         expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
///
/// ### Additional Examples
///
/// ### Service Account IAM Policy
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that only Jane can interact with
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMPolicy
///     properties:
///       serviceAccountId: ${sa.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iam.serviceAccountUser
///             members:
///               - user:jane@example.com
/// ```
///
/// ### Service Account IAM Binding
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that only Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMBinding
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       members:
///         - user:jane@example.com
/// ```
///
/// ### Service Account IAM Binding With IAM Conditions:
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that only Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMBinding
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       members:
///         - user:jane@example.com
///       condition:
///         title: expires_after_2019_12_31
///         description: Expiring at midnight of 2019-12-31
///         expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
/// ### Service Account IAM Member
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMMember
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       member: user:jane@example.com
///   # Allow SA service account use the default GCE account
///   gce-default-account-iam:
///     type: gcp:serviceaccount:IAMMember
///     properties:
///       serviceAccountId: ${default.name}
///       role: roles/iam.serviceAccountUser
///       member: serviceAccount:${sa.email}
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:compute:getDefaultServiceAccount
///       arguments: {}
/// ```
///
/// ### Service Account IAM Member With IAM Conditions:
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-service-account
///       displayName: A service account that Jane can use
///   admin-account-iam:
///     type: gcp:serviceaccount:IAMMember
///     properties:
///       serviceAccountId: ${sa.name}
///       role: roles/iam.serviceAccountUser
///       member: user:jane@example.com
///       condition:
///         title: expires_after_2019_12_31
///         description: Expiring at midnight of 2019-12-31
///         expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
///
/// ## Import
///
/// ### Importing with conditions:
///
/// Here are examples of importing IAM memberships and bindings that include conditions:
///
/// ```sh
/// $ pulumi import gcp:serviceaccount/iAMPolicy:IAMPolicy admin-account-iam "projects/{your-project-id}/serviceAccounts/{your-service-account-email} roles/iam.serviceAccountUser expires_after_2019_12_31"
/// ```
///
/// ```sh
/// $ pulumi import gcp:serviceaccount/iAMPolicy:IAMPolicy admin-account-iam "projects/{your-project-id}/serviceAccounts/{your-service-account-email} roles/iam.serviceAccountUser user:foo@example.com expires_after_2019_12_31"
/// ```
///
pub mod iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IAMPolicyArgs {
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The fully-qualified name of the service account to apply policy to.
        #[builder(into)]
        pub service_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IAMPolicyResult {
        /// (Computed) The etag of the service account IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The fully-qualified name of the service account to apply policy to.
        pub service_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: IAMPolicyArgs) -> IAMPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_data_binding = args.policy_data.get_inner();
        let service_account_id_binding = args.service_account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:serviceaccount/iAMPolicy:IAMPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccountId".into(),
                    value: &service_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "policyData".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccountId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IAMPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyData").unwrap(),
            ),
            service_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccountId").unwrap(),
            ),
        }
    }
}
