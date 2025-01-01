/// !> **IMPORTANT** This deploys an Azure Front Door (classic) resource which has been deprecated and will receive security updates only. Please migrate your existing Azure Front Door (classic) deployments to the new Azure Front Door (standard/premium) resources. For your convenience, the service team has exposed a `Front Door Classic` to `Front Door Standard/Premium` [migration tool](https://learn.microsoft.com/azure/frontdoor/tier-migration) to allow you to migrate your existing `Front Door Classic` instances to the new `Front Door Standard/Premium` product tiers.
///
/// Manages an Azure Front Door (classic) Rules Engine configuration and rules.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleFrontdoor:
///     type: azure:frontdoor:Frontdoor
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       backendPools:
///         - name: exampleBackendBing
///           loadBalancingName: exampleLoadBalancingSettings1
///           healthProbeName: exampleHealthProbeSetting1
///           backends:
///             - hostHeader: www.bing.com
///               address: www.bing.com
///               httpPort: 80
///               httpsPort: 443
///       backendPoolHealthProbes:
///         - name: exampleHealthProbeSetting1
///       backendPoolLoadBalancings:
///         - name: exampleLoadBalancingSettings1
///       frontendEndpoints:
///         - name: exampleFrontendEndpoint1
///           hostName: example-FrontDoor.azurefd.net
///       routingRules:
///         - name: exampleRoutingRule1
///           acceptedProtocols:
///             - Http
///             - Https
///           patternsToMatches:
///             - /*
///           frontendEndpoints:
///             - exampleFrontendEndpoint1
///   exampleRulesEngine:
///     type: azure:frontdoor:RulesEngine
///     name: example_rules_engine
///     properties:
///       name: exampleRulesEngineConfig1
///       frontdoorName: ${exampleFrontdoor.name}
///       resourceGroupName: ${exampleFrontdoor.resourceGroupName}
///       rules:
///         - name: debuggingoutput
///           priority: 1
///           action:
///             responseHeaders:
///               - headerActionType: Append
///                 headerName: X-TEST-HEADER
///                 value: Append Header Rule
///         - name: overwriteorigin
///           priority: 2
///           matchConditions:
///             - variable: RequestMethod
///               operator: Equal
///               values:
///                 - GET
///                 - POST
///           action:
///             responseHeaders:
///               - headerActionType: Overwrite
///                 headerName: Access-Control-Allow-Origin
///                 value: '*'
///               - headerActionType: Overwrite
///                 headerName: Access-Control-Allow-Credentials
///                 value: 'true'
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
pub mod rules_engine {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RulesEngineArgs {
        /// Whether this Rules engine configuration is enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Front Door instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub frontdoor_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Rules engine configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `rule` block as defined below.
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::frontdoor::RulesEngineRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RulesEngineResult {
        /// Whether this Rules engine configuration is enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Front Door instance. Changing this forces a new resource to be created.
        pub frontdoor_name: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Rules engine configuration. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `rule` block as defined below.
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::frontdoor::RulesEngineRule>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RulesEngineArgs) -> RulesEngineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_inner();
        let frontdoor_name_binding = args.frontdoor_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let rules_binding = args.rules.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:frontdoor/rulesEngine:RulesEngine".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "frontdoorName".into(),
                    value: &frontdoor_name_binding,
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
                    name: "rules".into(),
                    value: &rules_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "frontdoorName".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RulesEngineResult {
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            frontdoor_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontdoorName").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
        }
    }
}
