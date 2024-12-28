/// The [Hyperdrive Config](https://developers.cloudflare.com/hyperdrive/) resource allows you to manage Cloudflare Hyperdrive Configs.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let noDefaults = hyperdrive_config::create(
///         "noDefaults",
///         HyperdriveConfigArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .name("my-hyperdrive-config")
///             .origin(
///                 HyperdriveConfigOrigin::builder()
///                     .database("postgres")
///                     .host("my-database.example.com")
///                     .password("my-password")
///                     .port(5432)
///                     .scheme("postgres")
///                     .user("my-user")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/hyperdriveConfig:HyperdriveConfig example <account_id>/<hyperdrive_config_id>
/// ```
///
pub mod hyperdrive_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HyperdriveConfigArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The caching details for the Hyperdrive configuration.
        #[builder(into, default)]
        pub caching: pulumi_wasm_rust::Output<
            Option<super::types::HyperdriveConfigCaching>,
        >,
        /// The name of the Hyperdrive configuration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The origin details for the Hyperdrive configuration.
        #[builder(into)]
        pub origin: pulumi_wasm_rust::Output<super::types::HyperdriveConfigOrigin>,
        /// The identifier of this resource. This is the hyperdrive config value.
        #[builder(into, default)]
        pub resource_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HyperdriveConfigResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The caching details for the Hyperdrive configuration.
        pub caching: pulumi_wasm_rust::Output<super::types::HyperdriveConfigCaching>,
        /// The name of the Hyperdrive configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The origin details for the Hyperdrive configuration.
        pub origin: pulumi_wasm_rust::Output<super::types::HyperdriveConfigOrigin>,
        /// The identifier of this resource. This is the hyperdrive config value.
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HyperdriveConfigArgs) -> HyperdriveConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let caching_binding = args.caching.get_inner();
        let name_binding = args.name.get_inner();
        let origin_binding = args.origin.get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/hyperdriveConfig:HyperdriveConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "caching".into(),
                    value: &caching_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "origin".into(),
                    value: &origin_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "caching".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "origin".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HyperdriveConfigResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            caching: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("caching").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            origin: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("origin").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
        }
    }
}
