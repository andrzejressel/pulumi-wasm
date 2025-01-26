/// Three different resources help you manage your IAM policy for Dataplex Zone. Each of these resources serves a different use case:
///
/// * `gcp.dataplex.ZoneIamPolicy`: Authoritative. Sets the IAM policy for the zone and replaces any existing policy already attached.
/// * `gcp.dataplex.ZoneIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the zone are preserved.
/// * `gcp.dataplex.ZoneIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the zone are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.dataplex.ZoneIamPolicy`: Retrieves the IAM policy for the zone
///
/// > **Note:** `gcp.dataplex.ZoneIamPolicy` **cannot** be used in conjunction with `gcp.dataplex.ZoneIamBinding` and `gcp.dataplex.ZoneIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.dataplex.ZoneIamBinding` resources **can be** used in conjunction with `gcp.dataplex.ZoneIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.dataplex.ZoneIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:dataplex:ZoneIamPolicy
///     properties:
///       project: ${example.project}
///       location: ${example.location}
///       lake: ${example.lake}
///       dataplexZone: ${example.name}
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
/// ## gcp.dataplex.ZoneIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = zone_iam_binding::create(
///         "binding",
///         ZoneIamBindingArgs::builder()
///             .dataplex_zone("${example.name}")
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
/// ## gcp.dataplex.ZoneIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = zone_iam_member::create(
///         "member",
///         ZoneIamMemberArgs::builder()
///             .dataplex_zone("${example.name}")
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
/// # IAM policy for Dataplex Zone
/// Three different resources help you manage your IAM policy for Dataplex Zone. Each of these resources serves a different use case:
///
/// * `gcp.dataplex.ZoneIamPolicy`: Authoritative. Sets the IAM policy for the zone and replaces any existing policy already attached.
/// * `gcp.dataplex.ZoneIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the zone are preserved.
/// * `gcp.dataplex.ZoneIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the zone are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.dataplex.ZoneIamPolicy`: Retrieves the IAM policy for the zone
///
/// > **Note:** `gcp.dataplex.ZoneIamPolicy` **cannot** be used in conjunction with `gcp.dataplex.ZoneIamBinding` and `gcp.dataplex.ZoneIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.dataplex.ZoneIamBinding` resources **can be** used in conjunction with `gcp.dataplex.ZoneIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.dataplex.ZoneIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:dataplex:ZoneIamPolicy
///     properties:
///       project: ${example.project}
///       location: ${example.location}
///       lake: ${example.lake}
///       dataplexZone: ${example.name}
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
/// ## gcp.dataplex.ZoneIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = zone_iam_binding::create(
///         "binding",
///         ZoneIamBindingArgs::builder()
///             .dataplex_zone("${example.name}")
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
/// ## gcp.dataplex.ZoneIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = zone_iam_member::create(
///         "member",
///         ZoneIamMemberArgs::builder()
///             .dataplex_zone("${example.name}")
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
/// * projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{name}}
///
/// * {{project}}/{{location}}/{{lake}}/{{name}}
///
/// * {{location}}/{{lake}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Dataplex zone IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/zoneIamMember:ZoneIamMember editor "projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{zone}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/zoneIamMember:ZoneIamMember editor "projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{zone}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/zoneIamMember:ZoneIamMember editor projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{zone}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod zone_iam_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneIamMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataplex::ZoneIamMemberCondition>,
        >,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub dataplex_zone: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub lake: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
        pub member: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.dataplex.ZoneIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneIamMemberResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::dataplex::ZoneIamMemberCondition>,
        >,
        /// Used to find the parent resource to bind the IAM policy to
        pub dataplex_zone: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        pub lake: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
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
        pub member: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.dataplex.ZoneIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ZoneIamMemberArgs,
    ) -> ZoneIamMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_output(context).get_inner();
        let dataplex_zone_binding = args.dataplex_zone.get_output(context).get_inner();
        let lake_binding = args.lake.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let member_binding = args.member.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataplex/zoneIamMember:ZoneIamMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
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
                    name: "member".into(),
                    value: &member_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZoneIamMemberResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            dataplex_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataplexZone"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            lake: pulumi_wasm_rust::__private::into_domain(o.extract_field("lake")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            member: pulumi_wasm_rust::__private::into_domain(o.extract_field("member")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            role: pulumi_wasm_rust::__private::into_domain(o.extract_field("role")),
        }
    }
}
