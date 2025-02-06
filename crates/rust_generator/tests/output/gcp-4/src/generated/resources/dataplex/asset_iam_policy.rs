/// Three different resources help you manage your IAM policy for Dataplex Asset. Each of these resources serves a different use case:
///
/// * `gcp.dataplex.AssetIamPolicy`: Authoritative. Sets the IAM policy for the asset and replaces any existing policy already attached.
/// * `gcp.dataplex.AssetIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the asset are preserved.
/// * `gcp.dataplex.AssetIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the asset are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.dataplex.AssetIamPolicy`: Retrieves the IAM policy for the asset
///
/// > **Note:** `gcp.dataplex.AssetIamPolicy` **cannot** be used in conjunction with `gcp.dataplex.AssetIamBinding` and `gcp.dataplex.AssetIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.dataplex.AssetIamBinding` resources **can be** used in conjunction with `gcp.dataplex.AssetIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.dataplex.AssetIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:dataplex:AssetIamPolicy
///     properties:
///       project: ${example.project}
///       location: ${example.location}
///       lake: ${example.lake}
///       dataplexZone: ${example.dataplexZone}
///       asset: ${example.name}
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
/// ## gcp.dataplex.AssetIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = asset_iam_binding::create(
///         "binding",
///         AssetIamBindingArgs::builder()
///             .asset("${example.name}")
///             .dataplex_zone("${example.dataplexZone}")
///             .lake("${example.lake}")
///             .location("${example.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${example.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataplex.AssetIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = asset_iam_member::create(
///         "member",
///         AssetIamMemberArgs::builder()
///             .asset("${example.name}")
///             .dataplex_zone("${example.dataplexZone}")
///             .lake("${example.lake}")
///             .location("${example.location}")
///             .member("user:jane@example.com")
///             .project("${example.project}")
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
/// # IAM policy for Dataplex Asset
/// Three different resources help you manage your IAM policy for Dataplex Asset. Each of these resources serves a different use case:
///
/// * `gcp.dataplex.AssetIamPolicy`: Authoritative. Sets the IAM policy for the asset and replaces any existing policy already attached.
/// * `gcp.dataplex.AssetIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the asset are preserved.
/// * `gcp.dataplex.AssetIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the asset are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.dataplex.AssetIamPolicy`: Retrieves the IAM policy for the asset
///
/// > **Note:** `gcp.dataplex.AssetIamPolicy` **cannot** be used in conjunction with `gcp.dataplex.AssetIamBinding` and `gcp.dataplex.AssetIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.dataplex.AssetIamBinding` resources **can be** used in conjunction with `gcp.dataplex.AssetIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.dataplex.AssetIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:dataplex:AssetIamPolicy
///     properties:
///       project: ${example.project}
///       location: ${example.location}
///       lake: ${example.lake}
///       dataplexZone: ${example.dataplexZone}
///       asset: ${example.name}
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
/// ## gcp.dataplex.AssetIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = asset_iam_binding::create(
///         "binding",
///         AssetIamBindingArgs::builder()
///             .asset("${example.name}")
///             .dataplex_zone("${example.dataplexZone}")
///             .lake("${example.lake}")
///             .location("${example.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${example.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataplex.AssetIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = asset_iam_member::create(
///         "member",
///         AssetIamMemberArgs::builder()
///             .asset("${example.name}")
///             .dataplex_zone("${example.dataplexZone}")
///             .lake("${example.lake}")
///             .location("${example.location}")
///             .member("user:jane@example.com")
///             .project("${example.project}")
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
/// * projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{dataplex_zone}}/assets/{{name}}
///
/// * {{project}}/{{location}}/{{lake}}/{{dataplex_zone}}/{{name}}
///
/// * {{location}}/{{lake}}/{{dataplex_zone}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Dataplex asset IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/assetIamPolicy:AssetIamPolicy editor "projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{dataplex_zone}}/assets/{{asset}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/assetIamPolicy:AssetIamPolicy editor "projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{dataplex_zone}}/assets/{{asset}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/assetIamPolicy:AssetIamPolicy editor projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{dataplex_zone}}/assets/{{asset}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod asset_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssetIamPolicyArgs {
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub asset: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub dataplex_zone: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub lake: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub struct AssetIamPolicyResult {
        /// Used to find the parent resource to bind the IAM policy to
        pub asset: pulumi_wasm_rust::Output<String>,
        pub dataplex_zone: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        pub lake: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
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
        args: AssetIamPolicyArgs,
    ) -> AssetIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let asset_binding = args.asset.get_output(context).get_inner();
        let dataplex_zone_binding = args.dataplex_zone.get_output(context).get_inner();
        let lake_binding = args.lake.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let policy_data_binding = args.policy_data.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataplex/assetIamPolicy:AssetIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "asset".into(),
                    value: &asset_binding,
                },
                register_interface::ObjectField {
                    name: "dataplexZone".into(),
                    value: &dataplex_zone_binding,
                },
                register_interface::ObjectField {
                    name: "lake".into(),
                    value: &lake_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
        AssetIamPolicyResult {
            asset: pulumi_wasm_rust::__private::into_domain(o.extract_field("asset")),
            dataplex_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataplexZone"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            lake: pulumi_wasm_rust::__private::into_domain(o.extract_field("lake")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(o.extract_field("project")),
        }
    }
}
