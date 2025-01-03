/// Three different resources help you manage IAM policies on dataproc jobs. Each of these resources serves a different use case:
///
/// * `gcp.dataproc.JobIAMPolicy`: Authoritative. Sets the IAM policy for the job and replaces any existing policy already attached.
/// * `gcp.dataproc.JobIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the job are preserved.
/// * `gcp.dataproc.JobIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the job are preserved.
///
/// > **Note:** `gcp.dataproc.JobIAMPolicy` **cannot** be used in conjunction with `gcp.dataproc.JobIAMBinding` and `gcp.dataproc.JobIAMMember` or they will fight over what your policy should be. In addition, be careful not to accidentally unset ownership of the job as `gcp.dataproc.JobIAMPolicy` replaces the entire policy.
///
/// > **Note:** `gcp.dataproc.JobIAMBinding` resources **can be** used in conjunction with `gcp.dataproc.JobIAMMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.dataproc.JobIAMPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:dataproc:JobIAMPolicy
///     properties:
///       project: your-project
///       region: your-region
///       jobId: your-dataproc-job
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
/// ## gcp.dataproc.JobIAMBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = job_iam_binding::create(
///         "editor",
///         JobIamBindingArgs::builder()
///             .job_id("your-dataproc-job")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataproc.JobIAMMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = job_iam_member::create(
///         "editor",
///         JobIamMemberArgs::builder()
///             .job_id("your-dataproc-job")
///             .member("user:jane@example.com")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataproc.JobIAMPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:dataproc:JobIAMPolicy
///     properties:
///       project: your-project
///       region: your-region
///       jobId: your-dataproc-job
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
/// ## gcp.dataproc.JobIAMBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = job_iam_binding::create(
///         "editor",
///         JobIamBindingArgs::builder()
///             .job_id("your-dataproc-job")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataproc.JobIAMMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = job_iam_member::create(
///         "editor",
///         JobIamMemberArgs::builder()
///             .job_id("your-dataproc-job")
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
/// IAM policy imports use the `job_id` identifier of the Dataproc Job resource only. For example:
///
/// * `projects/{project}/regions/{region}/jobs/{job_id}`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = "projects/{project}/regions/{region}/jobs/{job_id}"
///
///   to = google_dataproc_job_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:dataproc/jobIAMPolicy:JobIAMPolicy default "projects/{project}/regions/{region}/jobs/{job_id}"
/// ```
///
pub mod job_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobIAMPolicyArgs {
        #[builder(into)]
        pub job_id: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by a `gcp.organizations.getIAMPolicy` data source.
        ///
        /// - - -
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The project in which the job belongs. If it
        /// is not provided, the provider will use a default.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region in which the job belongs. If it
        /// is not provided, the provider will use a default.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct JobIAMPolicyResult {
        /// (Computed) The etag of the jobs's IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        pub job_id: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by a `gcp.organizations.getIAMPolicy` data source.
        ///
        /// - - -
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The project in which the job belongs. If it
        /// is not provided, the provider will use a default.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The region in which the job belongs. If it
        /// is not provided, the provider will use a default.
        pub region: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: JobIAMPolicyArgs) -> JobIAMPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let job_id_binding = args.job_id.get_inner();
        let policy_data_binding = args.policy_data.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataproc/jobIAMPolicy:JobIAMPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "jobId".into(),
                    value: &job_id_binding,
                },
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "jobId".into(),
                },
                register_interface::ResultField {
                    name: "policyData".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        JobIAMPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            job_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobId").unwrap(),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyData").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
        }
    }
}
