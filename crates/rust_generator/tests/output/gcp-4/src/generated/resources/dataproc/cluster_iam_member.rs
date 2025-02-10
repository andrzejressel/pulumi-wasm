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
/// $ pulumi import gcp:dataproc/clusterIAMMember:ClusterIAMMember default projects/{project}/regions/{region}/clusters/{cluster}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_iam_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterIAMMemberArgs {
        /// The name or relative resource id of the cluster to manage IAM policies for.
        ///
        /// For `gcp.dataproc.ClusterIAMMember` or `gcp.dataproc.ClusterIAMBinding`:
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::ClusterIamMemberCondition>,
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
        pub member: pulumi_gestalt_rust::InputOrOutput<String>,
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
    pub struct ClusterIAMMemberResult {
        /// The name or relative resource id of the cluster to manage IAM policies for.
        ///
        /// For `gcp.dataproc.ClusterIAMMember` or `gcp.dataproc.ClusterIAMBinding`:
        pub cluster: pulumi_gestalt_rust::Output<String>,
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::ClusterIamMemberCondition>,
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
        pub member: pulumi_gestalt_rust::Output<String>,
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterIAMMemberArgs,
    ) -> ClusterIAMMemberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_binding = args.cluster.get_output(context);
        let condition_binding = args.condition.get_output(context);
        let member_binding = args.member.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataproc/clusterIAMMember:ClusterIAMMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cluster".into(),
                    value: cluster_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: condition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "member".into(),
                    value: member_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterIAMMemberResult {
            cluster: o.get_field("cluster"),
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            member: o.get_field("member"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            role: o.get_field("role"),
        }
    }
}
