/// Three different resources help you manage your IAM policy for Cloud AI Notebooks Runtime. Each of these resources serves a different use case:
///
/// * `gcp.notebooks.RuntimeIamPolicy`: Authoritative. Sets the IAM policy for the runtime and replaces any existing policy already attached.
/// * `gcp.notebooks.RuntimeIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the runtime are preserved.
/// * `gcp.notebooks.RuntimeIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the runtime are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.notebooks.RuntimeIamPolicy`: Retrieves the IAM policy for the runtime
///
/// > **Note:** `gcp.notebooks.RuntimeIamPolicy` **cannot** be used in conjunction with `gcp.notebooks.RuntimeIamBinding` and `gcp.notebooks.RuntimeIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.notebooks.RuntimeIamBinding` resources **can be** used in conjunction with `gcp.notebooks.RuntimeIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.notebooks.RuntimeIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:notebooks:RuntimeIamPolicy
///     properties:
///       project: ${runtime.project}
///       location: ${runtime.location}
///       runtimeName: ${runtime.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.notebooks.RuntimeIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = runtime_iam_binding::create(
///         "binding",
///         RuntimeIamBindingArgs::builder()
///             .location("${runtime.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${runtime.project}")
///             .role("roles/viewer")
///             .runtime_name("${runtime.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.notebooks.RuntimeIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = runtime_iam_member::create(
///         "member",
///         RuntimeIamMemberArgs::builder()
///             .location("${runtime.location}")
///             .member("user:jane@example.com")
///             .project("${runtime.project}")
///             .role("roles/viewer")
///             .runtime_name("${runtime.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Cloud AI Notebooks Runtime
/// Three different resources help you manage your IAM policy for Cloud AI Notebooks Runtime. Each of these resources serves a different use case:
///
/// * `gcp.notebooks.RuntimeIamPolicy`: Authoritative. Sets the IAM policy for the runtime and replaces any existing policy already attached.
/// * `gcp.notebooks.RuntimeIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the runtime are preserved.
/// * `gcp.notebooks.RuntimeIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the runtime are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.notebooks.RuntimeIamPolicy`: Retrieves the IAM policy for the runtime
///
/// > **Note:** `gcp.notebooks.RuntimeIamPolicy` **cannot** be used in conjunction with `gcp.notebooks.RuntimeIamBinding` and `gcp.notebooks.RuntimeIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.notebooks.RuntimeIamBinding` resources **can be** used in conjunction with `gcp.notebooks.RuntimeIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.notebooks.RuntimeIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:notebooks:RuntimeIamPolicy
///     properties:
///       project: ${runtime.project}
///       location: ${runtime.location}
///       runtimeName: ${runtime.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.notebooks.RuntimeIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = runtime_iam_binding::create(
///         "binding",
///         RuntimeIamBindingArgs::builder()
///             .location("${runtime.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${runtime.project}")
///             .role("roles/viewer")
///             .runtime_name("${runtime.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.notebooks.RuntimeIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = runtime_iam_member::create(
///         "member",
///         RuntimeIamMemberArgs::builder()
///             .location("${runtime.location}")
///             .member("user:jane@example.com")
///             .project("${runtime.project}")
///             .role("roles/viewer")
///             .runtime_name("${runtime.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/{{location}}/runtimes/{{runtime_name}}
///
/// * {{project}}/{{location}}/{{runtime_name}}
///
/// * {{location}}/{{runtime_name}}
///
/// * {{runtime_name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud AI Notebooks runtime IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:notebooks/runtimeIamPolicy:RuntimeIamPolicy editor "projects/{{project}}/locations/{{location}}/runtimes/{{runtime_name}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:notebooks/runtimeIamPolicy:RuntimeIamPolicy editor "projects/{{project}}/locations/{{location}}/runtimes/{{runtime_name}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:notebooks/runtimeIamPolicy:RuntimeIamPolicy editor projects/{{project}}/locations/{{location}}/runtimes/{{runtime_name}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod runtime_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuntimeIamPolicyArgs {
        /// A reference to the zone where the machine resides. Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub runtime_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RuntimeIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// A reference to the zone where the machine resides. Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub runtime_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RuntimeIamPolicyArgs,
    ) -> RuntimeIamPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let runtime_name_binding = args.runtime_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:notebooks/runtimeIamPolicy:RuntimeIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: policy_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtimeName".into(),
                    value: runtime_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RuntimeIamPolicyResult {
            etag: o.get_field("etag"),
            location: o.get_field("location"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
            runtime_name: o.get_field("runtimeName"),
        }
    }
}
