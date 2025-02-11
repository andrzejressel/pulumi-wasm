/// Three different resources help you manage IAM policies on bigtable instances. Each of these resources serves a different use case:
///
/// * `gcp.bigtable.InstanceIamPolicy`: Authoritative. Sets the IAM policy for the instance and replaces any existing policy already attached.
/// * `gcp.bigtable.InstanceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the instance are preserved.
/// * `gcp.bigtable.InstanceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the instance are preserved.
///
/// > **Note:** `gcp.bigtable.InstanceIamPolicy` **cannot** be used in conjunction with `gcp.bigtable.InstanceIamBinding` and `gcp.bigtable.InstanceIamMember` or they will fight over what your policy should be. In addition, be careful not to accidentally unset ownership of the instance as `gcp.bigtable.InstanceIamPolicy` replaces the entire policy.
///
/// > **Note:** `gcp.bigtable.InstanceIamBinding` resources **can be** used in conjunction with `gcp.bigtable.InstanceIamMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.bigtable.InstanceIamPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:bigtable:InstanceIamPolicy
///     properties:
///       project: your-project
///       instance: your-bigtable-instance
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/bigtable.user
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.bigtable.InstanceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = instance_iam_binding::create(
///         "editor",
///         InstanceIamBindingArgs::builder()
///             .instance("your-bigtable-instance")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/bigtable.user")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigtable.InstanceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = instance_iam_member::create(
///         "editor",
///         InstanceIamMemberArgs::builder()
///             .instance("your-bigtable-instance")
///             .member("user:jane@example.com")
///             .role("roles/bigtable.user")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigtable.InstanceIamPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:bigtable:InstanceIamPolicy
///     properties:
///       project: your-project
///       instance: your-bigtable-instance
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/bigtable.user
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.bigtable.InstanceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = instance_iam_binding::create(
///         "editor",
///         InstanceIamBindingArgs::builder()
///             .instance("your-bigtable-instance")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/bigtable.user")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigtable.InstanceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = instance_iam_member::create(
///         "editor",
///         InstanceIamMemberArgs::builder()
///             .instance("your-bigtable-instance")
///             .member("user:jane@example.com")
///             .role("roles/bigtable.user")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing IAM policies
///
/// IAM policy imports use the `instance` identifier of the Bigtable Instance resource only. For example:
///
/// * `"projects/{project}/instances/{instance}"`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = "projects/{project}/instances/{instance}"
///
///   to = google_bigtable_instance_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:bigtable/instanceIamPolicy:InstanceIamPolicy default projects/{project}/instances/{instance}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceIamPolicyArgs {
        /// The name or relative resource id of the instance to manage IAM policies for.
        ///
        /// For `gcp.bigtable.InstanceIamMember` or `gcp.bigtable.InstanceIamBinding`:
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceIamPolicyResult {
        /// (Computed) The etag of the instances's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The name or relative resource id of the instance to manage IAM policies for.
        ///
        /// For `gcp.bigtable.InstanceIamMember` or `gcp.bigtable.InstanceIamBinding`:
        pub instance: pulumi_gestalt_rust::Output<String>,
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceIamPolicyArgs,
    ) -> InstanceIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_binding = args.instance.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigtable/instanceIamPolicy:InstanceIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: &instance_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceIamPolicyResult {
            etag: o.get_field("etag"),
            instance: o.get_field("instance"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
        }
    }
}
