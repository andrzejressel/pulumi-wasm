/// Connects a custom domain name registered via `aws.apigateway.DomainName`
/// with a deployed API so that its methods can be called via the
/// custom domain name.
///
/// ## Import
///
/// For a non-root `base_path`:
///
/// For a non-root `base_path` and a private custom domain name:
///
/// Using `pulumi import`, import `aws_api_gateway_base_path_mapping` using the domain name and base path or domain name, base path and domain name ID (for private custom domain names). For example:
///
/// For an empty `base_path` or, in other words, a root path (`/`):
///
/// ```sh
/// $ pulumi import aws:apigateway/basePathMapping:BasePathMapping example example.com/
/// ```
/// For a non-root `base_path`:
///
/// ```sh
/// $ pulumi import aws:apigateway/basePathMapping:BasePathMapping example example.com/base-path
/// ```
/// For a non-root `base_path` and a private custom domain name:
///
/// ```sh
/// $ pulumi import aws:apigateway/basePathMapping:BasePathMapping example api.internal.example.com/base-path/abcde12345
/// ```
pub mod base_path_mapping {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BasePathMappingArgs {
        /// Path segment that must be prepended to the path when accessing the API via this mapping. If omitted, the API is exposed at the root of the given domain.
        #[builder(into, default)]
        pub base_path: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Already-registered domain name to connect the API to.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The identifier for the domain name resource. Supported only for private custom domain names.
        #[builder(into, default)]
        pub domain_name_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the API to connect.
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of a specific deployment stage to expose at the given path. If omitted, callers may select any stage by including its name as a path element after the base path.
        #[builder(into, default)]
        pub stage_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BasePathMappingResult {
        /// Path segment that must be prepended to the path when accessing the API via this mapping. If omitted, the API is exposed at the root of the given domain.
        pub base_path: pulumi_wasm_rust::Output<Option<String>>,
        /// Already-registered domain name to connect the API to.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The identifier for the domain name resource. Supported only for private custom domain names.
        pub domain_name_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the API to connect.
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Name of a specific deployment stage to expose at the given path. If omitted, callers may select any stage by including its name as a path element after the base path.
        pub stage_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BasePathMappingArgs,
    ) -> BasePathMappingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let base_path_binding = args.base_path.get_output(context).get_inner();
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let domain_name_id_binding = args.domain_name_id.get_output(context).get_inner();
        let rest_api_binding = args.rest_api.get_output(context).get_inner();
        let stage_name_binding = args.stage_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/basePathMapping:BasePathMapping".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "basePath".into(),
                    value: &base_path_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "domainNameId".into(),
                    value: &domain_name_id_binding,
                },
                register_interface::ObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding,
                },
                register_interface::ObjectField {
                    name: "stageName".into(),
                    value: &stage_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BasePathMappingResult {
            base_path: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("basePath"),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            domain_name_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainNameId"),
            ),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restApi"),
            ),
            stage_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stageName"),
            ),
        }
    }
}
