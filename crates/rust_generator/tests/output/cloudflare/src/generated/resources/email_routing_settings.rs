/// Provides a resource for managing Email Routing settings.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   myZone:
///     type: cloudflare:EmailRoutingSettings
///     name: my_zone
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       enabled: 'true'
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod email_routing_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailRoutingSettingsArgs {
        /// State of the zone settings for Email Routing. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Flag to check if the user skipped the configuration wizard.
        #[builder(into, default)]
        pub skip_wizard: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EmailRoutingSettingsResult {
        /// The date and time the settings have been created.
        pub created: pulumi_gestalt_rust::Output<String>,
        /// State of the zone settings for Email Routing. **Modifying this attribute will force creation of a new resource.**
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The date and time the settings have been modified.
        pub modified: pulumi_gestalt_rust::Output<String>,
        /// Domain of your zone.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Flag to check if the user skipped the configuration wizard.
        pub skip_wizard: pulumi_gestalt_rust::Output<bool>,
        /// Show the state of your account, and the type or configuration error.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Email Routing settings identifier.
        pub tag: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailRoutingSettingsArgs,
    ) -> EmailRoutingSettingsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_binding = args.enabled.get_output(context);
        let skip_wizard_binding = args.skip_wizard.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingSettings:EmailRoutingSettings".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipWizard".into(),
                    value: skip_wizard_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EmailRoutingSettingsResult {
            created: o.get_field("created"),
            enabled: o.get_field("enabled"),
            modified: o.get_field("modified"),
            name: o.get_field("name"),
            skip_wizard: o.get_field("skipWizard"),
            status: o.get_field("status"),
            tag: o.get_field("tag"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
