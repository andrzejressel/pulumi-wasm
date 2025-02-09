/// Three different resources help you manage your IAM policy for Identity-Aware Proxy WebRegionBackendService. Each of these resources serves a different use case:
///
/// * `gcp.iap.WebRegionBackendServiceIamPolicy`: Authoritative. Sets the IAM policy for the webregionbackendservice and replaces any existing policy already attached.
/// * `gcp.iap.WebRegionBackendServiceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the webregionbackendservice are preserved.
/// * `gcp.iap.WebRegionBackendServiceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the webregionbackendservice are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.iap.WebRegionBackendServiceIamPolicy`: Retrieves the IAM policy for the webregionbackendservice
///
/// > **Note:** `gcp.iap.WebRegionBackendServiceIamPolicy` **cannot** be used in conjunction with `gcp.iap.WebRegionBackendServiceIamBinding` and `gcp.iap.WebRegionBackendServiceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.iap.WebRegionBackendServiceIamBinding` resources **can be** used in conjunction with `gcp.iap.WebRegionBackendServiceIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.iap.WebRegionBackendServiceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:WebRegionBackendServiceIamPolicy
///     properties:
///       project: ${default.project}
///       region: ${default.region}
///       webRegionBackendService: ${default.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.httpsResourceAccessor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:WebRegionBackendServiceIamPolicy
///     properties:
///       project: ${default.project}
///       region: ${default.region}
///       webRegionBackendService: ${default.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.httpsResourceAccessor
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.iap.WebRegionBackendServiceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = web_region_backend_service_iam_binding::create(
///         "binding",
///         WebRegionBackendServiceIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${default.project}")
///             .region("${default.region}")
///             .role("roles/iap.httpsResourceAccessor")
///             .web_region_backend_service("${default.name}")
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
///     let binding = web_region_backend_service_iam_binding::create(
///         "binding",
///         WebRegionBackendServiceIamBindingArgs::builder()
///             .condition(
///                 WebRegionBackendServiceIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .project("${default.project}")
///             .region("${default.region}")
///             .role("roles/iap.httpsResourceAccessor")
///             .web_region_backend_service("${default.name}")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.iap.WebRegionBackendServiceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = web_region_backend_service_iam_member::create(
///         "member",
///         WebRegionBackendServiceIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${default.project}")
///             .region("${default.region}")
///             .role("roles/iap.httpsResourceAccessor")
///             .web_region_backend_service("${default.name}")
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
///     let member = web_region_backend_service_iam_member::create(
///         "member",
///         WebRegionBackendServiceIamMemberArgs::builder()
///             .condition(
///                 WebRegionBackendServiceIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .project("${default.project}")
///             .region("${default.region}")
///             .role("roles/iap.httpsResourceAccessor")
///             .web_region_backend_service("${default.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Identity-Aware Proxy WebRegionBackendService
/// Three different resources help you manage your IAM policy for Identity-Aware Proxy WebRegionBackendService. Each of these resources serves a different use case:
///
/// * `gcp.iap.WebRegionBackendServiceIamPolicy`: Authoritative. Sets the IAM policy for the webregionbackendservice and replaces any existing policy already attached.
/// * `gcp.iap.WebRegionBackendServiceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the webregionbackendservice are preserved.
/// * `gcp.iap.WebRegionBackendServiceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the webregionbackendservice are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.iap.WebRegionBackendServiceIamPolicy`: Retrieves the IAM policy for the webregionbackendservice
///
/// > **Note:** `gcp.iap.WebRegionBackendServiceIamPolicy` **cannot** be used in conjunction with `gcp.iap.WebRegionBackendServiceIamBinding` and `gcp.iap.WebRegionBackendServiceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.iap.WebRegionBackendServiceIamBinding` resources **can be** used in conjunction with `gcp.iap.WebRegionBackendServiceIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.iap.WebRegionBackendServiceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:WebRegionBackendServiceIamPolicy
///     properties:
///       project: ${default.project}
///       region: ${default.region}
///       webRegionBackendService: ${default.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.httpsResourceAccessor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:WebRegionBackendServiceIamPolicy
///     properties:
///       project: ${default.project}
///       region: ${default.region}
///       webRegionBackendService: ${default.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.httpsResourceAccessor
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.iap.WebRegionBackendServiceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = web_region_backend_service_iam_binding::create(
///         "binding",
///         WebRegionBackendServiceIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${default.project}")
///             .region("${default.region}")
///             .role("roles/iap.httpsResourceAccessor")
///             .web_region_backend_service("${default.name}")
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
///     let binding = web_region_backend_service_iam_binding::create(
///         "binding",
///         WebRegionBackendServiceIamBindingArgs::builder()
///             .condition(
///                 WebRegionBackendServiceIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .project("${default.project}")
///             .region("${default.region}")
///             .role("roles/iap.httpsResourceAccessor")
///             .web_region_backend_service("${default.name}")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.iap.WebRegionBackendServiceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = web_region_backend_service_iam_member::create(
///         "member",
///         WebRegionBackendServiceIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${default.project}")
///             .region("${default.region}")
///             .role("roles/iap.httpsResourceAccessor")
///             .web_region_backend_service("${default.name}")
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
///     let member = web_region_backend_service_iam_member::create(
///         "member",
///         WebRegionBackendServiceIamMemberArgs::builder()
///             .condition(
///                 WebRegionBackendServiceIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .project("${default.project}")
///             .region("${default.region}")
///             .role("roles/iap.httpsResourceAccessor")
///             .web_region_backend_service("${default.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/iap_web/compute-{{region}}/services/{{name}}
///
/// * {{project}}/{{region}}/{{name}}
///
/// * {{region}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Identity-Aware Proxy webregionbackendservice IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/webRegionBackendServiceIamPolicy:WebRegionBackendServiceIamPolicy editor "projects/{{project}}/iap_web/compute-{{region}}/services/{{web_region_backend_service}} roles/iap.httpsResourceAccessor user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/webRegionBackendServiceIamPolicy:WebRegionBackendServiceIamPolicy editor "projects/{{project}}/iap_web/compute-{{region}}/services/{{web_region_backend_service}} roles/iap.httpsResourceAccessor"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/webRegionBackendServiceIamPolicy:WebRegionBackendServiceIamPolicy editor projects/{{project}}/iap_web/compute-{{region}}/services/{{web_region_backend_service}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_region_backend_service_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebRegionBackendServiceIamPolicyArgs {
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub web_region_backend_service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WebRegionBackendServiceIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub web_region_backend_service: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WebRegionBackendServiceIamPolicyArgs,
    ) -> WebRegionBackendServiceIamPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let policy_data_binding_1 = args.policy_data.get_output(context);
        let policy_data_binding = policy_data_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let web_region_backend_service_binding_1 = args
            .web_region_backend_service
            .get_output(context);
        let web_region_backend_service_binding = web_region_backend_service_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:iap/webRegionBackendServiceIamPolicy:WebRegionBackendServiceIamPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
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
                    name: "webRegionBackendService".into(),
                    value: &web_region_backend_service_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WebRegionBackendServiceIamPolicyResult {
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            policy_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            web_region_backend_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("webRegionBackendService"),
            ),
        }
    }
}
