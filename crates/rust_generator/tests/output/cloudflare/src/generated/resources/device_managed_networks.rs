/// Provides a Cloudflare Device Managed Network resource. Device managed networks allow for building location-aware device settings policies.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod device_managed_networks {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceManagedNetworksArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The configuration containing information for the WARP client to detect the managed network.
        #[builder(into)]
        pub config: pulumi_gestalt_rust::InputOrOutput<
            super::types::DeviceManagedNetworksConfig,
        >,
        /// The name of the Device Managed Network. Must be unique.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of Device Managed Network. Available values: `tls`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DeviceManagedNetworksResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The configuration containing information for the WARP client to detect the managed network.
        pub config: pulumi_gestalt_rust::Output<
            super::types::DeviceManagedNetworksConfig,
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeviceManagedNetworksArgs,
    ) -> DeviceManagedNetworksResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let config_binding = args.config.get_output(context);
        let name_binding = args.name.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/deviceManagedNetworks:DeviceManagedNetworks".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "config".into(),
                    value: config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DeviceManagedNetworksResult {
            account_id: o.get_field("accountId"),
            config: o.get_field("config"),
            name: o.get_field("name"),
            type_: o.get_field("type"),
        }
    }
}
