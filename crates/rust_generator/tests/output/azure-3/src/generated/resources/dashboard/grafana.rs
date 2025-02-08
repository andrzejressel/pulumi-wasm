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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GrafanaArgs,
    ) -> GrafanaResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_key_enabled_binding = args
            .api_key_enabled
            .get_output(context)
            .get_inner();
        let auto_generated_domain_name_label_scope_binding = args
            .auto_generated_domain_name_label_scope
            .get_output(context)
            .get_inner();
        let azure_monitor_workspace_integrations_binding = args
            .azure_monitor_workspace_integrations
            .get_output(context)
            .get_inner();
        let deterministic_outbound_ip_enabled_binding = args
            .deterministic_outbound_ip_enabled
            .get_output(context)
            .get_inner();
        let grafana_major_version_binding = args
            .grafana_major_version
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let smtp_binding = args.smtp.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let zone_redundancy_enabled_binding = args
            .zone_redundancy_enabled
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dashboard/grafana:Grafana".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiKeyEnabled".into(),
                    value: &api_key_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "autoGeneratedDomainNameLabelScope".into(),
                    value: &auto_generated_domain_name_label_scope_binding,
                },
                register_interface::ObjectField {
                    name: "azureMonitorWorkspaceIntegrations".into(),
                    value: &azure_monitor_workspace_integrations_binding,
                },
                register_interface::ObjectField {
                    name: "deterministicOutboundIpEnabled".into(),
                    value: &deterministic_outbound_ip_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "grafanaMajorVersion".into(),
                    value: &grafana_major_version_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
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
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "smtp".into(),
                    value: &smtp_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zoneRedundancyEnabled".into(),
                    value: &zone_redundancy_enabled_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GrafanaResult {
            api_key_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiKeyEnabled"),
            ),
            auto_generated_domain_name_label_scope: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoGeneratedDomainNameLabelScope"),
            ),
            azure_monitor_workspace_integrations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureMonitorWorkspaceIntegrations"),
            ),
            deterministic_outbound_ip_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deterministicOutboundIpEnabled"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            grafana_major_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("grafanaMajorVersion"),
            ),
            grafana_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("grafanaVersion"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            outbound_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outboundIps"),
            ),
            public_network_access_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            smtp: pulumi_gestalt_rust::__private::into_domain(o.extract_field("smtp")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            zone_redundancy_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneRedundancyEnabled"),
            ),
        }
    }
}
