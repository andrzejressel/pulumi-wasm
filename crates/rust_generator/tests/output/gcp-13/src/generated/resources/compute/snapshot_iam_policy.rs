/// Three different resources help you manage your IAM policy for Compute Engine Snapshot. Each of these resources serves a different use case:
///
/// * `gcp.compute.SnapshotIamPolicy`: Authoritative. Sets the IAM policy for the snapshot and replaces any existing policy already attached.
/// * `gcp.compute.SnapshotIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the snapshot are preserved.
/// * `gcp.compute.SnapshotIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the snapshot are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.compute.SnapshotIamPolicy`: Retrieves the IAM policy for the snapshot
///
/// > **Note:** `gcp.compute.SnapshotIamPolicy` **cannot** be used in conjunction with `gcp.compute.SnapshotIamBinding` and `gcp.compute.SnapshotIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.compute.SnapshotIamBinding` resources **can be** used in conjunction with `gcp.compute.SnapshotIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.compute.SnapshotIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:SnapshotIamPolicy
///     properties:
///       project: ${snapshot.project}
///       name: ${snapshot.name}
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
/// ## gcp.compute.SnapshotIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = snapshot_iam_binding::create(
///         "binding",
///         SnapshotIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .name("${snapshot.name}")
///             .project("${snapshot.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.compute.SnapshotIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = snapshot_iam_member::create(
///         "member",
///         SnapshotIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .name("${snapshot.name}")
///             .project("${snapshot.project}")
///             .role("roles/viewer")
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
/// # IAM policy for Compute Engine Snapshot
/// Three different resources help you manage your IAM policy for Compute Engine Snapshot. Each of these resources serves a different use case:
///
/// * `gcp.compute.SnapshotIamPolicy`: Authoritative. Sets the IAM policy for the snapshot and replaces any existing policy already attached.
/// * `gcp.compute.SnapshotIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the snapshot are preserved.
/// * `gcp.compute.SnapshotIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the snapshot are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.compute.SnapshotIamPolicy`: Retrieves the IAM policy for the snapshot
///
/// > **Note:** `gcp.compute.SnapshotIamPolicy` **cannot** be used in conjunction with `gcp.compute.SnapshotIamBinding` and `gcp.compute.SnapshotIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.compute.SnapshotIamBinding` resources **can be** used in conjunction with `gcp.compute.SnapshotIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.compute.SnapshotIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:SnapshotIamPolicy
///     properties:
///       project: ${snapshot.project}
///       name: ${snapshot.name}
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
/// ## gcp.compute.SnapshotIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = snapshot_iam_binding::create(
///         "binding",
///         SnapshotIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .name("${snapshot.name}")
///             .project("${snapshot.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.compute.SnapshotIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = snapshot_iam_member::create(
///         "member",
///         SnapshotIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .name("${snapshot.name}")
///             .project("${snapshot.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/global/snapshots/{{name}}
///
/// * {{project}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Compute Engine snapshot IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:compute/snapshotIamPolicy:SnapshotIamPolicy editor "projects/{{project}}/global/snapshots/{{snapshot}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:compute/snapshotIamPolicy:SnapshotIamPolicy editor "projects/{{project}}/global/snapshots/{{snapshot}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:compute/snapshotIamPolicy:SnapshotIamPolicy editor projects/{{project}}/global/snapshots/{{snapshot}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod snapshot_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotIamPolicyArgs {
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SnapshotIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub name: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SnapshotIamPolicyArgs,
    ) -> SnapshotIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let policy_data_binding = args.policy_data.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/snapshotIamPolicy:SnapshotIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SnapshotIamPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(o.extract_field("project")),
        }
    }
}
