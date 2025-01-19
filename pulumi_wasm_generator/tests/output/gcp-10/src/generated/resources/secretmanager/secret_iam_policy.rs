/// Three different resources help you manage your IAM policy for Secret Manager Secret. Each of these resources serves a different use case:
///
/// * `gcp.secretmanager.SecretIamPolicy`: Authoritative. Sets the IAM policy for the secret and replaces any existing policy already attached.
/// * `gcp.secretmanager.SecretIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the secret are preserved.
/// * `gcp.secretmanager.SecretIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the secret are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.secretmanager.SecretIamPolicy`: Retrieves the IAM policy for the secret
///
/// > **Note:** `gcp.secretmanager.SecretIamPolicy` **cannot** be used in conjunction with `gcp.secretmanager.SecretIamBinding` and `gcp.secretmanager.SecretIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.secretmanager.SecretIamBinding` resources **can be** used in conjunction with `gcp.secretmanager.SecretIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.secretmanager.SecretIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:secretmanager:SecretIamPolicy
///     properties:
///       project: ${["secret-basic"].project}
///       secretId: ${["secret-basic"].secretId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:secretmanager:SecretIamPolicy
///     properties:
///       project: ${["secret-basic"].project}
///       secretId: ${["secret-basic"].secretId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.secretmanager.SecretIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = secret_iam_binding::create(
///         "binding",
///         SecretIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"secret-basic\"].secretId}")
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
///     let binding = secret_iam_binding::create(
///         "binding",
///         SecretIamBindingArgs::builder()
///             .condition(
///                 SecretIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"secret-basic\"].secretId}")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.secretmanager.SecretIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = secret_iam_member::create(
///         "member",
///         SecretIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${[\"secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"secret-basic\"].secretId}")
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
///     let member = secret_iam_member::create(
///         "member",
///         SecretIamMemberArgs::builder()
///             .condition(
///                 SecretIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .project("${[\"secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"secret-basic\"].secretId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Secret Manager Secret
/// Three different resources help you manage your IAM policy for Secret Manager Secret. Each of these resources serves a different use case:
///
/// * `gcp.secretmanager.SecretIamPolicy`: Authoritative. Sets the IAM policy for the secret and replaces any existing policy already attached.
/// * `gcp.secretmanager.SecretIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the secret are preserved.
/// * `gcp.secretmanager.SecretIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the secret are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.secretmanager.SecretIamPolicy`: Retrieves the IAM policy for the secret
///
/// > **Note:** `gcp.secretmanager.SecretIamPolicy` **cannot** be used in conjunction with `gcp.secretmanager.SecretIamBinding` and `gcp.secretmanager.SecretIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.secretmanager.SecretIamBinding` resources **can be** used in conjunction with `gcp.secretmanager.SecretIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.secretmanager.SecretIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:secretmanager:SecretIamPolicy
///     properties:
///       project: ${["secret-basic"].project}
///       secretId: ${["secret-basic"].secretId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:secretmanager:SecretIamPolicy
///     properties:
///       project: ${["secret-basic"].project}
///       secretId: ${["secret-basic"].secretId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.secretmanager.SecretIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = secret_iam_binding::create(
///         "binding",
///         SecretIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"secret-basic\"].secretId}")
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
///     let binding = secret_iam_binding::create(
///         "binding",
///         SecretIamBindingArgs::builder()
///             .condition(
///                 SecretIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"secret-basic\"].secretId}")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.secretmanager.SecretIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = secret_iam_member::create(
///         "member",
///         SecretIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${[\"secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"secret-basic\"].secretId}")
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
///     let member = secret_iam_member::create(
///         "member",
///         SecretIamMemberArgs::builder()
///             .condition(
///                 SecretIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .project("${[\"secret-basic\"].project}")
///             .role("roles/secretmanager.secretAccessor")
///             .secret_id("${[\"secret-basic\"].secretId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/secrets/{{secret_id}}
///
/// * {{project}}/{{secret_id}}
///
/// * {{secret_id}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Secret Manager secret IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:secretmanager/secretIamPolicy:SecretIamPolicy editor "projects/{{project}}/secrets/{{secret_id}} roles/secretmanager.secretAccessor user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:secretmanager/secretIamPolicy:SecretIamPolicy editor "projects/{{project}}/secrets/{{secret_id}} roles/secretmanager.secretAccessor"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:secretmanager/secretIamPolicy:SecretIamPolicy editor projects/{{project}}/secrets/{{secret_id}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod secret_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretIamPolicyArgs {
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into)]
        pub secret_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SecretIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        pub secret_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SecretIamPolicyArgs) -> SecretIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_data_binding = args.policy_data.get_inner();
        let project_binding = args.project.get_inner();
        let secret_id_binding = args.secret_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:secretmanager/secretIamPolicy:SecretIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "secretId".into(),
                    value: &secret_id_binding,
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
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "secretId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SecretIamPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyData").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            secret_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretId").unwrap(),
            ),
        }
    }
}
