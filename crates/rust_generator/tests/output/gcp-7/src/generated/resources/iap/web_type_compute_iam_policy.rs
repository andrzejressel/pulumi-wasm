/// Three different resources help you manage your IAM policy for Identity-Aware Proxy WebTypeCompute. Each of these resources serves a different use case:
///
/// * `gcp.iap.WebTypeComputeIamPolicy`: Authoritative. Sets the IAM policy for the webtypecompute and replaces any existing policy already attached.
/// * `gcp.iap.WebTypeComputeIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the webtypecompute are preserved.
/// * `gcp.iap.WebTypeComputeIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the webtypecompute are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.iap.WebTypeComputeIamPolicy`: Retrieves the IAM policy for the webtypecompute
///
/// > **Note:** `gcp.iap.WebTypeComputeIamPolicy` **cannot** be used in conjunction with `gcp.iap.WebTypeComputeIamBinding` and `gcp.iap.WebTypeComputeIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.iap.WebTypeComputeIamBinding` resources **can be** used in conjunction with `gcp.iap.WebTypeComputeIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.iap.WebTypeComputeIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:WebTypeComputeIamPolicy
///     properties:
///       project: ${projectService.project}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.httpsResourceAccessor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:WebTypeComputeIamPolicy
///     properties:
///       project: ${projectService.project}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.httpsResourceAccessor
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.iap.WebTypeComputeIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = web_type_compute_iam_binding::create(
///         "binding",
///         WebTypeComputeIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${projectService.project}")
///             .role("roles/iap.httpsResourceAccessor")
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
///     let binding = web_type_compute_iam_binding::create(
///         "binding",
///         WebTypeComputeIamBindingArgs::builder()
///             .condition(
///                 WebTypeComputeIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .project("${projectService.project}")
///             .role("roles/iap.httpsResourceAccessor")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.iap.WebTypeComputeIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = web_type_compute_iam_member::create(
///         "member",
///         WebTypeComputeIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${projectService.project}")
///             .role("roles/iap.httpsResourceAccessor")
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
///     let member = web_type_compute_iam_member::create(
///         "member",
///         WebTypeComputeIamMemberArgs::builder()
///             .condition(
///                 WebTypeComputeIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .project("${projectService.project}")
///             .role("roles/iap.httpsResourceAccessor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Identity-Aware Proxy WebTypeCompute
/// Three different resources help you manage your IAM policy for Identity-Aware Proxy WebTypeCompute. Each of these resources serves a different use case:
///
/// * `gcp.iap.WebTypeComputeIamPolicy`: Authoritative. Sets the IAM policy for the webtypecompute and replaces any existing policy already attached.
/// * `gcp.iap.WebTypeComputeIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the webtypecompute are preserved.
/// * `gcp.iap.WebTypeComputeIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the webtypecompute are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.iap.WebTypeComputeIamPolicy`: Retrieves the IAM policy for the webtypecompute
///
/// > **Note:** `gcp.iap.WebTypeComputeIamPolicy` **cannot** be used in conjunction with `gcp.iap.WebTypeComputeIamBinding` and `gcp.iap.WebTypeComputeIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.iap.WebTypeComputeIamBinding` resources **can be** used in conjunction with `gcp.iap.WebTypeComputeIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.iap.WebTypeComputeIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:WebTypeComputeIamPolicy
///     properties:
///       project: ${projectService.project}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.httpsResourceAccessor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:WebTypeComputeIamPolicy
///     properties:
///       project: ${projectService.project}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.httpsResourceAccessor
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.iap.WebTypeComputeIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = web_type_compute_iam_binding::create(
///         "binding",
///         WebTypeComputeIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${projectService.project}")
///             .role("roles/iap.httpsResourceAccessor")
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
///     let binding = web_type_compute_iam_binding::create(
///         "binding",
///         WebTypeComputeIamBindingArgs::builder()
///             .condition(
///                 WebTypeComputeIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .project("${projectService.project}")
///             .role("roles/iap.httpsResourceAccessor")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.iap.WebTypeComputeIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = web_type_compute_iam_member::create(
///         "member",
///         WebTypeComputeIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${projectService.project}")
///             .role("roles/iap.httpsResourceAccessor")
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
///     let member = web_type_compute_iam_member::create(
///         "member",
///         WebTypeComputeIamMemberArgs::builder()
///             .condition(
///                 WebTypeComputeIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .project("${projectService.project}")
///             .role("roles/iap.httpsResourceAccessor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/iap_web/compute
///
/// * {{project}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Identity-Aware Proxy webtypecompute IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/webTypeComputeIamPolicy:WebTypeComputeIamPolicy editor "projects/{{project}}/iap_web/compute roles/iap.httpsResourceAccessor user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/webTypeComputeIamPolicy:WebTypeComputeIamPolicy editor "projects/{{project}}/iap_web/compute roles/iap.httpsResourceAccessor"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/webTypeComputeIamPolicy:WebTypeComputeIamPolicy editor projects/{{project}}/iap_web/compute
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_type_compute_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebTypeComputeIamPolicyArgs {
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WebTypeComputeIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebTypeComputeIamPolicyArgs,
    ) -> WebTypeComputeIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:iap/webTypeComputeIamPolicy:WebTypeComputeIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: policy_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebTypeComputeIamPolicyResult {
            etag: o.get_field("etag"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
        }
    }
}
