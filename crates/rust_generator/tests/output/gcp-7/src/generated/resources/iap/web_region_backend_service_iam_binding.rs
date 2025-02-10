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
/// $ pulumi import gcp:iap/webRegionBackendServiceIamBinding:WebRegionBackendServiceIamBinding editor "projects/{{project}}/iap_web/compute-{{region}}/services/{{web_region_backend_service}} roles/iap.httpsResourceAccessor user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/webRegionBackendServiceIamBinding:WebRegionBackendServiceIamBinding editor "projects/{{project}}/iap_web/compute-{{region}}/services/{{web_region_backend_service}} roles/iap.httpsResourceAccessor"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/webRegionBackendServiceIamBinding:WebRegionBackendServiceIamBinding editor projects/{{project}}/iap_web/compute-{{region}}/services/{{web_region_backend_service}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_region_backend_service_iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebRegionBackendServiceIamBindingArgs {
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iap::WebRegionBackendServiceIamBindingCondition>,
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
        pub members: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.iap.WebRegionBackendServiceIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub web_region_backend_service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WebRegionBackendServiceIamBindingResult {
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::iap::WebRegionBackendServiceIamBindingCondition>,
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
        pub members: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.iap.WebRegionBackendServiceIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub web_region_backend_service: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebRegionBackendServiceIamBindingArgs,
    ) -> WebRegionBackendServiceIamBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let members_binding = args.members.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let role_binding = args.role.get_output(context);
        let web_region_backend_service_binding = args
            .web_region_backend_service
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:iap/webRegionBackendServiceIamBinding:WebRegionBackendServiceIamBinding"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: condition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "members".into(),
                    value: members_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webRegionBackendService".into(),
                    value: web_region_backend_service_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebRegionBackendServiceIamBindingResult {
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            members: o.get_field("members"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            role: o.get_field("role"),
            web_region_backend_service: o.get_field("webRegionBackendService"),
        }
    }
}
