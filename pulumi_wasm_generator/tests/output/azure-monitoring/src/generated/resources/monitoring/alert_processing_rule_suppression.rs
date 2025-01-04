/// Manages an Alert Processing Rule which suppress notifications.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAlertProcessingRuleSuppression:
///     type: azure:monitoring:AlertProcessingRuleSuppression
///     name: example
///     properties:
///       name: example
///       resourceGroupName: example
///       scopes:
///         - ${example.id}
///       condition:
///         targetResourceType:
///           operator: Equals
///           values:
///             - Microsoft.Compute/VirtualMachines
///         severity:
///           operator: Equals
///           values:
///             - Sev0
///             - Sev1
///             - Sev2
///       schedule:
///         effectiveFrom: 2022-01-01T01:02:03
///         effectiveUntil: 2022-02-02T01:02:03
///         timeZone: Pacific Standard Time
///         recurrence:
///           dailies:
///             - startTime: 17:00:00
///               endTime: 09:00:00
///           weeklies:
///             - daysOfWeeks:
///                 - Saturday
///                 - Sunday
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Alert Processing Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/alertProcessingRuleSuppression:AlertProcessingRuleSuppression example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.AlertsManagement/actionRules/actionRule1
/// ```
///
pub mod alert_processing_rule_suppression {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertProcessingRuleSuppressionArgs {
        /// A `condition` block as defined below.
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<
                super::super::types::monitoring::AlertProcessingRuleSuppressionCondition,
            >,
        >,
        /// Specifies a description for the Alert Processing Rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Alert Processing Rule be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Alert Processing Rule. Changing this forces a new Alert Processing Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Alert Processing Rule should exist. Changing this forces a new Alert Processing Rule to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `schedule` block as defined below.
        #[builder(into, default)]
        pub schedule: pulumi_wasm_rust::Output<
            Option<
                super::super::types::monitoring::AlertProcessingRuleSuppressionSchedule,
            >,
        >,
        /// A list of resource IDs which will be the target of Alert Processing Rule.
        #[builder(into)]
        pub scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// A mapping of tags which should be assigned to the Alert Processing Rule.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AlertProcessingRuleSuppressionResult {
        /// A `condition` block as defined below.
        pub condition: pulumi_wasm_rust::Output<
            Option<
                super::super::types::monitoring::AlertProcessingRuleSuppressionCondition,
            >,
        >,
        /// Specifies a description for the Alert Processing Rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Alert Processing Rule be enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Alert Processing Rule. Changing this forces a new Alert Processing Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Alert Processing Rule should exist. Changing this forces a new Alert Processing Rule to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `schedule` block as defined below.
        pub schedule: pulumi_wasm_rust::Output<
            Option<
                super::super::types::monitoring::AlertProcessingRuleSuppressionSchedule,
            >,
        >,
        /// A list of resource IDs which will be the target of Alert Processing Rule.
        pub scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// A mapping of tags which should be assigned to the Alert Processing Rule.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AlertProcessingRuleSuppressionArgs,
    ) -> AlertProcessingRuleSuppressionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let description_binding = args.description.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let schedule_binding = args.schedule.get_inner();
        let scopes_binding = args.scopes.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/alertProcessingRuleSuppression:AlertProcessingRuleSuppression"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
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
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "schedule".into(),
                },
                register_interface::ResultField {
                    name: "scopes".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AlertProcessingRuleSuppressionResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedule").unwrap(),
            ),
            scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopes").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
