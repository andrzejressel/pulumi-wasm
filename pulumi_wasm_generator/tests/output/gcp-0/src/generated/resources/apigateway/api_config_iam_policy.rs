/// Three different resources help you manage your IAM policy for API Gateway ApiConfig. Each of these resources serves a different use case:
///
/// * `gcp.apigateway.ApiConfigIamPolicy`: Authoritative. Sets the IAM policy for the apiconfig and replaces any existing policy already attached.
/// * `gcp.apigateway.ApiConfigIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the apiconfig are preserved.
/// * `gcp.apigateway.ApiConfigIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the apiconfig are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.apigateway.ApiConfigIamPolicy`: Retrieves the IAM policy for the apiconfig
///
/// > **Note:** `gcp.apigateway.ApiConfigIamPolicy` **cannot** be used in conjunction with `gcp.apigateway.ApiConfigIamBinding` and `gcp.apigateway.ApiConfigIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.apigateway.ApiConfigIamBinding` resources **can be** used in conjunction with `gcp.apigateway.ApiConfigIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## google\_api\_gateway\_api\_config\_iam\_policy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:apigateway:ApiConfigIamPolicy
///     properties:
///       api: ${apiCfg.api}
///       apiConfig: ${apiCfg.apiConfigId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/apigateway.viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.apigateway.ApiConfigIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = api_config_iam_binding::create(
///         "binding",
///         ApiConfigIamBindingArgs::builder()
///             .api("${apiCfg.api}")
///             .api_config("${apiCfg.apiConfigId}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/apigateway.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.apigateway.ApiConfigIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = api_config_iam_member::create(
///         "member",
///         ApiConfigIamMemberArgs::builder()
///             .api("${apiCfg.api}")
///             .api_config("${apiCfg.apiConfigId}")
///             .member("user:jane@example.com")
///             .role("roles/apigateway.viewer")
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
/// # IAM policy for API Gateway ApiConfig
/// Three different resources help you manage your IAM policy for API Gateway ApiConfig. Each of these resources serves a different use case:
///
/// * `gcp.apigateway.ApiConfigIamPolicy`: Authoritative. Sets the IAM policy for the apiconfig and replaces any existing policy already attached.
/// * `gcp.apigateway.ApiConfigIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the apiconfig are preserved.
/// * `gcp.apigateway.ApiConfigIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the apiconfig are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.apigateway.ApiConfigIamPolicy`: Retrieves the IAM policy for the apiconfig
///
/// > **Note:** `gcp.apigateway.ApiConfigIamPolicy` **cannot** be used in conjunction with `gcp.apigateway.ApiConfigIamBinding` and `gcp.apigateway.ApiConfigIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.apigateway.ApiConfigIamBinding` resources **can be** used in conjunction with `gcp.apigateway.ApiConfigIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## google\_api\_gateway\_api\_config\_iam\_policy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:apigateway:ApiConfigIamPolicy
///     properties:
///       api: ${apiCfg.api}
///       apiConfig: ${apiCfg.apiConfigId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/apigateway.viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.apigateway.ApiConfigIamBinding
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = api_config_iam_binding::create(
///         "binding",
///         ApiConfigIamBindingArgs::builder()
///             .api("${apiCfg.api}")
///             .api_config("${apiCfg.apiConfigId}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/apigateway.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.apigateway.ApiConfigIamMember
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = api_config_iam_member::create(
///         "member",
///         ApiConfigIamMemberArgs::builder()
///             .api("${apiCfg.api}")
///             .api_config("${apiCfg.apiConfigId}")
///             .member("user:jane@example.com")
///             .role("roles/apigateway.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/global/apis/{{api}}/configs/{{api_config}}
///
/// * {{project}}/{{api}}/{{api_config}}
///
/// * {{api}}/{{api_config}}
///
/// * {{api_config}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// API Gateway apiconfig IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:apigateway/apiConfigIamPolicy:ApiConfigIamPolicy editor "projects/{{project}}/locations/global/apis/{{api}}/configs/{{api_config}} roles/apigateway.viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:apigateway/apiConfigIamPolicy:ApiConfigIamPolicy editor "projects/{{project}}/locations/global/apis/{{api}}/configs/{{api_config}} roles/apigateway.viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:apigateway/apiConfigIamPolicy:ApiConfigIamPolicy editor projects/{{project}}/locations/global/apis/{{api}}/configs/{{api_config}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod api_config_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiConfigIamPolicyArgs {
        /// The API to attach the config to.
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub api: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub api_config: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub struct ApiConfigIamPolicyResult {
        /// The API to attach the config to.
        /// Used to find the parent resource to bind the IAM policy to
        pub api: pulumi_wasm_rust::Output<String>,
        pub api_config: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
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
        args: ApiConfigIamPolicyArgs,
    ) -> ApiConfigIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_binding = args.api.get_output(context).get_inner();
        let api_config_binding = args.api_config.get_output(context).get_inner();
        let policy_data_binding = args.policy_data.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigateway/apiConfigIamPolicy:ApiConfigIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "api".into(),
                    value: &api_binding,
                },
                register_interface::ObjectField {
                    name: "apiConfig".into(),
                    value: &api_config_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "api".into(),
                },
                register_interface::ResultField {
                    name: "apiConfig".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "policyData".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiConfigIamPolicyResult {
            api: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("api").unwrap(),
            ),
            api_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiConfig").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyData").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
        }
    }
}
