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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// $ pulumi import gcp:dataplex/assetIamMember:AssetIamMember editor "projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{dataplex_zone}}/assets/{{asset}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/assetIamMember:AssetIamMember editor "projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{dataplex_zone}}/assets/{{asset}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/assetIamMember:AssetIamMember editor projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{dataplex_zone}}/assets/{{asset}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod asset_iam_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssetIamMemberArgs {
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub asset: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataplex::AssetIamMemberCondition>,
        >,
        #[builder(into)]
        pub dataplex_zone: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub lake: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        #[builder(into)]
        pub member: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.dataplex.AssetIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AssetIamMemberResult {
        /// Used to find the parent resource to bind the IAM policy to
        pub asset: pulumi_gestalt_rust::Output<String>,
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataplex::AssetIamMemberCondition>,
        >,
        pub dataplex_zone: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub lake: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        pub member: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.dataplex.AssetIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssetIamMemberArgs,
    ) -> AssetIamMemberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let asset_binding = args.asset.get_output(context);
        let condition_binding = args.condition.get_output(context);
        let dataplex_zone_binding = args.dataplex_zone.get_output(context);
        let lake_binding = args.lake.get_output(context);
        let location_binding = args.location.get_output(context);
        let member_binding = args.member.get_output(context);
        let project_binding = args.project.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataplex/assetIamMember:AssetIamMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "asset".into(),
                    value: asset_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: condition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataplexZone".into(),
                    value: dataplex_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lake".into(),
                    value: lake_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
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
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AssetIamMemberResult {
            asset: o.get_field("asset"),
            condition: o.get_field("condition"),
            dataplex_zone: o.get_field("dataplexZone"),
            etag: o.get_field("etag"),
            lake: o.get_field("lake"),
            location: o.get_field("location"),
            member: o.get_field("member"),
            project: o.get_field("project"),
            role: o.get_field("role"),
        }
    }
}
