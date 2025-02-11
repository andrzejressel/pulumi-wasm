/// Three different resources help you manage your IAM policy for Dataplex EntryGroup. Each of these resources serves a different use case:
///
/// * `gcp.dataplex.EntryGroupIamPolicy`: Authoritative. Sets the IAM policy for the entrygroup and replaces any existing policy already attached.
/// * `gcp.dataplex.EntryGroupIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the entrygroup are preserved.
/// * `gcp.dataplex.EntryGroupIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the entrygroup are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.dataplex.EntryGroupIamPolicy`: Retrieves the IAM policy for the entrygroup
///
/// > **Note:** `gcp.dataplex.EntryGroupIamPolicy` **cannot** be used in conjunction with `gcp.dataplex.EntryGroupIamBinding` and `gcp.dataplex.EntryGroupIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.dataplex.EntryGroupIamBinding` resources **can be** used in conjunction with `gcp.dataplex.EntryGroupIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.dataplex.EntryGroupIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:dataplex:EntryGroupIamPolicy
///     properties:
///       project: ${testEntryGroupBasic.project}
///       location: ${testEntryGroupBasic.location}
///       entryGroupId: ${testEntryGroupBasic.entryGroupId}
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
/// ## gcp.dataplex.EntryGroupIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = entry_group_iam_binding::create(
///         "binding",
///         EntryGroupIamBindingArgs::builder()
///             .entry_group_id("${testEntryGroupBasic.entryGroupId}")
///             .location("${testEntryGroupBasic.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${testEntryGroupBasic.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataplex.EntryGroupIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = entry_group_iam_member::create(
///         "member",
///         EntryGroupIamMemberArgs::builder()
///             .entry_group_id("${testEntryGroupBasic.entryGroupId}")
///             .location("${testEntryGroupBasic.location}")
///             .member("user:jane@example.com")
///             .project("${testEntryGroupBasic.project}")
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
/// # IAM policy for Dataplex EntryGroup
/// Three different resources help you manage your IAM policy for Dataplex EntryGroup. Each of these resources serves a different use case:
///
/// * `gcp.dataplex.EntryGroupIamPolicy`: Authoritative. Sets the IAM policy for the entrygroup and replaces any existing policy already attached.
/// * `gcp.dataplex.EntryGroupIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the entrygroup are preserved.
/// * `gcp.dataplex.EntryGroupIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the entrygroup are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.dataplex.EntryGroupIamPolicy`: Retrieves the IAM policy for the entrygroup
///
/// > **Note:** `gcp.dataplex.EntryGroupIamPolicy` **cannot** be used in conjunction with `gcp.dataplex.EntryGroupIamBinding` and `gcp.dataplex.EntryGroupIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.dataplex.EntryGroupIamBinding` resources **can be** used in conjunction with `gcp.dataplex.EntryGroupIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.dataplex.EntryGroupIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:dataplex:EntryGroupIamPolicy
///     properties:
///       project: ${testEntryGroupBasic.project}
///       location: ${testEntryGroupBasic.location}
///       entryGroupId: ${testEntryGroupBasic.entryGroupId}
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
/// ## gcp.dataplex.EntryGroupIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = entry_group_iam_binding::create(
///         "binding",
///         EntryGroupIamBindingArgs::builder()
///             .entry_group_id("${testEntryGroupBasic.entryGroupId}")
///             .location("${testEntryGroupBasic.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${testEntryGroupBasic.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataplex.EntryGroupIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = entry_group_iam_member::create(
///         "member",
///         EntryGroupIamMemberArgs::builder()
///             .entry_group_id("${testEntryGroupBasic.entryGroupId}")
///             .location("${testEntryGroupBasic.location}")
///             .member("user:jane@example.com")
///             .project("${testEntryGroupBasic.project}")
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
/// * projects/{{project}}/locations/{{location}}/entryGroups/{{entry_group_id}}
///
/// * {{project}}/{{location}}/{{entry_group_id}}
///
/// * {{location}}/{{entry_group_id}}
///
/// * {{entry_group_id}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Dataplex entrygroup IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/entryGroupIamPolicy:EntryGroupIamPolicy editor "projects/{{project}}/locations/{{location}}/entryGroups/{{entry_group_id}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/entryGroupIamPolicy:EntryGroupIamPolicy editor "projects/{{project}}/locations/{{location}}/entryGroups/{{entry_group_id}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/entryGroupIamPolicy:EntryGroupIamPolicy editor projects/{{project}}/locations/{{location}}/entryGroups/{{entry_group_id}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod entry_group_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EntryGroupIamPolicyArgs {
        #[builder(into)]
        pub entry_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location where entry group will be created in.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
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
    }
    #[allow(dead_code)]
    pub struct EntryGroupIamPolicyResult {
        pub entry_group_id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The location where entry group will be created in.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_gestalt_rust::Output<String>,
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
        args: EntryGroupIamPolicyArgs,
    ) -> EntryGroupIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let entry_group_id_binding = args.entry_group_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataplex/entryGroupIamPolicy:EntryGroupIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entryGroupId".into(),
                    value: &entry_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
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
        EntryGroupIamPolicyResult {
            entry_group_id: o.get_field("entryGroupId"),
            etag: o.get_field("etag"),
            location: o.get_field("location"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
        }
    }
}
