/// Provides a Cloudflare Account resource. Account is the basic resource for
/// working with Cloudflare zones, teams and users.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create(
///         "example",
///         AccountArgs::builder()
///             .enforce_twofactor(true)
///             .name("some-enterprise-account")
///             .type_("enterprise")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/account:Account example <account_id>
/// ```
///
pub mod account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// Whether 2FA is enforced on the account. Defaults to `false`.
        #[builder(into, default)]
        pub enforce_twofactor: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the account that is displayed in the Cloudflare dashboard.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Account type. Available values: `enterprise`, `standard`. Defaults to `standard`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// Whether 2FA is enforced on the account. Defaults to `false`.
        pub enforce_twofactor: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the account that is displayed in the Cloudflare dashboard.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Account type. Available values: `enterprise`, `standard`. Defaults to `standard`. **Modifying this attribute will force creation of a new resource.**
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccountArgs) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enforce_twofactor_binding = args.enforce_twofactor.get_inner();
        let name_binding = args.name.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/account:Account".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enforceTwofactor".into(),
                    value: &enforce_twofactor_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enforceTwofactor".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountResult {
            enforce_twofactor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enforceTwofactor").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
