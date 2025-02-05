/// Represents an instance of an Event Threat Detection custom module, including
/// its full module name, display name, enablement state, and last updated time.
/// You can create a custom module at the organization level only.
///
///
/// To get more information about OrganizationEventThreatDetectionCustomModule, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/security-center-management/rest/v1/organizations.locations.eventThreatDetectionCustomModules)
/// * How-to Guides
///     * [Overview of custom modules for Event Threat Detection](https://cloud.google.com/security-command-center/docs/custom-modules-etd-overview)
///
/// ## Example Usage
///
/// ### Scc Management Organization Event Threat Detection Custom Module
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:securitycenter:ManagementOrganizationEventThreatDetectionCustomModule
///     properties:
///       organization: '123456789'
///       location: global
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
/// OrganizationEventThreatDetectionCustomModule can be imported using any of these accepted formats:
///
/// * `organizations/{{organization}}/locations/{{location}}/eventThreatDetectionCustomModules/{{name}}`
///
/// * `{{organization}}/{{location}}/{{name}}`
///
/// When using the `pulumi import` command, OrganizationEventThreatDetectionCustomModule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/managementOrganizationEventThreatDetectionCustomModule:ManagementOrganizationEventThreatDetectionCustomModule default organizations/{{organization}}/locations/{{location}}/eventThreatDetectionCustomModules/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/managementOrganizationEventThreatDetectionCustomModule:ManagementOrganizationEventThreatDetectionCustomModule default {{organization}}/{{location}}/{{name}}
/// ```
///
pub mod management_organization_event_threat_detection_custom_module {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagementOrganizationEventThreatDetectionCustomModuleArgs {
        /// Config for the module. For the resident module, its config value is defined at this level.
        /// For the inherited module, its config value is inherited from the ancestor module.
        #[builder(into, default)]
        pub config: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The human readable name to be displayed for the module.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The state of enablement for the module at the given level of the hierarchy.
        /// Possible values are: `ENABLED`, `DISABLED`.
        #[builder(into, default)]
        pub enablement_state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Location ID of the parent organization. Only global is supported at the moment.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Numerical ID of the parent organization.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub organization: pulumi_wasm_rust::InputOrOutput<String>,
        /// Immutable. Type for the module. e.g. CONFIGURABLE_BAD_IP.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagementOrganizationEventThreatDetectionCustomModuleResult {
        /// Config for the module. For the resident module, its config value is defined at this level.
        /// For the inherited module, its config value is inherited from the ancestor module.
        pub config: pulumi_wasm_rust::Output<Option<String>>,
        /// The human readable name to be displayed for the module.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The state of enablement for the module at the given level of the hierarchy.
        /// Possible values are: `ENABLED`, `DISABLED`.
        pub enablement_state: pulumi_wasm_rust::Output<Option<String>>,
        /// The editor that last updated the custom module
        pub last_editor: pulumi_wasm_rust::Output<String>,
        /// Location ID of the parent organization. Only global is supported at the moment.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource name of the Event Threat Detection custom module.
        /// Its format is "organizations/{organization}/locations/{location}/eventThreatDetectionCustomModules/{eventThreatDetectionCustomModule}".
        pub name: pulumi_wasm_rust::Output<String>,
        /// Numerical ID of the parent organization.
        ///
        ///
        /// - - -
        pub organization: pulumi_wasm_rust::Output<String>,
        /// Immutable. Type for the module. e.g. CONFIGURABLE_BAD_IP.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The time at which the custom module was last updated.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
        /// up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ManagementOrganizationEventThreatDetectionCustomModuleArgs,
    ) -> ManagementOrganizationEventThreatDetectionCustomModuleResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let config_binding = args.config.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let enablement_state_binding = args
            .enablement_state
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let organization_binding = args.organization.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/managementOrganizationEventThreatDetectionCustomModule:ManagementOrganizationEventThreatDetectionCustomModule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "enablementState".into(),
                    value: &enablement_state_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "organization".into(),
                    value: &organization_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ManagementOrganizationEventThreatDetectionCustomModuleResult {
            config: pulumi_wasm_rust::__private::into_domain(o.extract_field("config")),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            enablement_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enablementState"),
            ),
            last_editor: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastEditor"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            organization: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("organization"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
