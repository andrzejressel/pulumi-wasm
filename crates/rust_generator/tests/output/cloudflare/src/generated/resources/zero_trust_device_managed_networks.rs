/// Provides a Cloudflare Device Managed Network resource. Device managed networks allow for building location-aware device settings policies.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_device_managed_networks {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustDeviceManagedNetworksArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The configuration containing information for the WARP client to detect the managed network.
        #[builder(into)]
        pub config: pulumi_gestalt_rust::InputOrOutput<
            super::types::ZeroTrustDeviceManagedNetworksConfig,
        >,
        /// The name of the Device Managed Network. Must be unique.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of Device Managed Network. Available values: `tls`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustDeviceManagedNetworksResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The configuration containing information for the WARP client to detect the managed network.
        pub config: pulumi_gestalt_rust::Output<
            super::types::ZeroTrustDeviceManagedNetworksConfig,
        >,
        /// The name of the Device Managed Network. Must be unique.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The type of Device Managed Network. Available values: `tls`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZeroTrustDeviceManagedNetworksArgs,
    ) -> ZeroTrustDeviceManagedNetworksResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let config_binding_1 = args.config.get_output(context);
        let config_binding = config_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let type__binding_1 = args.type_.get_output(context);
        let type__binding = type__binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustDeviceManagedNetworks:ZeroTrustDeviceManagedNetworks"
                .into(),
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
        ZeroTrustDeviceManagedNetworksResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("config"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
