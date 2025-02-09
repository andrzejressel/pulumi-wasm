/// Provides a resource which customizes Cloudflare zone settings.
///
/// > You **should not** use this resource to manage every zone setting. This
///   resource is only intended to override those which you do not want the default.
///   Attempting to manage all settings will result in problems with the resource
///   applying in a consistent manner.
///
/// ## Plan-Dependent Settings
///
/// Note that some settings are only available on certain plans. Setting an argument
/// for a feature that is not available on the plan configured for the zone will
/// result in an error:
///
/// ```sh
/// Error: invalid zone setting "\<argument\>" (value: \<value\>) found - cannot be set as it is read only
/// ```
///
/// This is true even when setting the argument to its default value. These values
/// should either be omitted or set to `null` for zones with plans that don't
/// support the feature. See the [plan feature matrices](https://www.cloudflare.com/plans/) for details on
/// feature support by plan.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = zone_settings_override::create(
///         "test",
///         ZoneSettingsOverrideArgs::builder()
///             .settings(
///                 ZoneSettingsOverrideSettings::builder()
///                     .automaticHttpsRewrites("on")
///                     .brotli("on")
///                     .challengeTtl(2700)
///                     .minify(
///                         ZoneSettingsOverrideSettingsMinify::builder()
///                             .css("on")
///                             .html("off")
///                             .js("off")
///                             .build_struct(),
///                     )
///                     .mirage("on")
///                     .opportunisticEncryption("on")
///                     .securityHeader(
///                         ZoneSettingsOverrideSettingsSecurityHeader::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .securityLevel("high")
///                     .waf("on")
///                     .build_struct(),
///             )
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zone_settings_override {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneSettingsOverrideArgs {
        #[builder(into, default)]
        pub settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZoneSettingsOverrideSettings>,
        >,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneSettingsOverrideResult {
        pub initial_settings: pulumi_gestalt_rust::Output<
            Vec<super::types::ZoneSettingsOverrideInitialSetting>,
        >,
        pub initial_settings_read_at: pulumi_gestalt_rust::Output<String>,
        pub readonly_settings: pulumi_gestalt_rust::Output<Vec<String>>,
        pub settings: pulumi_gestalt_rust::Output<
            super::types::ZoneSettingsOverrideSettings,
        >,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
        pub zone_status: pulumi_gestalt_rust::Output<String>,
        pub zone_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZoneSettingsOverrideArgs,
    ) -> ZoneSettingsOverrideResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let settings_binding = args.settings.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zoneSettingsOverride:ZoneSettingsOverride".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settings".into(),
                    value: settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZoneSettingsOverrideResult {
            initial_settings: o.get_field("initialSettings"),
            initial_settings_read_at: o.get_field("initialSettingsReadAt"),
            readonly_settings: o.get_field("readonlySettings"),
            settings: o.get_field("settings"),
            zone_id: o.get_field("zoneId"),
            zone_status: o.get_field("zoneStatus"),
            zone_type: o.get_field("zoneType"),
        }
    }
}
