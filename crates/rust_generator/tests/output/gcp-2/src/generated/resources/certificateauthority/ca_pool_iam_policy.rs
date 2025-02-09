/// Three different resources help you manage your IAM policy for Certificate Authority Service CaPool. Each of these resources serves a different use case:
///
/// * `gcp.certificateauthority.CaPoolIamPolicy`: Authoritative. Sets the IAM policy for the capool and replaces any existing policy already attached.
/// * `gcp.certificateauthority.CaPoolIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the capool are preserved.
/// * `gcp.certificateauthority.CaPoolIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the capool are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.certificateauthority.CaPoolIamPolicy`: Retrieves the IAM policy for the capool
///
/// > **Note:** `gcp.certificateauthority.CaPoolIamPolicy` **cannot** be used in conjunction with `gcp.certificateauthority.CaPoolIamBinding` and `gcp.certificateauthority.CaPoolIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.certificateauthority.CaPoolIamBinding` resources **can be** used in conjunction with `gcp.certificateauthority.CaPoolIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.certificateauthority.CaPoolIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:certificateauthority:CaPoolIamPolicy
///     properties:
///       caPool: ${default.id}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/privateca.certificateManager
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:certificateauthority:CaPoolIamPolicy
///     properties:
///       caPool: ${default.id}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/privateca.certificateManager
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.certificateauthority.CaPoolIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = ca_pool_iam_binding::create(
///         "binding",
///         CaPoolIamBindingArgs::builder()
///             .ca_pool("${default.id}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/privateca.certificateManager")
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
///     let binding = ca_pool_iam_binding::create(
///         "binding",
///         CaPoolIamBindingArgs::builder()
///             .ca_pool("${default.id}")
///             .condition(
///                 CaPoolIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .role("roles/privateca.certificateManager")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.certificateauthority.CaPoolIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = ca_pool_iam_member::create(
///         "member",
///         CaPoolIamMemberArgs::builder()
///             .ca_pool("${default.id}")
///             .member("user:jane@example.com")
///             .role("roles/privateca.certificateManager")
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
///     let member = ca_pool_iam_member::create(
///         "member",
///         CaPoolIamMemberArgs::builder()
///             .ca_pool("${default.id}")
///             .condition(
///                 CaPoolIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .role("roles/privateca.certificateManager")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Certificate Authority Service CaPool
/// Three different resources help you manage your IAM policy for Certificate Authority Service CaPool. Each of these resources serves a different use case:
///
/// * `gcp.certificateauthority.CaPoolIamPolicy`: Authoritative. Sets the IAM policy for the capool and replaces any existing policy already attached.
/// * `gcp.certificateauthority.CaPoolIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the capool are preserved.
/// * `gcp.certificateauthority.CaPoolIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the capool are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.certificateauthority.CaPoolIamPolicy`: Retrieves the IAM policy for the capool
///
/// > **Note:** `gcp.certificateauthority.CaPoolIamPolicy` **cannot** be used in conjunction with `gcp.certificateauthority.CaPoolIamBinding` and `gcp.certificateauthority.CaPoolIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.certificateauthority.CaPoolIamBinding` resources **can be** used in conjunction with `gcp.certificateauthority.CaPoolIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.certificateauthority.CaPoolIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:certificateauthority:CaPoolIamPolicy
///     properties:
///       caPool: ${default.id}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/privateca.certificateManager
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:certificateauthority:CaPoolIamPolicy
///     properties:
///       caPool: ${default.id}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/privateca.certificateManager
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.certificateauthority.CaPoolIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = ca_pool_iam_binding::create(
///         "binding",
///         CaPoolIamBindingArgs::builder()
///             .ca_pool("${default.id}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/privateca.certificateManager")
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
///     let binding = ca_pool_iam_binding::create(
///         "binding",
///         CaPoolIamBindingArgs::builder()
///             .ca_pool("${default.id}")
///             .condition(
///                 CaPoolIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .role("roles/privateca.certificateManager")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.certificateauthority.CaPoolIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = ca_pool_iam_member::create(
///         "member",
///         CaPoolIamMemberArgs::builder()
///             .ca_pool("${default.id}")
///             .member("user:jane@example.com")
///             .role("roles/privateca.certificateManager")
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
///     let member = ca_pool_iam_member::create(
///         "member",
///         CaPoolIamMemberArgs::builder()
///             .ca_pool("${default.id}")
///             .condition(
///                 CaPoolIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .role("roles/privateca.certificateManager")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/{{location}}/caPools/{{name}}
///
/// * {{project}}/{{location}}/{{name}}
///
/// * {{location}}/{{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Certificate Authority Service capool IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/caPoolIamPolicy:CaPoolIamPolicy editor "projects/{{project}}/locations/{{location}}/caPools/{{ca_pool}} roles/privateca.certificateManager user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/caPoolIamPolicy:CaPoolIamPolicy editor "projects/{{project}}/locations/{{location}}/caPools/{{ca_pool}} roles/privateca.certificateManager"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/caPoolIamPolicy:CaPoolIamPolicy editor projects/{{project}}/locations/{{location}}/caPools/{{ca_pool}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ca_pool_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CaPoolIamPolicyArgs {
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub ca_pool: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Location of the CaPool. A full list of valid locations can be found by
        /// running `gcloud privateca locations list`.
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
    pub struct CaPoolIamPolicyResult {
        /// Used to find the parent resource to bind the IAM policy to
        pub ca_pool: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Location of the CaPool. A full list of valid locations can be found by
        /// running `gcloud privateca locations list`.
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CaPoolIamPolicyArgs,
    ) -> CaPoolIamPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let ca_pool_binding_1 = args.ca_pool.get_output(context);
        let ca_pool_binding = ca_pool_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let policy_data_binding_1 = args.policy_data.get_output(context);
        let policy_data_binding = policy_data_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:certificateauthority/caPoolIamPolicy:CaPoolIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "caPool".into(),
                    value: &ca_pool_binding,
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
        CaPoolIamPolicyResult {
            ca_pool: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("caPool"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            policy_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
        }
    }
}
