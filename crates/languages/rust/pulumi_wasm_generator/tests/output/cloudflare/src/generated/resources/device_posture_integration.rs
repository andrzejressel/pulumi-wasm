/// Provides a Cloudflare Device Posture Integration resource. Device
/// posture integrations configure third-party data providers for device
/// posture rules.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = device_posture_integration::create(
///         "example",
///         DevicePostureIntegrationArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .configs(
///                 vec![
///                     DevicePostureIntegrationConfig::builder()
///                     .apiUrl("https://example.com/api")
///                     .authUrl("https://example.com/connect/token").clientId("client-id")
///                     .clientSecret("client-secret").build_struct(),
///                 ],
///             )
///             .interval("24h")
///             .name("Device posture integration")
///             .type_("workspace_one")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/devicePostureIntegration:DevicePostureIntegration example <account_id>/<device_posture_integration_id>
/// ```
///
pub mod device_posture_integration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DevicePostureIntegrationArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The device posture integration's connection authorization parameters.
        #[builder(into, default)]
        pub configs: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::DevicePostureIntegrationConfig>>,
        >,
        #[builder(into, default)]
        pub identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Indicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`.
        #[builder(into, default)]
        pub interval: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the device posture integration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`, `sentinelone_s2s`, `tanium_s2s`, `custom_s2s`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DevicePostureIntegrationResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The device posture integration's connection authorization parameters.
        pub configs: pulumi_wasm_rust::Output<
            Option<Vec<super::types::DevicePostureIntegrationConfig>>,
        >,
        pub identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`.
        pub interval: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the device posture integration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`, `sentinelone_s2s`, `tanium_s2s`, `custom_s2s`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DevicePostureIntegrationArgs,
    ) -> DevicePostureIntegrationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let configs_binding = args.configs.get_output(context).get_inner();
        let identifier_binding = args.identifier.get_output(context).get_inner();
        let interval_binding = args.interval.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/devicePostureIntegration:DevicePostureIntegration"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "configs".into(),
                    value: &configs_binding,
                },
                register_interface::ObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding,
                },
                register_interface::ObjectField {
                    name: "interval".into(),
                    value: &interval_binding,
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
        DevicePostureIntegrationResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configs"),
            ),
            identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identifier"),
            ),
            interval: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("interval"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
