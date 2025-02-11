/// Represents an instance of an Event Threat Detection custom module, including
/// its full module name, display name, enablement state, andlast updated time.
/// You can create a custom module at the organization level only.
///
///
/// To get more information about EventThreatDetectionCustomModule, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v1/organizations.eventThreatDetectionSettings.customModules)
/// * How-to Guides
///     * [Overview of custom modules for Event Threat Detection](https://cloud.google.com/security-command-center/docs/custom-modules-etd-overview)
///
/// ## Example Usage
///
/// ### Scc Event Threat Detection Custom Module
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:securitycenter:EventThreatDetectionCustomModule
///     properties:
///       organization: '123456789'
///       displayName: basic_custom_module
///       enablementState: ENABLED
///       type: CONFIGURABLE_BAD_IP
///       description: My Event Threat Detection Custom Module
///       config:
///         fn::toJSON:
///           metadata:
///             severity: LOW
///             description: Flagged by Forcepoint as malicious
///             recommendation: Contact the owner of the relevant project.
///           ips:
///             - 192.0.2.1
///             - 192.0.2.0/24
/// ```
///
/// ## Import
///
/// EventThreatDetectionCustomModule can be imported using any of these accepted formats:
///
/// * `organizations/{{organization}}/eventThreatDetectionSettings/customModules/{{name}}`
///
/// * `{{organization}}/{{name}}`
///
/// When using the `pulumi import` command, EventThreatDetectionCustomModule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/eventThreatDetectionCustomModule:EventThreatDetectionCustomModule default organizations/{{organization}}/eventThreatDetectionSettings/customModules/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/eventThreatDetectionCustomModule:EventThreatDetectionCustomModule default {{organization}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_threat_detection_custom_module {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventThreatDetectionCustomModuleArgs {
        /// Config for the module. For the resident module, its config value is defined at this level.
        /// For the inherited module, its config value is inherited from the ancestor module.
        #[builder(into)]
        pub config: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The human readable name to be displayed for the module.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The state of enablement for the module at the given level of the hierarchy.
        /// Possible values are: `ENABLED`, `DISABLED`.
        #[builder(into)]
        pub enablement_state: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Numerical ID of the parent organization.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub organization: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Immutable. Type for the module. e.g. CONFIGURABLE_BAD_IP.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EventThreatDetectionCustomModuleResult {
        /// Config for the module. For the resident module, its config value is defined at this level.
        /// For the inherited module, its config value is inherited from the ancestor module.
        pub config: pulumi_gestalt_rust::Output<String>,
        /// The human readable name to be displayed for the module.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The state of enablement for the module at the given level of the hierarchy.
        /// Possible values are: `ENABLED`, `DISABLED`.
        pub enablement_state: pulumi_gestalt_rust::Output<String>,
        /// The editor that last updated the custom module
        pub last_editor: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the Event Threat Detection custom module.
        /// Its format is "organizations/{organization}/eventThreatDetectionSettings/customModules/{module}".
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Numerical ID of the parent organization.
        ///
        ///
        /// - - -
        pub organization: pulumi_gestalt_rust::Output<String>,
        /// Immutable. Type for the module. e.g. CONFIGURABLE_BAD_IP.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The time at which the custom module was last updated.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
        /// up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventThreatDetectionCustomModuleArgs,
    ) -> EventThreatDetectionCustomModuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let config_binding = args.config.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enablement_state_binding = args.enablement_state.get_output(context);
        let organization_binding = args.organization.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:securitycenter/eventThreatDetectionCustomModule:EventThreatDetectionCustomModule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "config".into(),
                    value: &config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enablementState".into(),
                    value: &enablement_state_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organization".into(),
                    value: &organization_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventThreatDetectionCustomModuleResult {
            config: o.get_field("config"),
            display_name: o.get_field("displayName"),
            enablement_state: o.get_field("enablementState"),
            last_editor: o.get_field("lastEditor"),
            name: o.get_field("name"),
            organization: o.get_field("organization"),
            type_: o.get_field("type"),
            update_time: o.get_field("updateTime"),
        }
    }
}
