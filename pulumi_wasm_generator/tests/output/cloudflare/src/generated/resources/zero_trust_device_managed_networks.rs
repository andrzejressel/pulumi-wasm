/// Provides a Cloudflare Device Managed Network resource. Device managed networks allow for building location-aware device settings policies.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let managedNetworks = zero_trust_device_managed_networks::create(
///         "managedNetworks",
///         ZeroTrustDeviceManagedNetworksArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .config(
///                 ZeroTrustDeviceManagedNetworksConfig::builder()
///                     .sha256(
///                         "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c",
///                     )
///                     .tlsSockaddr("foobar:1234")
///                     .build_struct(),
///             )
///             .name("managed-network-1")
///             .type_("tls")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustDeviceManagedNetworks:ZeroTrustDeviceManagedNetworks example <account_id>/<device_managed_networks_id>
/// ```
///
pub mod zero_trust_device_managed_networks {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustDeviceManagedNetworksArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The configuration containing information for the WARP client to detect the managed network.
        #[builder(into)]
        pub config: pulumi_wasm_rust::Output<
            super::types::ZeroTrustDeviceManagedNetworksConfig,
        >,
        /// The name of the Device Managed Network. Must be unique.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The type of Device Managed Network. Available values: `tls`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustDeviceManagedNetworksResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The configuration containing information for the WARP client to detect the managed network.
        pub config: pulumi_wasm_rust::Output<
            super::types::ZeroTrustDeviceManagedNetworksConfig,
        >,
        /// The name of the Device Managed Network. Must be unique.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The type of Device Managed Network. Available values: `tls`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ZeroTrustDeviceManagedNetworksArgs,
    ) -> ZeroTrustDeviceManagedNetworksResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let config_binding = args.config.get_inner();
        let name_binding = args.name.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustDeviceManagedNetworks:ZeroTrustDeviceManagedNetworks"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
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
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "config".into(),
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
        ZeroTrustDeviceManagedNetworksResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("config").unwrap(),
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
