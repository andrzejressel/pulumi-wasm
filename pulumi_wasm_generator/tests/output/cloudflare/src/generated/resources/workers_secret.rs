/// Provides a Cloudflare Worker secret resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let mySecret = workers_secret::create(
///         "mySecret",
///         WorkersSecretArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .name("MY_EXAMPLE_SECRET_TEXT")
///             .script_name("script_1")
///             .secret_text("my_secret_value")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workersSecret:WorkersSecret example <account_id>/<script_name>/<secret_name>
/// ```
///
pub mod workers_secret {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkersSecretArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Worker secret. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Worker script to associate the secret with. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub script_name: pulumi_wasm_rust::Output<String>,
        /// The text of the Worker secret. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub secret_text: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkersSecretResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Worker secret. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Worker script to associate the secret with. **Modifying this attribute will force creation of a new resource.**
        pub script_name: pulumi_wasm_rust::Output<String>,
        /// The text of the Worker secret. **Modifying this attribute will force creation of a new resource.**
        pub secret_text: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkersSecretArgs) -> WorkersSecretResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let name_binding = args.name.get_inner();
        let script_name_binding = args.script_name.get_inner();
        let secret_text_binding = args.secret_text.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/workersSecret:WorkersSecret".into(),
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
                register_interface::ObjectField {
                    name: "scriptName".into(),
                    value: &script_name_binding,
                },
                register_interface::ObjectField {
                    name: "secretText".into(),
                    value: &secret_text_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "scriptName".into(),
                },
                register_interface::ResultField {
                    name: "secretText".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkersSecretResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            script_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scriptName").unwrap(),
            ),
            secret_text: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretText").unwrap(),
            ),
        }
    }
}
