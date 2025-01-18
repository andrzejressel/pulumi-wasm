/// Manages a Public IP Address.
///
/// > **Note** If this resource is to be associated with a resource that requires disassociation before destruction (such as `azure.network.NetworkInterface`) it is recommended to set the `lifecycle` argument `create_before_destroy = true`. Otherwise, it can fail to disassociate on destruction.
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
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: acceptanceTestPublicIp1
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       allocationMethod: Static
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// Public IPs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/publicIp:PublicIp myPublicIp /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/publicIPAddresses/myPublicIpAddress1
/// ```
///
pub mod public_ip {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PublicIpArgs {
        /// Defines the allocation method for this IP address. Possible values are `Static` or `Dynamic`.
        ///
        /// > **Note** `Dynamic` Public IP Addresses aren't allocated until they're assigned to a resource (such as a Virtual Machine or a Load Balancer) by design within Azure. See `ip_address` argument.
        #[builder(into)]
        pub allocation_method: pulumi_wasm_rust::Output<String>,
        /// The DDoS protection mode of the public IP. Possible values are `Disabled`, `Enabled`, and `VirtualNetworkInherited`. Defaults to `VirtualNetworkInherited`.
        #[builder(into, default)]
        pub ddos_protection_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of DDoS protection plan associated with the public IP.
        ///
        /// > **Note:** `ddos_protection_plan_id` can only be set when `ddos_protection_mode` is `Enabled`.
        #[builder(into, default)]
        pub ddos_protection_plan_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Label for the Domain Name. Will be used to make up the FQDN. If a domain name label is specified, an A DNS record is created for the public IP in the Microsoft Azure DNS system.
        #[builder(into, default)]
        pub domain_name_label: pulumi_wasm_rust::Output<Option<String>>,
        /// Scope for the domain name label. If a domain name label scope is specified, an A DNS record is created for the public IP in the Microsoft Azure DNS system with a hashed value includes in FQDN. Possible values are `NoReuse`, `ResourceGroupReuse`, `SubscriptionReuse` and `TenantReuse`. Changing this forces a new Public IP to be created.
        #[builder(into, default)]
        pub domain_name_label_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Edge Zone within the Azure Region where this Public IP should exist. Changing this forces a new Public IP to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the timeout for the TCP idle connection. The value can be set between 4 and 30 minutes.
        #[builder(into, default)]
        pub idle_timeout_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// A mapping of IP tags to assign to the public IP. Changing this forces a new resource to be created.
        ///
        /// > **Note** IP Tag `RoutingPreference` requires multiple `zones` and `Standard` SKU to be set.
        #[builder(into, default)]
        pub ip_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The IP Version to use, IPv6 or IPv4. Changing this forces a new resource to be created. Defaults to `IPv4`.
        ///
        /// > **Note** Only `static` IP address allocation is supported for IPv6.
        #[builder(into, default)]
        pub ip_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the Public IP should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Public IP. Changing this forces a new Public IP to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// If specified then public IP address allocated will be provided from the public IP prefix resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub public_ip_prefix_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where this Public IP should exist. Changing this forces a new Public IP to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A fully qualified domain name that resolves to this public IP address. If the reverseFqdn is specified, then a PTR DNS record is created pointing from the IP address in the in-addr.arpa domain to the reverse FQDN.
        #[builder(into, default)]
        pub reverse_fqdn: pulumi_wasm_rust::Output<Option<String>>,
        /// The SKU of the Public IP. Accepted values are `Basic` and `Standard`. Defaults to `Standard`. Changing this forces a new resource to be created.
        ///
        /// > **Note** Public IP Standard SKUs require `allocation_method` to be set to `Static`.
        #[builder(into, default)]
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// The SKU Tier that should be used for the Public IP. Possible values are `Regional` and `Global`. Defaults to `Regional`. Changing this forces a new resource to be created.
        ///
        /// > **Note** When `sku_tier` is set to `Global`, `sku` must be set to `Standard`.
        #[builder(into, default)]
        pub sku_tier: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A collection containing the availability zone to allocate the Public IP in. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Availability Zones are only supported with a [Standard SKU](https://docs.microsoft.com/azure/virtual-network/virtual-network-ip-addresses-overview-arm#standard) and [in select regions](https://docs.microsoft.com/azure/availability-zones/az-overview) at this time. Standard SKU Public IP Addresses that do not specify a zone are **not** zone-redundant by default.
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct PublicIpResult {
        /// Defines the allocation method for this IP address. Possible values are `Static` or `Dynamic`.
        ///
        /// > **Note** `Dynamic` Public IP Addresses aren't allocated until they're assigned to a resource (such as a Virtual Machine or a Load Balancer) by design within Azure. See `ip_address` argument.
        pub allocation_method: pulumi_wasm_rust::Output<String>,
        /// The DDoS protection mode of the public IP. Possible values are `Disabled`, `Enabled`, and `VirtualNetworkInherited`. Defaults to `VirtualNetworkInherited`.
        pub ddos_protection_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of DDoS protection plan associated with the public IP.
        ///
        /// > **Note:** `ddos_protection_plan_id` can only be set when `ddos_protection_mode` is `Enabled`.
        pub ddos_protection_plan_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Label for the Domain Name. Will be used to make up the FQDN. If a domain name label is specified, an A DNS record is created for the public IP in the Microsoft Azure DNS system.
        pub domain_name_label: pulumi_wasm_rust::Output<Option<String>>,
        /// Scope for the domain name label. If a domain name label scope is specified, an A DNS record is created for the public IP in the Microsoft Azure DNS system with a hashed value includes in FQDN. Possible values are `NoReuse`, `ResourceGroupReuse`, `SubscriptionReuse` and `TenantReuse`. Changing this forces a new Public IP to be created.
        pub domain_name_label_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Edge Zone within the Azure Region where this Public IP should exist. Changing this forces a new Public IP to be created.
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// Fully qualified domain name of the A DNS record associated with the public IP. `domain_name_label` must be specified to get the `fqdn`. This is the concatenation of the `domain_name_label` and the regionalized DNS zone
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// Specifies the timeout for the TCP idle connection. The value can be set between 4 and 30 minutes.
        pub idle_timeout_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// The IP address value that was allocated.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// A mapping of IP tags to assign to the public IP. Changing this forces a new resource to be created.
        ///
        /// > **Note** IP Tag `RoutingPreference` requires multiple `zones` and `Standard` SKU to be set.
        pub ip_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The IP Version to use, IPv6 or IPv4. Changing this forces a new resource to be created. Defaults to `IPv4`.
        ///
        /// > **Note** Only `static` IP address allocation is supported for IPv6.
        pub ip_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the Public IP should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Public IP. Changing this forces a new Public IP to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// If specified then public IP address allocated will be provided from the public IP prefix resource. Changing this forces a new resource to be created.
        pub public_ip_prefix_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where this Public IP should exist. Changing this forces a new Public IP to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A fully qualified domain name that resolves to this public IP address. If the reverseFqdn is specified, then a PTR DNS record is created pointing from the IP address in the in-addr.arpa domain to the reverse FQDN.
        pub reverse_fqdn: pulumi_wasm_rust::Output<Option<String>>,
        /// The SKU of the Public IP. Accepted values are `Basic` and `Standard`. Defaults to `Standard`. Changing this forces a new resource to be created.
        ///
        /// > **Note** Public IP Standard SKUs require `allocation_method` to be set to `Static`.
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// The SKU Tier that should be used for the Public IP. Possible values are `Regional` and `Global`. Defaults to `Regional`. Changing this forces a new resource to be created.
        ///
        /// > **Note** When `sku_tier` is set to `Global`, `sku` must be set to `Standard`.
        pub sku_tier: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A collection containing the availability zone to allocate the Public IP in. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Availability Zones are only supported with a [Standard SKU](https://docs.microsoft.com/azure/virtual-network/virtual-network-ip-addresses-overview-arm#standard) and [in select regions](https://docs.microsoft.com/azure/availability-zones/az-overview) at this time. Standard SKU Public IP Addresses that do not specify a zone are **not** zone-redundant by default.
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PublicIpArgs) -> PublicIpResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allocation_method_binding = args.allocation_method.get_inner();
        let ddos_protection_mode_binding = args.ddos_protection_mode.get_inner();
        let ddos_protection_plan_id_binding = args.ddos_protection_plan_id.get_inner();
        let domain_name_label_binding = args.domain_name_label.get_inner();
        let domain_name_label_scope_binding = args.domain_name_label_scope.get_inner();
        let edge_zone_binding = args.edge_zone.get_inner();
        let idle_timeout_in_minutes_binding = args.idle_timeout_in_minutes.get_inner();
        let ip_tags_binding = args.ip_tags.get_inner();
        let ip_version_binding = args.ip_version.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let public_ip_prefix_id_binding = args.public_ip_prefix_id.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let reverse_fqdn_binding = args.reverse_fqdn.get_inner();
        let sku_binding = args.sku.get_inner();
        let sku_tier_binding = args.sku_tier.get_inner();
        let tags_binding = args.tags.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/publicIp:PublicIp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allocationMethod".into(),
                    value: &allocation_method_binding,
                },
                register_interface::ObjectField {
                    name: "ddosProtectionMode".into(),
                    value: &ddos_protection_mode_binding,
                },
                register_interface::ObjectField {
                    name: "ddosProtectionPlanId".into(),
                    value: &ddos_protection_plan_id_binding,
                },
                register_interface::ObjectField {
                    name: "domainNameLabel".into(),
                    value: &domain_name_label_binding,
                },
                register_interface::ObjectField {
                    name: "domainNameLabelScope".into(),
                    value: &domain_name_label_scope_binding,
                },
                register_interface::ObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding,
                },
                register_interface::ObjectField {
                    name: "idleTimeoutInMinutes".into(),
                    value: &idle_timeout_in_minutes_binding,
                },
                register_interface::ObjectField {
                    name: "ipTags".into(),
                    value: &ip_tags_binding,
                },
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
                    name: "publicIpPrefixId".into(),
                    value: &public_ip_prefix_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "reverseFqdn".into(),
                    value: &reverse_fqdn_binding,
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
                    name: "allocationMethod".into(),
                },
                register_interface::ResultField {
                    name: "ddosProtectionMode".into(),
                },
                register_interface::ResultField {
                    name: "ddosProtectionPlanId".into(),
                },
                register_interface::ResultField {
                    name: "domainNameLabel".into(),
                },
                register_interface::ResultField {
                    name: "domainNameLabelScope".into(),
                },
                register_interface::ResultField {
                    name: "edgeZone".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "idleTimeoutInMinutes".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "ipTags".into(),
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
                    name: "publicIpPrefixId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "reverseFqdn".into(),
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
        PublicIpResult {
            allocation_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationMethod").unwrap(),
            ),
            ddos_protection_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ddosProtectionMode").unwrap(),
            ),
            ddos_protection_plan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ddosProtectionPlanId").unwrap(),
            ),
            domain_name_label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainNameLabel").unwrap(),
            ),
            domain_name_label_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainNameLabelScope").unwrap(),
            ),
            edge_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeZone").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            idle_timeout_in_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleTimeoutInMinutes").unwrap(),
            ),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            ip_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipTags").unwrap(),
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
            public_ip_prefix_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpPrefixId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            reverse_fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reverseFqdn").unwrap(),
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
