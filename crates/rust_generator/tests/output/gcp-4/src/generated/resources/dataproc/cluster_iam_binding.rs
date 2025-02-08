/// Three different resources help you manage IAM policies on dataproc clusters. Each of these resources serves a different use case:
///
/// * `gcp.dataproc.ClusterIAMPolicy`: Authoritative. Sets the IAM policy for the cluster and replaces any existing policy already attached.
/// * `gcp.dataproc.ClusterIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the cluster are preserved.
/// * `gcp.dataproc.ClusterIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the cluster are preserved.
///
/// > **Note:** `gcp.dataproc.ClusterIAMPolicy` **cannot** be used in conjunction with `gcp.dataproc.ClusterIAMBinding` and `gcp.dataproc.ClusterIAMMember` or they will fight over what your policy should be. In addition, be careful not to accidentally unset ownership of the cluster as `gcp.dataproc.ClusterIAMPolicy` replaces the entire policy.
///
/// > **Note:** `gcp.dataproc.ClusterIAMBinding` resources **can be** used in conjunction with `gcp.dataproc.ClusterIAMMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.dataproc.ClusterIAMPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:dataproc:ClusterIAMPolicy
///     properties:
///       project: your-project
///       region: your-region
///       cluster: your-dataproc-cluster
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
/// ## gcp.dataproc.ClusterIAMBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = cluster_iam_binding::create(
///         "editor",
///         ClusterIamBindingArgs::builder()
///             .cluster("your-dataproc-cluster")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataproc.ClusterIAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = cluster_iam_member::create(
///         "editor",
///         ClusterIamMemberArgs::builder()
///             .cluster("your-dataproc-cluster")
///             .member("user:jane@example.com")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataproc.ClusterIAMPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:dataproc:ClusterIAMPolicy
///     properties:
///       project: your-project
///       region: your-region
///       cluster: your-dataproc-cluster
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
/// ## gcp.dataproc.ClusterIAMBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = cluster_iam_binding::create(
///         "editor",
///         ClusterIamBindingArgs::builder()
///             .cluster("your-dataproc-cluster")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataproc.ClusterIAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = cluster_iam_member::create(
///         "editor",
///         ClusterIamMemberArgs::builder()
///             .cluster("your-dataproc-cluster")
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
/// IAM policy imports use the `cluster` identifier of the Dataproc Cluster resource only. For example:
///
/// * `projects/{project}/regions/{region}/clusters/{cluster}`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = projects/{project}/regions/{region}/clusters/{cluster}
///
///   to = google_dataproc_cluster_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:dataproc/clusterIAMBinding:ClusterIAMBinding default projects/{project}/regions/{region}/clusters/{cluster}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterIAMBindingArgs {
        /// The name or relative resource id of the cluster to manage IAM policies for.
        ///
        /// For `gcp.dataproc.ClusterIAMMember` or `gcp.dataproc.ClusterIAMBinding`:
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::ClusterIamBindingCondition>,
        >,
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
        /// The project in which the cluster belongs. If it
        /// is not provided, the provider will use a default.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region in which the cluster belongs. If it
        /// is not provided, the provider will use a default.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.dataproc.ClusterIAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        ///
        /// `gcp.dataproc.ClusterIAMPolicy` only:
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterIAMBindingResult {
        /// The name or relative resource id of the cluster to manage IAM policies for.
        ///
        /// For `gcp.dataproc.ClusterIAMMember` or `gcp.dataproc.ClusterIAMBinding`:
        pub cluster: pulumi_gestalt_rust::Output<String>,
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::ClusterIamBindingCondition>,
        >,
        /// (Computed) The etag of the clusters's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        pub members: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The project in which the cluster belongs. If it
        /// is not provided, the provider will use a default.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The region in which the cluster belongs. If it
        /// is not provided, the provider will use a default.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.dataproc.ClusterIAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        ///
        /// `gcp.dataproc.ClusterIAMPolicy` only:
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClusterIAMBindingArgs,
    ) -> ClusterIAMBindingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_binding = args.cluster.get_output(context).get_inner();
        let condition_binding = args.condition.get_output(context).get_inner();
        let members_binding = args.members.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataproc/clusterIAMBinding:ClusterIAMBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding,
                },
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "members".into(),
                    value: &members_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterIAMBindingResult {
            cluster: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cluster"),
            ),
            condition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            members: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("members"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            role: pulumi_gestalt_rust::__private::into_domain(o.extract_field("role")),
        }
    }
}
