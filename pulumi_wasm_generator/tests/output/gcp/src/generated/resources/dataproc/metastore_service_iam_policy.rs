/// Three different resources help you manage your IAM policy for Dataproc metastore Service. Each of these resources serves a different use case:
///
/// * `gcp.dataproc.MetastoreServiceIamPolicy`: Authoritative. Sets the IAM policy for the service and replaces any existing policy already attached.
/// * `gcp.dataproc.MetastoreServiceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the service are preserved.
/// * `gcp.dataproc.MetastoreServiceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the service are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.dataproc.MetastoreServiceIamPolicy`: Retrieves the IAM policy for the service
///
/// > **Note:** `gcp.dataproc.MetastoreServiceIamPolicy` **cannot** be used in conjunction with `gcp.dataproc.MetastoreServiceIamBinding` and `gcp.dataproc.MetastoreServiceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.dataproc.MetastoreServiceIamBinding` resources **can be** used in conjunction with `gcp.dataproc.MetastoreServiceIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.dataproc.MetastoreServiceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:dataproc:MetastoreServiceIamPolicy
///     properties:
///       project: ${default.project}
///       location: ${default.location}
///       serviceId: ${default.serviceId}
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
/// ## gcp.dataproc.MetastoreServiceIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = metastore_service_iam_binding::create(
///         "binding",
///         MetastoreServiceIamBindingArgs::builder()
///             .location("${default.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${default.project}")
///             .role("roles/viewer")
///             .service_id("${default.serviceId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataproc.MetastoreServiceIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = metastore_service_iam_member::create(
///         "member",
///         MetastoreServiceIamMemberArgs::builder()
///             .location("${default.location}")
///             .member("user:jane@example.com")
///             .project("${default.project}")
///             .role("roles/viewer")
///             .service_id("${default.serviceId}")
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
/// # IAM policy for Dataproc metastore Service
/// Three different resources help you manage your IAM policy for Dataproc metastore Service. Each of these resources serves a different use case:
///
/// * `gcp.dataproc.MetastoreServiceIamPolicy`: Authoritative. Sets the IAM policy for the service and replaces any existing policy already attached.
/// * `gcp.dataproc.MetastoreServiceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the service are preserved.
/// * `gcp.dataproc.MetastoreServiceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the service are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.dataproc.MetastoreServiceIamPolicy`: Retrieves the IAM policy for the service
///
/// > **Note:** `gcp.dataproc.MetastoreServiceIamPolicy` **cannot** be used in conjunction with `gcp.dataproc.MetastoreServiceIamBinding` and `gcp.dataproc.MetastoreServiceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.dataproc.MetastoreServiceIamBinding` resources **can be** used in conjunction with `gcp.dataproc.MetastoreServiceIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.dataproc.MetastoreServiceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:dataproc:MetastoreServiceIamPolicy
///     properties:
///       project: ${default.project}
///       location: ${default.location}
///       serviceId: ${default.serviceId}
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
/// ## gcp.dataproc.MetastoreServiceIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = metastore_service_iam_binding::create(
///         "binding",
///         MetastoreServiceIamBindingArgs::builder()
///             .location("${default.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${default.project}")
///             .role("roles/viewer")
///             .service_id("${default.serviceId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataproc.MetastoreServiceIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = metastore_service_iam_member::create(
///         "member",
///         MetastoreServiceIamMemberArgs::builder()
///             .location("${default.location}")
///             .member("user:jane@example.com")
///             .project("${default.project}")
///             .role("roles/viewer")
///             .service_id("${default.serviceId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/{{location}}/services/{{service_id}}
///
/// * {{project}}/{{location}}/{{service_id}}
///
/// * {{location}}/{{service_id}}
///
/// * {{service_id}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Dataproc metastore service IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataproc/metastoreServiceIamPolicy:MetastoreServiceIamPolicy editor "projects/{{project}}/locations/{{location}}/services/{{service_id}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataproc/metastoreServiceIamPolicy:MetastoreServiceIamPolicy editor "projects/{{project}}/locations/{{location}}/services/{{service_id}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataproc/metastoreServiceIamPolicy:MetastoreServiceIamPolicy editor projects/{{project}}/locations/{{location}}/services/{{service_id}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod metastore_service_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetastoreServiceIamPolicyArgs {
        /// The location where the metastore service should reside.
        /// The default value is `global`.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into)]
        pub service_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MetastoreServiceIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The location where the metastore service should reside.
        /// The default value is `global`.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        pub service_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: MetastoreServiceIamPolicyArgs,
    ) -> MetastoreServiceIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let policy_data_binding = args.policy_data.get_inner();
        let project_binding = args.project.get_inner();
        let service_id_binding = args.service_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataproc/metastoreServiceIamPolicy:MetastoreServiceIamPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
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
                register_interface::ObjectField {
                    name: "serviceId".into(),
                    value: &service_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "policyData".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "serviceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MetastoreServiceIamPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyData").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceId").unwrap(),
            ),
        }
    }
}
