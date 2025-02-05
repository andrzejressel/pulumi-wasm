/// Manages an Alert Management Prometheus Rule Group.
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
///   exampleActionGroup:
///     type: azure:monitoring:ActionGroup
///     name: example
///     properties:
///       name: example-mag
///       resourceGroupName: ${example.name}
///       shortName: testag
///   exampleWorkspace:
///     type: azure:monitoring:Workspace
///     name: example
///     properties:
///       name: example-amw
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   exampleKubernetesCluster:
///     type: azure:containerservice:KubernetesCluster
///     name: example
///     properties:
///       name: example-cluster
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       dnsPrefix: example-aks
///       defaultNodePool:
///         name: default
///         nodeCount: 1
///         vmSize: Standard_DS2_v2
///         hostEncryptionEnabled: true
///       identity:
///         type: SystemAssigned
///   exampleAlertPrometheusRuleGroup:
///     type: azure:monitoring:AlertPrometheusRuleGroup
///     name: example
///     properties:
///       name: example-amprg
///       location: West Europe
///       resourceGroupName: ${example.name}
///       clusterName: ${exampleKubernetesCluster.name}
///       description: This is the description of the following rule group
///       ruleGroupEnabled: false
///       interval: PT1M
///       scopes:
///         - ${exampleWorkspace.id}
///       rules:
///         - enabled: false
///           expression: |
///             histogram_quantile(0.99, sum(rate(jobs_duration_seconds_bucket{service="billing-processing"}[5m])) by (job_type))
///           record: job_type:billing_jobs_duration_seconds:99p5m
///           labels:
///             team: prod
///         - alert: Billing_Processing_Very_Slow
///           enabled: true
///           expression: |
///             histogram_quantile(0.99, sum(rate(jobs_duration_seconds_bucket{service="billing-processing"}[5m])) by (job_type))
///           for: PT5M
///           severity: 2
///           actions:
///             - actionGroupId: ${exampleActionGroup.id}
///           alertResolution:
///             autoResolved: true
///             timeToResolve: PT10M
///           annotations:
///             annotationName: annotationValue
///           labels:
///             team: prod
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Alert Management Prometheus Rule Group can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/alertPrometheusRuleGroup:AlertPrometheusRuleGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.AlertsManagement/prometheusRuleGroups/ruleGroup1
/// ```
///
pub mod alert_prometheus_rule_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertPrometheusRuleGroupArgs {
        /// Specifies the name of the Managed Kubernetes Cluster.
        #[builder(into, default)]
        pub cluster_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The description of the Alert Management Prometheus Rule Group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the interval in which to run the Alert Management Prometheus Rule Group represented in ISO 8601 duration format. Possible values are between `PT1M` and `PT15M`.
        #[builder(into, default)]
        pub interval: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the Azure Region where the Alert Management Prometheus Rule Group should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Alert Management Prometheus Rule Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group where the Alert Management Prometheus Rule Group should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Is this Alert Management Prometheus Rule Group enabled? Possible values are `true` and `false`.
        #[builder(into, default)]
        pub rule_group_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// One or more `rule` blocks as defined below.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::monitoring::AlertPrometheusRuleGroupRule>,
        >,
        /// Specifies the resource ID of the Azure Monitor Workspace.
        #[builder(into)]
        pub scopes: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// A mapping of tags to assign to the Alert Management Prometheus Rule Group.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AlertPrometheusRuleGroupResult {
        /// Specifies the name of the Managed Kubernetes Cluster.
        pub cluster_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of the Alert Management Prometheus Rule Group.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the interval in which to run the Alert Management Prometheus Rule Group represented in ISO 8601 duration format. Possible values are between `PT1M` and `PT15M`.
        pub interval: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Alert Management Prometheus Rule Group should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Alert Management Prometheus Rule Group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Alert Management Prometheus Rule Group should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Is this Alert Management Prometheus Rule Group enabled? Possible values are `true` and `false`.
        pub rule_group_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `rule` blocks as defined below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::monitoring::AlertPrometheusRuleGroupRule>,
        >,
        /// Specifies the resource ID of the Azure Monitor Workspace.
        pub scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// A mapping of tags to assign to the Alert Management Prometheus Rule Group.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AlertPrometheusRuleGroupArgs,
    ) -> AlertPrometheusRuleGroupResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let interval_binding = args.interval.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let rule_group_enabled_binding = args
            .rule_group_enabled
            .get_output(context)
            .get_inner();
        let rules_binding = args.rules.get_output(context).get_inner();
        let scopes_binding = args.scopes.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/alertPrometheusRuleGroup:AlertPrometheusRuleGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "interval".into(),
                    value: &interval_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "ruleGroupEnabled".into(),
                    value: &rule_group_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        AlertPrometheusRuleGroupResult {
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            interval: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("interval"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            rule_group_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ruleGroupEnabled"),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(o.extract_field("rules")),
            scopes: pulumi_wasm_rust::__private::into_domain(o.extract_field("scopes")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
