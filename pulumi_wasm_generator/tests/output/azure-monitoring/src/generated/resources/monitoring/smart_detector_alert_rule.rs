/// Manages an Monitor Smart Detector Alert Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod smart_detector_alert_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SmartDetectorAlertRuleArgs {
        /// An `action_group` block as defined below.
        #[builder(into)]
        pub action_group: pulumi_wasm_rust::Output<
            super::super::types::monitoring::SmartDetectorAlertRuleActionGroup,
        >,
        /// Specifies a description for the Smart Detector Alert Rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Built-In Smart Detector type that this alert rule will use. Currently the only possible values are `FailureAnomaliesDetector`, `RequestPerformanceDegradationDetector`, `DependencyPerformanceDegradationDetector`, `ExceptionVolumeChangedDetector`, `TraceSeverityDetector`, `MemoryLeakDetector`.
        #[builder(into)]
        pub detector_type: pulumi_wasm_rust::Output<String>,
        /// Is the Smart Detector Alert Rule enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the frequency of this Smart Detector Alert Rule in ISO8601 format.
        #[builder(into)]
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Monitor Smart Detector Alert Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the resource group in which the Monitor Smart Detector Alert Rule should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the scopes of this Smart Detector Alert Rule.
        #[builder(into)]
        pub scope_resource_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the severity of this Smart Detector Alert Rule. Possible values are `Sev0`, `Sev1`, `Sev2`, `Sev3` or `Sev4`.
        #[builder(into)]
        pub severity: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the duration (in ISO8601 format) to wait before notifying on the alert rule again.
        #[builder(into, default)]
        pub throttling_duration: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SmartDetectorAlertRuleResult {
        /// An `action_group` block as defined below.
        pub action_group: pulumi_wasm_rust::Output<
            super::super::types::monitoring::SmartDetectorAlertRuleActionGroup,
        >,
        /// Specifies a description for the Smart Detector Alert Rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Built-In Smart Detector type that this alert rule will use. Currently the only possible values are `FailureAnomaliesDetector`, `RequestPerformanceDegradationDetector`, `DependencyPerformanceDegradationDetector`, `ExceptionVolumeChangedDetector`, `TraceSeverityDetector`, `MemoryLeakDetector`.
        pub detector_type: pulumi_wasm_rust::Output<String>,
        /// Is the Smart Detector Alert Rule enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the frequency of this Smart Detector Alert Rule in ISO8601 format.
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Monitor Smart Detector Alert Rule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group in which the Monitor Smart Detector Alert Rule should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the scopes of this Smart Detector Alert Rule.
        pub scope_resource_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the severity of this Smart Detector Alert Rule. Possible values are `Sev0`, `Sev1`, `Sev2`, `Sev3` or `Sev4`.
        pub severity: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the duration (in ISO8601 format) to wait before notifying on the alert rule again.
        pub throttling_duration: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SmartDetectorAlertRuleArgs,
    ) -> SmartDetectorAlertRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_group_binding = args.action_group.get_inner();
        let description_binding = args.description.get_inner();
        let detector_type_binding = args.detector_type.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let frequency_binding = args.frequency.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let scope_resource_ids_binding = args.scope_resource_ids.get_inner();
        let severity_binding = args.severity.get_inner();
        let tags_binding = args.tags.get_inner();
        let throttling_duration_binding = args.throttling_duration.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/smartDetectorAlertRule:SmartDetectorAlertRule"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "actionGroup".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "detectorType".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "frequency".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scopeResourceIds".into(),
                },
                register_interface::ResultField {
                    name: "severity".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "throttlingDuration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SmartDetectorAlertRuleResult {
            action_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionGroup").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            detector_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("detectorType").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            frequency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frequency").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            scope_resource_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopeResourceIds").unwrap(),
            ),
            severity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("severity").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            throttling_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("throttlingDuration").unwrap(),
            ),
        }
    }
}