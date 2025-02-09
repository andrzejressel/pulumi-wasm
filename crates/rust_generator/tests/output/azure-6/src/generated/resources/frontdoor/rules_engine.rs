/// !> **IMPORTANT** This deploys an Azure Front Door (classic) resource which has been deprecated and will receive security updates only. Please migrate your existing Azure Front Door (classic) deployments to the new Azure Front Door (standard/premium) resources. For your convenience, the service team has exposed a `Front Door Classic` to `Front Door Standard/Premium` [migration tool](https://learn.microsoft.com/azure/frontdoor/tier-migration) to allow you to migrate your existing `Front Door Classic` instances to the new `Front Door Standard/Premium` product tiers.
///
/// Manages an Azure Front Door (classic) Rules Engine configuration and rules.
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
///             .name("example-rg")
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
///             .name("example")
///             .resource_group_name("${example.name}")
///             .routing_rules(
///                 vec![
///                     FrontdoorRoutingRule::builder().acceptedProtocols(vec!["Http",
///                     "Https",]).frontendEndpoints(vec!["exampleFrontendEndpoint1",])
///                     .name("exampleRoutingRule1").patternsToMatches(vec!["/*",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleRulesEngine = rules_engine::create(
///         "exampleRulesEngine",
///         RulesEngineArgs::builder()
///             .frontdoor_name("${exampleFrontdoor.name}")
///             .name("exampleRulesEngineConfig1")
///             .resource_group_name("${exampleFrontdoor.resourceGroupName}")
///             .rules(
///                 vec![
///                     RulesEngineRule::builder().action(RulesEngineRuleAction::builder()
///                     .responseHeaders(vec![RulesEngineRuleActionResponseHeader::builder()
///                     .headerActionType("Append").headerName("X-TEST-HEADER")
///                     .value("Append Header Rule").build_struct(),]).build_struct())
///                     .name("debuggingoutput").priority(1).build_struct(),
///                     RulesEngineRule::builder().action(RulesEngineRuleAction::builder()
///                     .responseHeaders(vec![RulesEngineRuleActionResponseHeader::builder()
///                     .headerActionType("Overwrite")
///                     .headerName("Access-Control-Allow-Origin").value("*").build_struct(),
///                     RulesEngineRuleActionResponseHeader::builder()
///                     .headerActionType("Overwrite")
///                     .headerName("Access-Control-Allow-Credentials").value("true")
///                     .build_struct(),]).build_struct())
///                     .matchConditions(vec![RulesEngineRuleMatchCondition::builder()
///                     .operator("Equal").values(vec!["GET", "POST",])
///                     .variable("RequestMethod").build_struct(),]).name("overwriteorigin")
///                     .priority(2).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Front Door Rules Engine's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:frontdoor/rulesEngine:RulesEngine example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Network/frontdoors/frontdoor1/rulesEngines/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod rules_engine {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RulesEngineArgs {
        /// Whether this Rules engine configuration is enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Front Door instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub frontdoor_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Rules engine configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `rule` block as defined below.
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::frontdoor::RulesEngineRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RulesEngineResult {
        /// Whether this Rules engine configuration is enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Front Door instance. Changing this forces a new resource to be created.
        pub frontdoor_name: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Rules engine configuration. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `rule` block as defined below.
        pub rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::frontdoor::RulesEngineRule>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RulesEngineArgs,
    ) -> RulesEngineResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_binding = args.enabled.get_output(context);
        let frontdoor_name_binding = args.frontdoor_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:frontdoor/rulesEngine:RulesEngine".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontdoorName".into(),
                    value: frontdoor_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RulesEngineResult {
            enabled: o.get_field("enabled"),
            frontdoor_name: o.get_field("frontdoorName"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            rules: o.get_field("rules"),
        }
    }
}
