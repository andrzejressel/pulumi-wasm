/// Provides a Cloudflare Device Posture Integration resource. Device
/// posture integrations configure third-party data providers for device
/// posture rules.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod device_posture_integration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DevicePostureIntegrationArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The device posture integration's connection authorization parameters.
        #[builder(into, default)]
        pub configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::DevicePostureIntegrationConfig>>,
        >,
        #[builder(into, default)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`.
        #[builder(into, default)]
        pub interval: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the device posture integration.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`, `sentinelone_s2s`, `tanium_s2s`, `custom_s2s`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DevicePostureIntegrationResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The device posture integration's connection authorization parameters.
        pub configs: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::DevicePostureIntegrationConfig>>,
        >,
        pub identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`.
        pub interval: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the device posture integration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`, `sentinelone_s2s`, `tanium_s2s`, `custom_s2s`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DevicePostureIntegrationArgs,
    ) -> DevicePostureIntegrationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let configs_binding = args.configs.get_output(context);
        let identifier_binding = args.identifier.get_output(context);
        let interval_binding = args.interval.get_output(context);
        let name_binding = args.name.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/devicePostureIntegration:DevicePostureIntegration"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configs".into(),
                    value: configs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identifier".into(),
                    value: identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interval".into(),
                    value: interval_binding.get_id(),
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
        DevicePostureIntegrationResult {
            account_id: o.get_field("accountId"),
            configs: o.get_field("configs"),
            identifier: o.get_field("identifier"),
            interval: o.get_field("interval"),
            name: o.get_field("name"),
            type_: o.get_field("type"),
        }
    }
}
