/// Three different resources help you manage your IAM policy for Compute Engine RegionDisk. Each of these resources serves a different use case:
///
/// * `gcp.compute.RegionDiskIamPolicy`: Authoritative. Sets the IAM policy for the regiondisk and replaces any existing policy already attached.
/// * `gcp.compute.RegionDiskIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the regiondisk are preserved.
/// * `gcp.compute.RegionDiskIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the regiondisk are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.compute.RegionDiskIamPolicy`: Retrieves the IAM policy for the regiondisk
///
/// > **Note:** `gcp.compute.RegionDiskIamPolicy` **cannot** be used in conjunction with `gcp.compute.RegionDiskIamBinding` and `gcp.compute.RegionDiskIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.compute.RegionDiskIamBinding` resources **can be** used in conjunction with `gcp.compute.RegionDiskIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.compute.RegionDiskIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:RegionDiskIamPolicy
///     properties:
///       project: ${regiondisk.project}
///       region: ${regiondisk.region}
///       name: ${regiondisk.name}
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
/// ## gcp.compute.RegionDiskIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = region_disk_iam_binding::create(
///         "binding",
///         RegionDiskIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .name("${regiondisk.name}")
///             .project("${regiondisk.project}")
///             .region("${regiondisk.region}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.compute.RegionDiskIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = region_disk_iam_member::create(
///         "member",
///         RegionDiskIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .name("${regiondisk.name}")
///             .project("${regiondisk.project}")
///             .region("${regiondisk.region}")
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
/// # IAM policy for Compute Engine RegionDisk
/// Three different resources help you manage your IAM policy for Compute Engine RegionDisk. Each of these resources serves a different use case:
///
/// * `gcp.compute.RegionDiskIamPolicy`: Authoritative. Sets the IAM policy for the regiondisk and replaces any existing policy already attached.
/// * `gcp.compute.RegionDiskIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the regiondisk are preserved.
/// * `gcp.compute.RegionDiskIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the regiondisk are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.compute.RegionDiskIamPolicy`: Retrieves the IAM policy for the regiondisk
///
/// > **Note:** `gcp.compute.RegionDiskIamPolicy` **cannot** be used in conjunction with `gcp.compute.RegionDiskIamBinding` and `gcp.compute.RegionDiskIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.compute.RegionDiskIamBinding` resources **can be** used in conjunction with `gcp.compute.RegionDiskIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.compute.RegionDiskIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:RegionDiskIamPolicy
///     properties:
///       project: ${regiondisk.project}
///       region: ${regiondisk.region}
///       name: ${regiondisk.name}
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
/// ## gcp.compute.RegionDiskIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = region_disk_iam_binding::create(
///         "binding",
///         RegionDiskIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .name("${regiondisk.name}")
///             .project("${regiondisk.project}")
///             .region("${regiondisk.region}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.compute.RegionDiskIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = region_disk_iam_member::create(
///         "member",
///         RegionDiskIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .name("${regiondisk.name}")
///             .project("${regiondisk.project}")
///             .region("${regiondisk.region}")
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
/// * projects/{{project}}/regions/{{region}}/disks/{{name}}
///
/// * {{project}}/{{region}}/{{name}}
///
/// * {{region}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Compute Engine regiondisk IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:compute/regionDiskIamPolicy:RegionDiskIamPolicy editor "projects/{{project}}/regions/{{region}}/disks/{{region_disk}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:compute/regionDiskIamPolicy:RegionDiskIamPolicy editor "projects/{{project}}/regions/{{region}}/disks/{{region_disk}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:compute/regionDiskIamPolicy:RegionDiskIamPolicy editor projects/{{project}}/regions/{{region}}/disks/{{region_disk}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod region_disk_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionDiskIamPolicyArgs {
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
        /// A reference to the region where the disk resides. Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no region is provided in the parent identifier and no
        /// region is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegionDiskIamPolicyResult {
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
        /// A reference to the region where the disk resides. Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no region is provided in the parent identifier and no
        /// region is specified, it is taken from the provider configuration.
        pub region: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegionDiskIamPolicyArgs,
    ) -> RegionDiskIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let policy_data_binding = args.policy_data.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionDiskIamPolicy:RegionDiskIamPolicy".into(),
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
                    name: "name".into(),
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegionDiskIamPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
