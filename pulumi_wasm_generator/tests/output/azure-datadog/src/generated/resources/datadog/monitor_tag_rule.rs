/// Manages TagRules on the datadog Monitor.
///
/// ## Example Usage
///
/// ### Adding TagRules on monitor
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-datadog
///       location: West US 2
///   exampleMonitor:
///     type: azure:datadog:Monitor
///     name: example
///     properties:
///       name: example-monitor
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       datadogOrganization:
///         apiKey: XXXX
///         applicationKey: XXXX
///       user:
///         name: Example
///         email: abc@xyz.com
///       skuName: Linked
///       identity:
///         type: SystemAssigned
///   exampleMonitorTagRule:
///     type: azure:datadog:MonitorTagRule
///     name: example
///     properties:
///       datadogMonitorId: ${exampleMonitor.id}
///       logs:
///         - subscriptionLogEnabled: true
///       metrics:
///         - filters:
///             - name: Test
///               value: Logs
///               action: Include
/// ```
///
/// ## Import
///
/// Tag Rules on the Datadog Monitor can be imported using the `tag rule resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datadog/monitorTagRule:MonitorTagRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Datadog/monitors/monitor1/tagRules/default
/// ```
///
pub mod monitor_tag_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitorTagRuleArgs {
        /// The Datadog Monitor Id which should be used for this Datadog Monitor Tag Rule. Changing this forces a new Datadog Monitor Tag Rule to be created.
        #[builder(into)]
        pub datadog_monitor_id: pulumi_wasm_rust::Output<String>,
        /// A `log` block as defined below.
        #[builder(into, default)]
        pub logs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datadog::MonitorTagRuleLog>>,
        >,
        /// A `metric` block as defined below.
        #[builder(into, default)]
        pub metrics: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datadog::MonitorTagRuleMetric>>,
        >,
        /// The name of the Tag Rules configuration. The allowed value is `default`. Defaults to `default`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MonitorTagRuleResult {
        /// The Datadog Monitor Id which should be used for this Datadog Monitor Tag Rule. Changing this forces a new Datadog Monitor Tag Rule to be created.
        pub datadog_monitor_id: pulumi_wasm_rust::Output<String>,
        /// A `log` block as defined below.
        pub logs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datadog::MonitorTagRuleLog>>,
        >,
        /// A `metric` block as defined below.
        pub metrics: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datadog::MonitorTagRuleMetric>>,
        >,
        /// The name of the Tag Rules configuration. The allowed value is `default`. Defaults to `default`.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MonitorTagRuleArgs) -> MonitorTagRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let datadog_monitor_id_binding = args.datadog_monitor_id.get_inner();
        let logs_binding = args.logs.get_inner();
        let metrics_binding = args.metrics.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datadog/monitorTagRule:MonitorTagRule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "datadogMonitorId".into(),
                    value: &datadog_monitor_id_binding,
                },
                register_interface::ObjectField {
                    name: "logs".into(),
                    value: &logs_binding,
                },
                register_interface::ObjectField {
                    name: "metrics".into(),
                    value: &metrics_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "datadogMonitorId".into(),
                },
                register_interface::ResultField {
                    name: "logs".into(),
                },
                register_interface::ResultField {
                    name: "metrics".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MonitorTagRuleResult {
            datadog_monitor_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("datadogMonitorId").unwrap(),
            ),
            logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logs").unwrap(),
            ),
            metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metrics").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}