/// Three different resources help you manage your IAM policy for Cloud Run (v2 API) Service. Each of these resources serves a different use case:
///
/// * `gcp.cloudrunv2.ServiceIamPolicy`: Authoritative. Sets the IAM policy for the service and replaces any existing policy already attached.
/// * `gcp.cloudrunv2.ServiceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the service are preserved.
/// * `gcp.cloudrunv2.ServiceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the service are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.cloudrunv2.ServiceIamPolicy`: Retrieves the IAM policy for the service
///
/// > **Note:** `gcp.cloudrunv2.ServiceIamPolicy` **cannot** be used in conjunction with `gcp.cloudrunv2.ServiceIamBinding` and `gcp.cloudrunv2.ServiceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.cloudrunv2.ServiceIamBinding` resources **can be** used in conjunction with `gcp.cloudrunv2.ServiceIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.cloudrunv2.ServiceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:cloudrunv2:ServiceIamPolicy
///     properties:
///       project: ${default.project}
///       location: ${default.location}
///       name: ${default.name}
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
/// ## gcp.cloudrunv2.ServiceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = service_iam_binding::create(
///         "binding",
///         ServiceIamBindingArgs::builder()
///             .location("${default.location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${default.name}")
///             .project("${default.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.cloudrunv2.ServiceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = service_iam_member::create(
///         "member",
///         ServiceIamMemberArgs::builder()
///             .location("${default.location}")
///             .member("user:jane@example.com")
///             .name("${default.name}")
///             .project("${default.project}")
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
/// # IAM policy for Cloud Run (v2 API) Service
/// Three different resources help you manage your IAM policy for Cloud Run (v2 API) Service. Each of these resources serves a different use case:
///
/// * `gcp.cloudrunv2.ServiceIamPolicy`: Authoritative. Sets the IAM policy for the service and replaces any existing policy already attached.
/// * `gcp.cloudrunv2.ServiceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the service are preserved.
/// * `gcp.cloudrunv2.ServiceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the service are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.cloudrunv2.ServiceIamPolicy`: Retrieves the IAM policy for the service
///
/// > **Note:** `gcp.cloudrunv2.ServiceIamPolicy` **cannot** be used in conjunction with `gcp.cloudrunv2.ServiceIamBinding` and `gcp.cloudrunv2.ServiceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.cloudrunv2.ServiceIamBinding` resources **can be** used in conjunction with `gcp.cloudrunv2.ServiceIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.cloudrunv2.ServiceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:cloudrunv2:ServiceIamPolicy
///     properties:
///       project: ${default.project}
///       location: ${default.location}
///       name: ${default.name}
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
/// ## gcp.cloudrunv2.ServiceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = service_iam_binding::create(
///         "binding",
///         ServiceIamBindingArgs::builder()
///             .location("${default.location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${default.name}")
///             .project("${default.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.cloudrunv2.ServiceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = service_iam_member::create(
///         "member",
///         ServiceIamMemberArgs::builder()
///             .location("${default.location}")
///             .member("user:jane@example.com")
///             .name("${default.name}")
///             .project("${default.project}")
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
/// * projects/{{project}}/locations/{{location}}/services/{{name}}
///
/// * {{project}}/{{location}}/{{name}}
///
/// * {{location}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud Run (v2 API) service IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:cloudrunv2/serviceIamPolicy:ServiceIamPolicy editor "projects/{{project}}/locations/{{location}}/services/{{service}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:cloudrunv2/serviceIamPolicy:ServiceIamPolicy editor "projects/{{project}}/locations/{{location}}/services/{{service}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:cloudrunv2/serviceIamPolicy:ServiceIamPolicy editor projects/{{project}}/locations/{{location}}/services/{{service}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceIamPolicyArgs {
        /// The location of the cloud run service Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
    pub struct ServiceIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The location of the cloud run service Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub name: pulumi_gestalt_rust::Output<String>,
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
        args: ServiceIamPolicyArgs,
    ) -> ServiceIamPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let policy_data_binding_1 = args.policy_data.get_output(context);
        let policy_data_binding = policy_data_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudrunv2/serviceIamPolicy:ServiceIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
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
        ServiceIamPolicyResult {
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            policy_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
        }
    }
}
