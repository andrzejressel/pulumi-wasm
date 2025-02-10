/// Manages a Dashboard Grafana.
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
///   exampleGrafana:
///     type: azure:dashboard:Grafana
///     name: example
///     properties:
///       name: example-dg
///       resourceGroupName: ${example.name}
///       location: West Europe
///       grafanaMajorVersion: 10
///       apiKeyEnabled: true
///       deterministicOutboundIpEnabled: true
///       publicNetworkAccessEnabled: false
///       identity:
///         type: SystemAssigned
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Dashboard Grafana can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dashboard/grafana:Grafana example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Dashboard/grafana/workspace1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod grafana {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GrafanaArgs {
        /// Whether to enable the api key setting of the Grafana instance. Defaults to `false`.
        #[builder(into, default)]
        pub api_key_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Scope for dns deterministic name hash calculation. The only possible value is `TenantReuse`. Defaults to `TenantReuse`.
        #[builder(into, default)]
        pub auto_generated_domain_name_label_scope: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A `azure_monitor_workspace_integrations` block as defined below.
        #[builder(into, default)]
        pub azure_monitor_workspace_integrations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::dashboard::GrafanaAzureMonitorWorkspaceIntegration,
                >,
            >,
        >,
        /// Whether to enable the Grafana instance to use deterministic outbound IPs. Defaults to `false`.
        #[builder(into, default)]
        pub deterministic_outbound_ip_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Which major version of Grafana to deploy. Possible values are `9`, `10`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub grafana_major_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `identity` block as defined below. Changing this forces a new Dashboard Grafana to be created.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dashboard::GrafanaIdentity>,
        >,
        /// Specifies the Azure Region where the Dashboard Grafana should exist. Changing this forces a new Dashboard Grafana to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Dashboard Grafana. Changing this forces a new Dashboard Grafana to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to enable traffic over the public interface. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the name of the Resource Group where the Dashboard Grafana should exist. Changing this forces a new Dashboard Grafana to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the SKU used for the Grafana instance. Possible values are `Standard` and `Essential`. Defaults to `Standard`. Changing this forces a new Dashboard Grafana to be created.
        #[builder(into, default)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `smtp` block as defined below.
        #[builder(into, default)]
        pub smtp: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dashboard::GrafanaSmtp>,
        >,
        /// A mapping of tags which should be assigned to the Dashboard Grafana.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to enable the zone redundancy setting of the Grafana instance. Defaults to `false`. Changing this forces a new Dashboard Grafana to be created.
        #[builder(into, default)]
        pub zone_redundancy_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GrafanaResult {
        /// Whether to enable the api key setting of the Grafana instance. Defaults to `false`.
        pub api_key_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Scope for dns deterministic name hash calculation. The only possible value is `TenantReuse`. Defaults to `TenantReuse`.
        pub auto_generated_domain_name_label_scope: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// A `azure_monitor_workspace_integrations` block as defined below.
        pub azure_monitor_workspace_integrations: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::dashboard::GrafanaAzureMonitorWorkspaceIntegration,
                >,
            >,
        >,
        /// Whether to enable the Grafana instance to use deterministic outbound IPs. Defaults to `false`.
        pub deterministic_outbound_ip_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The endpoint of the Grafana instance.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Which major version of Grafana to deploy. Possible values are `9`, `10`. Changing this forces a new resource to be created.
        pub grafana_major_version: pulumi_gestalt_rust::Output<String>,
        /// The full Grafana software semantic version deployed.
        pub grafana_version: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below. Changing this forces a new Dashboard Grafana to be created.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::dashboard::GrafanaIdentity>,
        >,
        /// Specifies the Azure Region where the Dashboard Grafana should exist. Changing this forces a new Dashboard Grafana to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Dashboard Grafana. Changing this forces a new Dashboard Grafana to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of outbound IPs if deterministicOutboundIP is enabled.
        pub outbound_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether to enable traffic over the public interface. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Resource Group where the Dashboard Grafana should exist. Changing this forces a new Dashboard Grafana to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the SKU used for the Grafana instance. Possible values are `Standard` and `Essential`. Defaults to `Standard`. Changing this forces a new Dashboard Grafana to be created.
        pub sku: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `smtp` block as defined below.
        pub smtp: pulumi_gestalt_rust::Output<
            Option<super::super::types::dashboard::GrafanaSmtp>,
        >,
        /// A mapping of tags which should be assigned to the Dashboard Grafana.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to enable the zone redundancy setting of the Grafana instance. Defaults to `false`. Changing this forces a new Dashboard Grafana to be created.
        pub zone_redundancy_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GrafanaArgs,
    ) -> GrafanaResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_key_enabled_binding = args.api_key_enabled.get_output(context);
        let auto_generated_domain_name_label_scope_binding = args
            .auto_generated_domain_name_label_scope
            .get_output(context);
        let azure_monitor_workspace_integrations_binding = args
            .azure_monitor_workspace_integrations
            .get_output(context);
        let deterministic_outbound_ip_enabled_binding = args
            .deterministic_outbound_ip_enabled
            .get_output(context);
        let grafana_major_version_binding = args
            .grafana_major_version
            .get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let smtp_binding = args.smtp.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let zone_redundancy_enabled_binding = args
            .zone_redundancy_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:dashboard/grafana:Grafana".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiKeyEnabled".into(),
                    value: api_key_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoGeneratedDomainNameLabelScope".into(),
                    value: auto_generated_domain_name_label_scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureMonitorWorkspaceIntegrations".into(),
                    value: azure_monitor_workspace_integrations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deterministicOutboundIpEnabled".into(),
                    value: deterministic_outbound_ip_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grafanaMajorVersion".into(),
                    value: grafana_major_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: public_network_access_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smtp".into(),
                    value: smtp_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneRedundancyEnabled".into(),
                    value: zone_redundancy_enabled_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GrafanaResult {
            api_key_enabled: o.get_field("apiKeyEnabled"),
            auto_generated_domain_name_label_scope: o
                .get_field("autoGeneratedDomainNameLabelScope"),
            azure_monitor_workspace_integrations: o
                .get_field("azureMonitorWorkspaceIntegrations"),
            deterministic_outbound_ip_enabled: o
                .get_field("deterministicOutboundIpEnabled"),
            endpoint: o.get_field("endpoint"),
            grafana_major_version: o.get_field("grafanaMajorVersion"),
            grafana_version: o.get_field("grafanaVersion"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            outbound_ips: o.get_field("outboundIps"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            smtp: o.get_field("smtp"),
            tags: o.get_field("tags"),
            zone_redundancy_enabled: o.get_field("zoneRedundancyEnabled"),
        }
    }
}
