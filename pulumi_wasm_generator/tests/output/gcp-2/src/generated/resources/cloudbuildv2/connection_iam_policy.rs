/// Three different resources help you manage your IAM policy for Cloud Build v2 Connection. Each of these resources serves a different use case:
///
/// * `gcp.cloudbuildv2.ConnectionIAMPolicy`: Authoritative. Sets the IAM policy for the connection and replaces any existing policy already attached.
/// * `gcp.cloudbuildv2.ConnectionIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the connection are preserved.
/// * `gcp.cloudbuildv2.ConnectionIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the connection are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.cloudbuildv2.ConnectionIAMPolicy`: Retrieves the IAM policy for the connection
///
/// > **Note:** `gcp.cloudbuildv2.ConnectionIAMPolicy` **cannot** be used in conjunction with `gcp.cloudbuildv2.ConnectionIAMBinding` and `gcp.cloudbuildv2.ConnectionIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.cloudbuildv2.ConnectionIAMBinding` resources **can be** used in conjunction with `gcp.cloudbuildv2.ConnectionIAMMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.cloudbuildv2.ConnectionIAMPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:cloudbuildv2:ConnectionIAMPolicy
///     properties:
///       project: ${["my-connection"].project}
///       location: ${["my-connection"].location}
///       name: ${["my-connection"].name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/cloudbuild.connectionViewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.cloudbuildv2.ConnectionIAMBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = connection_iam_binding::create(
///         "binding",
///         ConnectionIamBindingArgs::builder()
///             .location("${[\"my-connection\"].location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${[\"my-connection\"].name}")
///             .project("${[\"my-connection\"].project}")
///             .role("roles/cloudbuild.connectionViewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.cloudbuildv2.ConnectionIAMMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = connection_iam_member::create(
///         "member",
///         ConnectionIamMemberArgs::builder()
///             .location("${[\"my-connection\"].location}")
///             .member("user:jane@example.com")
///             .name("${[\"my-connection\"].name}")
///             .project("${[\"my-connection\"].project}")
///             .role("roles/cloudbuild.connectionViewer")
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
/// # IAM policy for Cloud Build v2 Connection
/// Three different resources help you manage your IAM policy for Cloud Build v2 Connection. Each of these resources serves a different use case:
///
/// * `gcp.cloudbuildv2.ConnectionIAMPolicy`: Authoritative. Sets the IAM policy for the connection and replaces any existing policy already attached.
/// * `gcp.cloudbuildv2.ConnectionIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the connection are preserved.
/// * `gcp.cloudbuildv2.ConnectionIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the connection are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.cloudbuildv2.ConnectionIAMPolicy`: Retrieves the IAM policy for the connection
///
/// > **Note:** `gcp.cloudbuildv2.ConnectionIAMPolicy` **cannot** be used in conjunction with `gcp.cloudbuildv2.ConnectionIAMBinding` and `gcp.cloudbuildv2.ConnectionIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.cloudbuildv2.ConnectionIAMBinding` resources **can be** used in conjunction with `gcp.cloudbuildv2.ConnectionIAMMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.cloudbuildv2.ConnectionIAMPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:cloudbuildv2:ConnectionIAMPolicy
///     properties:
///       project: ${["my-connection"].project}
///       location: ${["my-connection"].location}
///       name: ${["my-connection"].name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/cloudbuild.connectionViewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.cloudbuildv2.ConnectionIAMBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = connection_iam_binding::create(
///         "binding",
///         ConnectionIamBindingArgs::builder()
///             .location("${[\"my-connection\"].location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${[\"my-connection\"].name}")
///             .project("${[\"my-connection\"].project}")
///             .role("roles/cloudbuild.connectionViewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.cloudbuildv2.ConnectionIAMMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = connection_iam_member::create(
///         "member",
///         ConnectionIamMemberArgs::builder()
///             .location("${[\"my-connection\"].location}")
///             .member("user:jane@example.com")
///             .name("${[\"my-connection\"].name}")
///             .project("${[\"my-connection\"].project}")
///             .role("roles/cloudbuild.connectionViewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/{{location}}/connections/{{name}}
///
/// * {{project}}/{{location}}/{{name}}
///
/// * {{location}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud Build v2 connection IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/connectionIAMPolicy:ConnectionIAMPolicy editor "projects/{{project}}/locations/{{location}}/connections/{{connection}} roles/cloudbuild.connectionViewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/connectionIAMPolicy:ConnectionIAMPolicy editor "projects/{{project}}/locations/{{location}}/connections/{{connection}} roles/cloudbuild.connectionViewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/connectionIAMPolicy:ConnectionIAMPolicy editor projects/{{project}}/locations/{{location}}/connections/{{connection}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod connection_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionIAMPolicyArgs {
        /// The location for the resource Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub struct ConnectionIAMPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The location for the resource Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_wasm_rust::Output<String>,
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
        args: ConnectionIAMPolicyArgs,
    ) -> ConnectionIAMPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let policy_data_binding = args.policy_data.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudbuildv2/connectionIAMPolicy:ConnectionIAMPolicy".into(),
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
        ConnectionIAMPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(o.extract_field("project")),
        }
    }
}
