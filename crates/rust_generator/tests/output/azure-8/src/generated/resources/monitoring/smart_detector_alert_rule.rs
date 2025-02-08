/// Manages an Monitor Smart Detector Alert Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleActionGroup = action_group::create(
///         "exampleActionGroup",
///         ActionGroupArgs::builder()
///             .name("example-action-group")
///             .resource_group_name("${example.name}")
///             .short_name("example")
///             .build_struct(),
///     );
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("web")
///             .location("${example.location}")
///             .name("example-appinsights")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSmartDetectorAlertRule = smart_detector_alert_rule::create(
///         "exampleSmartDetectorAlertRule",
///         SmartDetectorAlertRuleArgs::builder()
///             .action_group(
///                 SmartDetectorAlertRuleActionGroup::builder()
///                     .ids(vec!["${exampleActionGroup.id}",])
///                     .build_struct(),
///             )
///             .detector_type("FailureAnomaliesDetector")
///             .frequency("PT1M")
///             .name("example-smart-detector-alert-rule")
///             .resource_group_name("${example.name}")
///             .scope_resource_ids(vec!["${exampleInsights.id}",])
///             .severity("Sev0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Monitor Smart Detector Alert Rule can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/smartDetectorAlertRule:SmartDetectorAlertRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.AlertsManagement/smartDetectorAlertRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod smart_detector_alert_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SmartDetectorAlertRuleArgs {
        /// An `action_group` block as defined below.
        #[builder(into)]
        pub action_group: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::monitoring::SmartDetectorAlertRuleActionGroup,
        >,
        /// Specifies a description for the Smart Detector Alert Rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Built-In Smart Detector type that this alert rule will use. Currently the only possible values are `FailureAnomaliesDetector`, `RequestPerformanceDegradationDetector`, `DependencyPerformanceDegradationDetector`, `ExceptionVolumeChangedDetector`, `TraceSeverityDetector`, `MemoryLeakDetector`.
        #[builder(into)]
        pub detector_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is the Smart Detector Alert Rule enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the frequency of this Smart Detector Alert Rule in ISO8601 format.
        #[builder(into)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Monitor Smart Detector Alert Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the resource group in which the Monitor Smart Detector Alert Rule should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the scopes of this Smart Detector Alert Rule.
        #[builder(into)]
        pub scope_resource_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Specifies the severity of this Smart Detector Alert Rule. Possible values are `Sev0`, `Sev1`, `Sev2`, `Sev3` or `Sev4`.
        #[builder(into)]
        pub severity: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the duration (in ISO8601 format) to wait before notifying on the alert rule again.
        #[builder(into, default)]
        pub throttling_duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SmartDetectorAlertRuleResult {
        /// An `action_group` block as defined below.
        pub action_group: pulumi_gestalt_rust::Output<
            super::super::types::monitoring::SmartDetectorAlertRuleActionGroup,
        >,
        /// Specifies a description for the Smart Detector Alert Rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Built-In Smart Detector type that this alert rule will use. Currently the only possible values are `FailureAnomaliesDetector`, `RequestPerformanceDegradationDetector`, `DependencyPerformanceDegradationDetector`, `ExceptionVolumeChangedDetector`, `TraceSeverityDetector`, `MemoryLeakDetector`.
        pub detector_type: pulumi_gestalt_rust::Output<String>,
        /// Is the Smart Detector Alert Rule enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the frequency of this Smart Detector Alert Rule in ISO8601 format.
        pub frequency: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Monitor Smart Detector Alert Rule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the resource group in which the Monitor Smart Detector Alert Rule should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the scopes of this Smart Detector Alert Rule.
        pub scope_resource_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the severity of this Smart Detector Alert Rule. Possible values are `Sev0`, `Sev1`, `Sev2`, `Sev3` or `Sev4`.
        pub severity: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the duration (in ISO8601 format) to wait before notifying on the alert rule again.
        pub throttling_duration: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SmartDetectorAlertRuleArgs,
    ) -> SmartDetectorAlertRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_group_binding = args.action_group.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let detector_type_binding = args.detector_type.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let frequency_binding = args.frequency.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let scope_resource_ids_binding = args
            .scope_resource_ids
            .get_output(context)
            .get_inner();
        let severity_binding = args.severity.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let throttling_duration_binding = args
            .throttling_duration
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/smartDetectorAlertRule:SmartDetectorAlertRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actionGroup".into(),
                    value: &action_group_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "detectorType".into(),
                    value: &detector_type_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "frequency".into(),
                    value: &frequency_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "scopeResourceIds".into(),
                    value: &scope_resource_ids_binding,
                },
                register_interface::ObjectField {
                    name: "severity".into(),
                    value: &severity_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "throttlingDuration".into(),
                    value: &throttling_duration_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SmartDetectorAlertRuleResult {
            action_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("actionGroup"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            detector_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("detectorType"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            frequency: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frequency"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            scope_resource_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scopeResourceIds"),
            ),
            severity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("severity"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            throttling_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("throttlingDuration"),
            ),
        }
    }
}
