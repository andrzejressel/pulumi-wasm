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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alert_prometheus_rule_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertPrometheusRuleGroupArgs {
        /// Specifies the name of the Managed Kubernetes Cluster.
        #[builder(into, default)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of the Alert Management Prometheus Rule Group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the interval in which to run the Alert Management Prometheus Rule Group represented in ISO 8601 duration format. Possible values are between `PT1M` and `PT15M`.
        #[builder(into, default)]
        pub interval: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Azure Region where the Alert Management Prometheus Rule Group should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Alert Management Prometheus Rule Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group where the Alert Management Prometheus Rule Group should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is this Alert Management Prometheus Rule Group enabled? Possible values are `true` and `false`.
        #[builder(into, default)]
        pub rule_group_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// One or more `rule` blocks as defined below.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::monitoring::AlertPrometheusRuleGroupRule>,
        >,
        /// Specifies the resource ID of the Azure Monitor Workspace.
        #[builder(into)]
        pub scopes: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A mapping of tags to assign to the Alert Management Prometheus Rule Group.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AlertPrometheusRuleGroupResult {
        /// Specifies the name of the Managed Kubernetes Cluster.
        pub cluster_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of the Alert Management Prometheus Rule Group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the interval in which to run the Alert Management Prometheus Rule Group represented in ISO 8601 duration format. Possible values are between `PT1M` and `PT15M`.
        pub interval: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Alert Management Prometheus Rule Group should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Alert Management Prometheus Rule Group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Alert Management Prometheus Rule Group should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Is this Alert Management Prometheus Rule Group enabled? Possible values are `true` and `false`.
        pub rule_group_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// One or more `rule` blocks as defined below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::monitoring::AlertPrometheusRuleGroupRule>,
        >,
        /// Specifies the resource ID of the Azure Monitor Workspace.
        pub scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A mapping of tags to assign to the Alert Management Prometheus Rule Group.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AlertPrometheusRuleGroupArgs,
    ) -> AlertPrometheusRuleGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_name_binding = args.cluster_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let interval_binding = args.interval.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let rule_group_enabled_binding = args.rule_group_enabled.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let scopes_binding = args.scopes.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:monitoring/alertPrometheusRuleGroup:AlertPrometheusRuleGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interval".into(),
                    value: &interval_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleGroupEnabled".into(),
                    value: &rule_group_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: &rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AlertPrometheusRuleGroupResult {
            cluster_name: o.get_field("clusterName"),
            description: o.get_field("description"),
            interval: o.get_field("interval"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            rule_group_enabled: o.get_field("ruleGroupEnabled"),
            rules: o.get_field("rules"),
            scopes: o.get_field("scopes"),
            tags: o.get_field("tags"),
        }
    }
}
