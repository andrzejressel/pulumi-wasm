/// Provides a resource for managing Cloudflare Pages domains.
///
/// > A DNS record for the domain is not automatically created. You need to create
///    a `cloudflare.Record` resource for the domain you want to use.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   my-domain:
///     type: cloudflare:PagesDomain
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       projectName: my-example-project
///       domain: example.com
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/pagesDomain:PagesDomain example <account_id>/<project_name>/<domain-name>
/// ```
///
pub mod pages_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PagesDomainArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Custom domain. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub domain: pulumi_wasm_rust::Output<String>,
        /// Name of the Pages Project. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub project_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PagesDomainResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Custom domain. **Modifying this attribute will force creation of a new resource.**
        pub domain: pulumi_wasm_rust::Output<String>,
        /// Name of the Pages Project. **Modifying this attribute will force creation of a new resource.**
        pub project_name: pulumi_wasm_rust::Output<String>,
        /// Status of the custom domain.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PagesDomainArgs) -> PagesDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let domain_binding = args.domain.get_inner();
        let project_name_binding = args.project_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/pagesDomain:PagesDomain".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "projectName".into(),
                    value: &project_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "projectName".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PagesDomainResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            project_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectName").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
