/// !> **IMPORTANT** This deploys an Azure Front Door (classic) resource which has been deprecated and will receive security updates only. Please migrate your existing Azure Front Door (classic) deployments to the new Azure Front Door (standard/premium) resources. For your convenience, the service team has exposed a `Front Door Classic` to `Front Door Standard/Premium` [migration tool](https://learn.microsoft.com/azure/frontdoor/tier-migration) to allow you to migrate your existing `Front Door Classic` instances to the new `Front Door Standard/Premium` product tiers.
///
/// Manages an Azure Front Door (classic) instance.
///
/// Azure Front Door Service is Microsoft's highly available and scalable web application acceleration platform and global HTTP(S) load balancer. It provides built-in DDoS protection and application layer security and caching. Front Door enables you to build applications that maximize and automate high-availability and performance for your end-users. Use Front Door with Azure services including Web/Mobile Apps, Cloud Services and Virtual Machines – or combine it with on-premises services for hybrid deployments and smooth cloud migration.
///
/// Below are some of the key scenarios that Azure Front Door Service addresses:
///
/// * Use Front Door to improve application scale and availability with instant multi-region failover
/// * Use Front Door to improve application performance with SSL offload and routing requests to the fastest available application backend.
/// * Use Front Door for application layer security and DDoS protection for your application.
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
///             .name("FrontDoorExampleResourceGroup")
///             .build_struct(),
///     );
///     let exampleFrontdoor = frontdoor::create(
///         "exampleFrontdoor",
///         FrontdoorArgs::builder()
///             .backend_pool_health_probes(
///                 vec![
///                     FrontdoorBackendPoolHealthProbe::builder()
///                     .name("exampleHealthProbeSetting1").build_struct(),
///                 ],
///             )
///             .backend_pool_load_balancings(
///                 vec![
///                     FrontdoorBackendPoolLoadBalancing::builder()
///                     .name("exampleLoadBalancingSettings1").build_struct(),
///                 ],
///             )
///             .backend_pools(
///                 vec![
///                     FrontdoorBackendPool::builder()
///                     .backends(vec![FrontdoorBackendPoolBackend::builder()
///                     .address("www.bing.com").hostHeader("www.bing.com").httpPort(80)
///                     .httpsPort(443).build_struct(),])
///                     .healthProbeName("exampleHealthProbeSetting1")
///                     .loadBalancingName("exampleLoadBalancingSettings1")
///                     .name("exampleBackendBing").build_struct(),
///                 ],
///             )
///             .frontend_endpoints(
///                 vec![
///                     FrontdoorFrontendEndpoint::builder()
///                     .hostName("example-FrontDoor.azurefd.net")
///                     .name("exampleFrontendEndpoint1").build_struct(),
///                 ],
///             )
///             .name("example-FrontDoor")
///             .resource_group_name("${example.name}")
///             .routing_rules(
///                 vec![
///                     FrontdoorRoutingRule::builder().acceptedProtocols(vec!["Http",
///                     "Https",])
///                     .forwardingConfiguration(FrontdoorRoutingRuleForwardingConfiguration::builder()
///                     .backendPoolName("exampleBackendBing")
///                     .forwardingProtocol("MatchRequest").build_struct())
///                     .frontendEndpoints(vec!["exampleFrontendEndpoint1",])
///                     .name("exampleRoutingRule1").patternsToMatches(vec!["/*",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Front Doors can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:frontdoor/frontdoor:Frontdoor example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/frontDoors/frontdoor1
/// ```
///
pub mod frontdoor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorArgs {
        /// A `backend_pool_health_probe` block as defined below.
        #[builder(into)]
        pub backend_pool_health_probes: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::frontdoor::FrontdoorBackendPoolHealthProbe>,
        >,
        /// A `backend_pool_load_balancing` block as defined below.
        #[builder(into)]
        pub backend_pool_load_balancings: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::frontdoor::FrontdoorBackendPoolLoadBalancing>,
        >,
        /// A `backend_pool_settings` block as defined below.
        #[builder(into, default)]
        pub backend_pool_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::frontdoor::FrontdoorBackendPoolSetting>>,
        >,
        /// A `backend_pool` block as defined below.
        ///
        /// > Azure by default allows specifying up to 50 Backend Pools - but this quota can be increased via Microsoft Support.
        #[builder(into)]
        pub backend_pools: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::frontdoor::FrontdoorBackendPool>,
        >,
        /// A friendly name for the Front Door service.
        #[builder(into, default)]
        pub friendly_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `frontend_endpoint` block as defined below.
        #[builder(into)]
        pub frontend_endpoints: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::frontdoor::FrontdoorFrontendEndpoint>,
        >,
        /// Should the Front Door Load Balancer be Enabled? Defaults to `true`.
        #[builder(into, default)]
        pub load_balancer_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the Front Door service. Must be globally unique. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group in which the Front Door service should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `routing_rule` block as defined below.
        #[builder(into)]
        pub routing_rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::frontdoor::FrontdoorRoutingRule>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FrontdoorResult {
        /// A `backend_pool_health_probe` block as defined below.
        pub backend_pool_health_probes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::frontdoor::FrontdoorBackendPoolHealthProbe>,
        >,
        /// A map/dictionary of Backend Pool Health Probe Names (key) to the Backend Pool Health Probe ID (value)
        pub backend_pool_health_probes_map: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A map/dictionary of Backend Pool Load Balancing Setting Names (key) to the Backend Pool Load Balancing Setting ID (value)
        pub backend_pool_load_balancing_settings_map: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A `backend_pool_load_balancing` block as defined below.
        pub backend_pool_load_balancings: pulumi_gestalt_rust::Output<
            Vec<super::super::types::frontdoor::FrontdoorBackendPoolLoadBalancing>,
        >,
        /// A `backend_pool_settings` block as defined below.
        pub backend_pool_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::types::frontdoor::FrontdoorBackendPoolSetting>,
        >,
        /// A `backend_pool` block as defined below.
        ///
        /// > Azure by default allows specifying up to 50 Backend Pools - but this quota can be increased via Microsoft Support.
        pub backend_pools: pulumi_gestalt_rust::Output<
            Vec<super::super::types::frontdoor::FrontdoorBackendPool>,
        >,
        /// A map/dictionary of Backend Pool Names (key) to the Backend Pool ID (value)
        pub backend_pools_map: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The host that each frontendEndpoint must CNAME to.
        pub cname: pulumi_gestalt_rust::Output<String>,
        pub explicit_resource_orders: pulumi_gestalt_rust::Output<
            Vec<super::super::types::frontdoor::FrontdoorExplicitResourceOrder>,
        >,
        /// A friendly name for the Front Door service.
        pub friendly_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `frontend_endpoint` block as defined below.
        pub frontend_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::frontdoor::FrontdoorFrontendEndpoint>,
        >,
        /// A map/dictionary of Frontend Endpoint Names (key) to the Frontend Endpoint ID (value)
        pub frontend_endpoints_map: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The unique ID of the Front Door which is embedded into the incoming headers `X-Azure-FDID` attribute and maybe used to filter traffic sent by the Front Door to your backend.
        pub header_frontdoor_id: pulumi_gestalt_rust::Output<String>,
        /// Should the Front Door Load Balancer be Enabled? Defaults to `true`.
        pub load_balancer_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Front Door service. Must be globally unique. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group in which the Front Door service should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `routing_rule` block as defined below.
        pub routing_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::frontdoor::FrontdoorRoutingRule>,
        >,
        /// A map/dictionary of Routing Rule Names (key) to the Routing Rule ID (value)
        pub routing_rules_map: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FrontdoorArgs,
    ) -> FrontdoorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let backend_pool_health_probes_binding = args
            .backend_pool_health_probes
            .get_output(context)
            .get_inner();
        let backend_pool_load_balancings_binding = args
            .backend_pool_load_balancings
            .get_output(context)
            .get_inner();
        let backend_pool_settings_binding = args
            .backend_pool_settings
            .get_output(context)
            .get_inner();
        let backend_pools_binding = args.backend_pools.get_output(context).get_inner();
        let friendly_name_binding = args.friendly_name.get_output(context).get_inner();
        let frontend_endpoints_binding = args
            .frontend_endpoints
            .get_output(context)
            .get_inner();
        let load_balancer_enabled_binding = args
            .load_balancer_enabled
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let routing_rules_binding = args.routing_rules.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:frontdoor/frontdoor:Frontdoor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backendPoolHealthProbes".into(),
                    value: &backend_pool_health_probes_binding,
                },
                register_interface::ObjectField {
                    name: "backendPoolLoadBalancings".into(),
                    value: &backend_pool_load_balancings_binding,
                },
                register_interface::ObjectField {
                    name: "backendPoolSettings".into(),
                    value: &backend_pool_settings_binding,
                },
                register_interface::ObjectField {
                    name: "backendPools".into(),
                    value: &backend_pools_binding,
                },
                register_interface::ObjectField {
                    name: "friendlyName".into(),
                    value: &friendly_name_binding,
                },
                register_interface::ObjectField {
                    name: "frontendEndpoints".into(),
                    value: &frontend_endpoints_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancerEnabled".into(),
                    value: &load_balancer_enabled_binding,
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
                    name: "routingRules".into(),
                    value: &routing_rules_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FrontdoorResult {
            backend_pool_health_probes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendPoolHealthProbes"),
            ),
            backend_pool_health_probes_map: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendPoolHealthProbesMap"),
            ),
            backend_pool_load_balancing_settings_map: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendPoolLoadBalancingSettingsMap"),
            ),
            backend_pool_load_balancings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendPoolLoadBalancings"),
            ),
            backend_pool_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendPoolSettings"),
            ),
            backend_pools: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendPools"),
            ),
            backend_pools_map: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendPoolsMap"),
            ),
            cname: pulumi_gestalt_rust::__private::into_domain(o.extract_field("cname")),
            explicit_resource_orders: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("explicitResourceOrders"),
            ),
            friendly_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("friendlyName"),
            ),
            frontend_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frontendEndpoints"),
            ),
            frontend_endpoints_map: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frontendEndpointsMap"),
            ),
            header_frontdoor_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("headerFrontdoorId"),
            ),
            load_balancer_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadBalancerEnabled"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            routing_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routingRules"),
            ),
            routing_rules_map: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routingRulesMap"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
