/// Provides a Cloudflare Device Managed Network resource. Device managed networks allow for building location-aware device settings policies.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let managedNetworks = device_managed_networks::create(
///         "managedNetworks",
///         DeviceManagedNetworksArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .config(
///                 DeviceManagedNetworksConfig::builder()
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
/// $ pulumi import cloudflare:index/deviceManagedNetworks:DeviceManagedNetworks example <account_id>/<device_managed_networks_id>
/// ```
///
pub mod device_managed_networks {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceManagedNetworksArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The configuration containing information for the WARP client to detect the managed network.
        #[builder(into)]
        pub config: pulumi_wasm_rust::InputOrOutput<
            super::types::DeviceManagedNetworksConfig,
        >,
        /// The name of the Device Managed Network. Must be unique.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of Device Managed Network. Available values: `tls`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DeviceManagedNetworksResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The configuration containing information for the WARP client to detect the managed network.
        pub config: pulumi_wasm_rust::Output<super::types::DeviceManagedNetworksConfig>,
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
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DeviceManagedNetworksArgs,
    ) -> DeviceManagedNetworksResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let config_binding = args.config.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/deviceManagedNetworks:DeviceManagedNetworks".into(),
            name: name.to_string(),
            version: super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DeviceManagedNetworksResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            config: pulumi_wasm_rust::__private::into_domain(o.extract_field("config")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
