/// Manages a Firewall Rule associated with a Redis Cache.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   server:
///     type: random:RandomId
///     properties:
///       keepers:
///         azi_id: 1
///       byteLength: 8
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: redis-resourcegroup
///       location: West Europe
///   exampleCache:
///     type: azure:redis:Cache
///     name: example
///     properties:
///       name: redis${server.hex}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       capacity: 1
///       family: P
///       skuName: Premium
///       enableNonSslPort: false
///       redisConfiguration:
///         maxmemoryReserved: 2
///         maxmemoryDelta: 2
///         maxmemoryPolicy: allkeys-lru
///   exampleFirewallRule:
///     type: azure:redis:FirewallRule
///     name: example
///     properties:
///       name: someIPrange
///       redisCacheName: ${exampleCache.name}
///       resourceGroupName: ${example.name}
///       startIp: 1.2.3.4
///       endIp: 2.3.4.5
/// ```
///
/// ## Import
///
/// Redis Firewall Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:redis/firewallRule:FirewallRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Cache/redis/cache1/firewallRules/rule1
/// ```
///
pub mod firewall_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallRuleArgs {
        /// The highest IP address included in the range.
        #[builder(into)]
        pub end_ip: pulumi_wasm_rust::Output<String>,
        /// The name of the Firewall Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Redis Cache. Changing this forces a new resource to be created.
        #[builder(into)]
        pub redis_cache_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which this Redis Cache exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The lowest IP address included in the range
        #[builder(into)]
        pub start_ip: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FirewallRuleResult {
        /// The highest IP address included in the range.
        pub end_ip: pulumi_wasm_rust::Output<String>,
        /// The name of the Firewall Rule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Redis Cache. Changing this forces a new resource to be created.
        pub redis_cache_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which this Redis Cache exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The lowest IP address included in the range
        pub start_ip: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FirewallRuleArgs) -> FirewallRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let end_ip_binding = args.end_ip.get_inner();
        let name_binding = args.name.get_inner();
        let redis_cache_name_binding = args.redis_cache_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let start_ip_binding = args.start_ip.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:redis/firewallRule:FirewallRule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "endIp".into(),
                    value: &end_ip_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "redisCacheName".into(),
                    value: &redis_cache_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "startIp".into(),
                    value: &start_ip_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "endIp".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "redisCacheName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "startIp".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FirewallRuleResult {
            end_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endIp").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            redis_cache_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redisCacheName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            start_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startIp").unwrap(),
            ),
        }
    }
}
