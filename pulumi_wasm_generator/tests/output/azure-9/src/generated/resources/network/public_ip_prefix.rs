/// Manages a Public IP Prefix.
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
///   examplePublicIpPrefix:
///     type: azure:network:PublicIpPrefix
///     name: example
///     properties:
///       name: acceptanceTestPublicIpPrefix1
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       prefixLength: 31
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// Public IP Prefixes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/publicIpPrefix:PublicIpPrefix myPublicIpPrefix /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/publicIPPrefixes/myPublicIpPrefix1
/// ```
///
pub mod public_ip_prefix {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PublicIpPrefixArgs {
        /// The IP Version to use, `IPv6` or `IPv4`. Changing this forces a new resource to be created. Default is `IPv4`.
        #[builder(into, default)]
        pub ip_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Public IP Prefix resource . Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the number of bits of the prefix. The value can be set between 0 (4,294,967,296 addresses) and 31 (2 addresses). Defaults to `28`(16 addresses). Changing this forces a new resource to be created.
        ///
        /// > **Please Note:** There may be Public IP address limits on the subscription . [More information available here](https://docs.microsoft.com/azure/azure-subscription-service-limits?toc=%2fazure%2fvirtual-network%2ftoc.json#publicip-address)
        #[builder(into, default)]
        pub prefix_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the resource group in which to create the Public IP Prefix. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU of the Public IP Prefix. Accepted values are `Standard`. Defaults to `Standard`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Public IP Prefix can only be created with Standard SKUs at this time.
        #[builder(into, default)]
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// The SKU Tier that should be used for the Public IP. Possible values are `Regional` and `Global`. Defaults to `Regional`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sku_tier: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of Availability Zones in which this Public IP Prefix should be located. Changing this forces a new Public IP Prefix to be created.
        ///
        /// > **Please Note:** Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct PublicIpPrefixResult {
        /// The IP address prefix value that was allocated.
        pub ip_prefix: pulumi_wasm_rust::Output<String>,
        /// The IP Version to use, `IPv6` or `IPv4`. Changing this forces a new resource to be created. Default is `IPv4`.
        pub ip_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Public IP Prefix resource . Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the number of bits of the prefix. The value can be set between 0 (4,294,967,296 addresses) and 31 (2 addresses). Defaults to `28`(16 addresses). Changing this forces a new resource to be created.
        ///
        /// > **Please Note:** There may be Public IP address limits on the subscription . [More information available here](https://docs.microsoft.com/azure/azure-subscription-service-limits?toc=%2fazure%2fvirtual-network%2ftoc.json#publicip-address)
        pub prefix_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the resource group in which to create the Public IP Prefix. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU of the Public IP Prefix. Accepted values are `Standard`. Defaults to `Standard`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Public IP Prefix can only be created with Standard SKUs at this time.
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// The SKU Tier that should be used for the Public IP. Possible values are `Regional` and `Global`. Defaults to `Regional`. Changing this forces a new resource to be created.
        pub sku_tier: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of Availability Zones in which this Public IP Prefix should be located. Changing this forces a new Public IP Prefix to be created.
        ///
        /// > **Please Note:** Availability Zones are [only supported in several regions at this time](https://docs.microsoft.com/azure/availability-zones/az-overview).
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PublicIpPrefixArgs) -> PublicIpPrefixResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ip_version_binding = args.ip_version.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let prefix_length_binding = args.prefix_length.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_binding = args.sku.get_inner();
        let sku_tier_binding = args.sku_tier.get_inner();
        let tags_binding = args.tags.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/publicIpPrefix:PublicIpPrefix".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ipVersion".into(),
                    value: &ip_version_binding,
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
                    name: "prefixLength".into(),
                    value: &prefix_length_binding,
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
                    name: "skuTier".into(),
                    value: &sku_tier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "ipPrefix".into(),
                },
                register_interface::ResultField {
                    name: "ipVersion".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "prefixLength".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "skuTier".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PublicIpPrefixResult {
            ip_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipPrefix").unwrap(),
            ),
            ip_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipVersion").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            prefix_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefixLength").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            sku_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuTier").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
