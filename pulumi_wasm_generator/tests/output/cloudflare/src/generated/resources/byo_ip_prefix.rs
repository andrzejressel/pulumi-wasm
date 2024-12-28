/// Provides the ability to manage Bring-Your-Own-IP prefixes (BYOIP)
/// which are used with or without Magic Transit.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = byo_ip_prefix::create(
///         "example",
///         ByoIpPrefixArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .advertisement("on")
///             .description("Example IP Prefix")
///             .prefix_id("d41d8cd98f00b204e9800998ecf8427e")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/byoIpPrefix:ByoIpPrefix example <account_id>/<prefix_id>
/// ```
///
pub mod byo_ip_prefix {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ByoIpPrefixArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Whether or not the prefix shall be announced. A prefix can be activated or deactivated once every 15 minutes (attempting more regular updates will trigger rate limiting). Available values: `on`, `off`.
        #[builder(into, default)]
        pub advertisement: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the BYO IP prefix.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The assigned Bring-Your-Own-IP prefix ID. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub prefix_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ByoIpPrefixResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Whether or not the prefix shall be announced. A prefix can be activated or deactivated once every 15 minutes (attempting more regular updates will trigger rate limiting). Available values: `on`, `off`.
        pub advertisement: pulumi_wasm_rust::Output<String>,
        /// Description of the BYO IP prefix.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The assigned Bring-Your-Own-IP prefix ID. **Modifying this attribute will force creation of a new resource.**
        pub prefix_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ByoIpPrefixArgs) -> ByoIpPrefixResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let advertisement_binding = args.advertisement.get_inner();
        let description_binding = args.description.get_inner();
        let prefix_id_binding = args.prefix_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/byoIpPrefix:ByoIpPrefix".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "advertisement".into(),
                    value: &advertisement_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "prefixId".into(),
                    value: &prefix_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "advertisement".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "prefixId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ByoIpPrefixResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            advertisement: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advertisement").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            prefix_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefixId").unwrap(),
            ),
        }
    }
}
