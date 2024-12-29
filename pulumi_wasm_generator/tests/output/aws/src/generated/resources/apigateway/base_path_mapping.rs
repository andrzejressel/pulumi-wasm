/// Connects a custom domain name registered via `aws.apigateway.DomainName`
/// with a deployed API so that its methods can be called via the
/// custom domain name.
///
/// ## Import
///
/// For a non-root `base_path`:
///
/// Using `pulumi import`, import `aws_api_gateway_base_path_mapping` using the domain name and base path. For example:
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
pub mod base_path_mapping {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BasePathMappingArgs {
        /// Path segment that must be prepended to the path when accessing the API via this mapping. If omitted, the API is exposed at the root of the given domain.
        #[builder(into, default)]
        pub base_path: pulumi_wasm_rust::Output<Option<String>>,
        /// Already-registered domain name to connect the API to.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// ID of the API to connect.
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Name of a specific deployment stage to expose at the given path. If omitted, callers may select any stage by including its name as a path element after the base path.
        #[builder(into, default)]
        pub stage_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BasePathMappingResult {
        /// Path segment that must be prepended to the path when accessing the API via this mapping. If omitted, the API is exposed at the root of the given domain.
        pub base_path: pulumi_wasm_rust::Output<Option<String>>,
        /// Already-registered domain name to connect the API to.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// ID of the API to connect.
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Name of a specific deployment stage to expose at the given path. If omitted, callers may select any stage by including its name as a path element after the base path.
        pub stage_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BasePathMappingArgs) -> BasePathMappingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let base_path_binding = args.base_path.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let rest_api_binding = args.rest_api.get_inner();
        let stage_name_binding = args.stage_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/basePathMapping:BasePathMapping".into(),
            name: name.to_string(),
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
                    name: "restApi".into(),
                    value: &rest_api_binding,
                },
                register_interface::ObjectField {
                    name: "stageName".into(),
                    value: &stage_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "basePath".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "restApi".into(),
                },
                register_interface::ResultField {
                    name: "stageName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BasePathMappingResult {
            base_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("basePath").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApi").unwrap(),
            ),
            stage_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stageName").unwrap(),
            ),
        }
    }
}
