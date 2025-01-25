/// Three different resources help you manage your IAM policy for Cloud Endpoints Service. Each of these resources serves a different use case:
///
/// * `gcp.endpoints.ServiceIamPolicy`: Authoritative. Sets the IAM policy for the service and replaces any existing policy already attached.
/// * `gcp.endpoints.ServiceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the service are preserved.
/// * `gcp.endpoints.ServiceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the service are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.endpoints.ServiceIamPolicy`: Retrieves the IAM policy for the service
///
/// > **Note:** `gcp.endpoints.ServiceIamPolicy` **cannot** be used in conjunction with `gcp.endpoints.ServiceIamBinding` and `gcp.endpoints.ServiceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.endpoints.ServiceIamBinding` resources **can be** used in conjunction with `gcp.endpoints.ServiceIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.endpoints.ServiceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:endpoints:ServiceIamPolicy
///     properties:
///       serviceName: ${endpointsService.serviceName}
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
/// ## gcp.endpoints.ServiceIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = service_iam_binding::create(
///         "binding",
///         ServiceIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .role("roles/viewer")
///             .service_name("${endpointsService.serviceName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.endpoints.ServiceIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = service_iam_member::create(
///         "member",
///         ServiceIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .role("roles/viewer")
///             .service_name("${endpointsService.serviceName}")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ## > **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
/// -
///
/// # IAM policy for Cloud Endpoints Service
/// Three different resources help you manage your IAM policy for Cloud Endpoints Service. Each of these resources serves a different use case:
///
/// * `gcp.endpoints.ServiceIamPolicy`: Authoritative. Sets the IAM policy for the service and replaces any existing policy already attached.
/// * `gcp.endpoints.ServiceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the service are preserved.
/// * `gcp.endpoints.ServiceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the service are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.endpoints.ServiceIamPolicy`: Retrieves the IAM policy for the service
///
/// > **Note:** `gcp.endpoints.ServiceIamPolicy` **cannot** be used in conjunction with `gcp.endpoints.ServiceIamBinding` and `gcp.endpoints.ServiceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.endpoints.ServiceIamBinding` resources **can be** used in conjunction with `gcp.endpoints.ServiceIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.endpoints.ServiceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:endpoints:ServiceIamPolicy
///     properties:
///       serviceName: ${endpointsService.serviceName}
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
/// ## gcp.endpoints.ServiceIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = service_iam_binding::create(
///         "binding",
///         ServiceIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .role("roles/viewer")
///             .service_name("${endpointsService.serviceName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.endpoints.ServiceIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = service_iam_member::create(
///         "member",
///         ServiceIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .role("roles/viewer")
///             .service_name("${endpointsService.serviceName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * services/{{service_name}}
///
/// * {{service_name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud Endpoints service IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:endpoints/serviceIamMember:ServiceIamMember editor "services/{{service_name}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:endpoints/serviceIamMember:ServiceIamMember editor "services/{{service_name}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:endpoints/serviceIamMember:ServiceIamMember editor services/{{service_name}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod service_iam_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceIamMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::endpoints::ServiceIamMemberCondition>,
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
        pub member: pulumi_wasm_rust::InputOrOutput<String>,
        /// The role that should be applied. Only one
        /// `gcp.endpoints.ServiceIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub service_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceIamMemberResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::endpoints::ServiceIamMemberCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
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
        /// The role that should be applied. Only one
        /// `gcp.endpoints.ServiceIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
        pub service_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServiceIamMemberArgs,
    ) -> ServiceIamMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_output(context).get_inner();
        let member_binding = args.member.get_output(context).get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let service_name_binding = args.service_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:endpoints/serviceIamMember:ServiceIamMember".into(),
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
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "member".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceIamMemberResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            member: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("member").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
        }
    }
}
