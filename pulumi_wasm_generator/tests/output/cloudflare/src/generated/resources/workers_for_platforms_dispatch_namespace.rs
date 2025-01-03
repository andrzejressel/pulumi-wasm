/// The [Workers for Platforms](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/) resource allows you
/// to manage Cloudflare Workers for Platforms dispatch namespaces.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: cloudflare:WorkersForPlatformsDispatchNamespace
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       name: example-namespace
///   customerWorker1:
///     type: cloudflare:WorkersScript
///     name: customer_worker_1
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       name: customer-worker-1
///       content:
///         fn::invoke:
///           Function: std:file
///           Arguments:
///             input: script.js
///           Return: result
///       dispatchNamespace: ${example.name}
///       tags:
///         - free
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workersForPlatformsDispatchNamespace:WorkersForPlatformsDispatchNamespace example <account_id>/<namespace_name>
/// ```
///
pub mod workers_for_platforms_dispatch_namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkersForPlatformsDispatchNamespaceArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Workers for Platforms namespace.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkersForPlatformsDispatchNamespaceResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Workers for Platforms namespace.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: WorkersForPlatformsDispatchNamespaceArgs,
    ) -> WorkersForPlatformsDispatchNamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/workersForPlatformsDispatchNamespace:WorkersForPlatformsDispatchNamespace"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkersForPlatformsDispatchNamespaceResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
