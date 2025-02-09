/// Three different resources help you manage your IAM policy for Data catalog Taxonomy. Each of these resources serves a different use case:
///
/// * `gcp.datacatalog.TaxonomyIamPolicy`: Authoritative. Sets the IAM policy for the taxonomy and replaces any existing policy already attached.
/// * `gcp.datacatalog.TaxonomyIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the taxonomy are preserved.
/// * `gcp.datacatalog.TaxonomyIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the taxonomy are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.datacatalog.TaxonomyIamPolicy`: Retrieves the IAM policy for the taxonomy
///
/// > **Note:** `gcp.datacatalog.TaxonomyIamPolicy` **cannot** be used in conjunction with `gcp.datacatalog.TaxonomyIamBinding` and `gcp.datacatalog.TaxonomyIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.datacatalog.TaxonomyIamBinding` resources **can be** used in conjunction with `gcp.datacatalog.TaxonomyIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.datacatalog.TaxonomyIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:datacatalog:TaxonomyIamPolicy
///     properties:
///       taxonomy: ${basicTaxonomy.name}
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
/// ## gcp.datacatalog.TaxonomyIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = taxonomy_iam_binding::create(
///         "binding",
///         TaxonomyIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .role("roles/viewer")
///             .taxonomy("${basicTaxonomy.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.datacatalog.TaxonomyIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = taxonomy_iam_member::create(
///         "member",
///         TaxonomyIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .role("roles/viewer")
///             .taxonomy("${basicTaxonomy.name}")
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
/// # IAM policy for Data catalog Taxonomy
/// Three different resources help you manage your IAM policy for Data catalog Taxonomy. Each of these resources serves a different use case:
///
/// * `gcp.datacatalog.TaxonomyIamPolicy`: Authoritative. Sets the IAM policy for the taxonomy and replaces any existing policy already attached.
/// * `gcp.datacatalog.TaxonomyIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the taxonomy are preserved.
/// * `gcp.datacatalog.TaxonomyIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the taxonomy are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.datacatalog.TaxonomyIamPolicy`: Retrieves the IAM policy for the taxonomy
///
/// > **Note:** `gcp.datacatalog.TaxonomyIamPolicy` **cannot** be used in conjunction with `gcp.datacatalog.TaxonomyIamBinding` and `gcp.datacatalog.TaxonomyIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.datacatalog.TaxonomyIamBinding` resources **can be** used in conjunction with `gcp.datacatalog.TaxonomyIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.datacatalog.TaxonomyIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:datacatalog:TaxonomyIamPolicy
///     properties:
///       taxonomy: ${basicTaxonomy.name}
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
/// ## gcp.datacatalog.TaxonomyIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = taxonomy_iam_binding::create(
///         "binding",
///         TaxonomyIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .role("roles/viewer")
///             .taxonomy("${basicTaxonomy.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.datacatalog.TaxonomyIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = taxonomy_iam_member::create(
///         "member",
///         TaxonomyIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .role("roles/viewer")
///             .taxonomy("${basicTaxonomy.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/{{region}}/taxonomies/{{taxonomy}}
///
/// * {{project}}/{{region}}/{{taxonomy}}
///
/// * {{region}}/{{taxonomy}}
///
/// * {{taxonomy}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Data catalog taxonomy IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:datacatalog/taxonomyIamMember:TaxonomyIamMember editor "projects/{{project}}/locations/{{region}}/taxonomies/{{taxonomy}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:datacatalog/taxonomyIamMember:TaxonomyIamMember editor "projects/{{project}}/locations/{{region}}/taxonomies/{{taxonomy}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:datacatalog/taxonomyIamMember:TaxonomyIamMember editor projects/{{project}}/locations/{{region}}/taxonomies/{{taxonomy}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod taxonomy_iam_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TaxonomyIamMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datacatalog::TaxonomyIamMemberCondition>,
        >,
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
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.datacatalog.TaxonomyIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub taxonomy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TaxonomyIamMemberResult {
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::datacatalog::TaxonomyIamMemberCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
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
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.datacatalog.TaxonomyIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub taxonomy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TaxonomyIamMemberArgs,
    ) -> TaxonomyIamMemberResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let condition_binding_1 = args.condition.get_output(context);
        let condition_binding = condition_binding_1.get_inner();
        let member_binding_1 = args.member.get_output(context);
        let member_binding = member_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let role_binding_1 = args.role.get_output(context);
        let role_binding = role_binding_1.get_inner();
        let taxonomy_binding_1 = args.taxonomy.get_output(context);
        let taxonomy_binding = taxonomy_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:datacatalog/taxonomyIamMember:TaxonomyIamMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
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
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "taxonomy".into(),
                    value: &taxonomy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TaxonomyIamMemberResult {
            condition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            member: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("member"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            role: pulumi_gestalt_rust::__private::into_domain(o.extract_field("role")),
            taxonomy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("taxonomy"),
            ),
        }
    }
}
