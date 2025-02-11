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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod monitor_tag_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitorTagRuleArgs {
        /// The Datadog Monitor Id which should be used for this Datadog Monitor Tag Rule. Changing this forces a new Datadog Monitor Tag Rule to be created.
        #[builder(into)]
        pub datadog_monitor_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `log` block as defined below.
        #[builder(into, default)]
        pub logs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datadog::MonitorTagRuleLog>>,
        >,
        /// A `metric` block as defined below.
        #[builder(into, default)]
        pub metrics: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datadog::MonitorTagRuleMetric>>,
        >,
        /// The name of the Tag Rules configuration. The allowed value is `default`. Defaults to `default`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MonitorTagRuleResult {
        /// The Datadog Monitor Id which should be used for this Datadog Monitor Tag Rule. Changing this forces a new Datadog Monitor Tag Rule to be created.
        pub datadog_monitor_id: pulumi_gestalt_rust::Output<String>,
        /// A `log` block as defined below.
        pub logs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datadog::MonitorTagRuleLog>>,
        >,
        /// A `metric` block as defined below.
        pub metrics: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datadog::MonitorTagRuleMetric>>,
        >,
        /// The name of the Tag Rules configuration. The allowed value is `default`. Defaults to `default`.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MonitorTagRuleArgs,
    ) -> MonitorTagRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let datadog_monitor_id_binding = args.datadog_monitor_id.get_output(context);
        let logs_binding = args.logs.get_output(context);
        let metrics_binding = args.metrics.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datadog/monitorTagRule:MonitorTagRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "datadogMonitorId".into(),
                    value: &datadog_monitor_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logs".into(),
                    value: &logs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metrics".into(),
                    value: &metrics_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MonitorTagRuleResult {
            datadog_monitor_id: o.get_field("datadogMonitorId"),
            logs: o.get_field("logs"),
            metrics: o.get_field("metrics"),
            name: o.get_field("name"),
        }
    }
}
