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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod public_ip {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PublicIpArgs {
        /// Defines the allocation method for this IP address. Possible values are `Static` or `Dynamic`.
        ///
        /// > **Note** `Dynamic` Public IP Addresses aren't allocated until they're assigned to a resource (such as a Virtual Machine or a Load Balancer) by design within Azure. See `ip_address` argument.
        #[builder(into)]
        pub allocation_method: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The DDoS protection mode of the public IP. Possible values are `Disabled`, `Enabled`, and `VirtualNetworkInherited`. Defaults to `VirtualNetworkInherited`.
        #[builder(into, default)]
        pub ddos_protection_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of DDoS protection plan associated with the public IP.
        ///
        /// > **Note:** `ddos_protection_plan_id` can only be set when `ddos_protection_mode` is `Enabled`.
        #[builder(into, default)]
        pub ddos_protection_plan_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Label for the Domain Name. Will be used to make up the FQDN. If a domain name label is specified, an A DNS record is created for the public IP in the Microsoft Azure DNS system.
        #[builder(into, default)]
        pub domain_name_label: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Scope for the domain name label. If a domain name label scope is specified, an A DNS record is created for the public IP in the Microsoft Azure DNS system with a hashed value includes in FQDN. Possible values are `NoReuse`, `ResourceGroupReuse`, `SubscriptionReuse` and `TenantReuse`. Changing this forces a new Public IP to be created.
        #[builder(into, default)]
        pub domain_name_label_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Edge Zone within the Azure Region where this Public IP should exist. Changing this forces a new Public IP to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the timeout for the TCP idle connection. The value can be set between 4 and 30 minutes.
        #[builder(into, default)]
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A mapping of IP tags to assign to the public IP. Changing this forces a new resource to be created.
        ///
        /// > **Note** IP Tag `RoutingPreference` requires multiple `zones` and `Standard` SKU to be set.
        #[builder(into, default)]
        pub ip_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The IP Version to use, IPv6 or IPv4. Changing this forces a new resource to be created. Defaults to `IPv4`.
        ///
        /// > **Note** Only `static` IP address allocation is supported for IPv6.
        #[builder(into, default)]
        pub ip_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the Public IP should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Public IP. Changing this forces a new Public IP to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If specified then public IP address allocated will be provided from the public IP prefix resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub public_ip_prefix_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where this Public IP should exist. Changing this forces a new Public IP to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A fully qualified domain name that resolves to this public IP address. If the reverseFqdn is specified, then a PTR DNS record is created pointing from the IP address in the in-addr.arpa domain to the reverse FQDN.
        #[builder(into, default)]
        pub reverse_fqdn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The SKU of the Public IP. Accepted values are `Basic` and `Standard`. Defaults to `Standard`. Changing this forces a new resource to be created.
        ///
        /// > **Note** Public IP Standard SKUs require `allocation_method` to be set to `Static`.
        #[builder(into, default)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The SKU Tier that should be used for the Public IP. Possible values are `Regional` and `Global`. Defaults to `Regional`. Changing this forces a new resource to be created.
        ///
        /// > **Note** When `sku_tier` is set to `Global`, `sku` must be set to `Standard`.
        #[builder(into, default)]
        pub sku_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A collection containing the availability zone to allocate the Public IP in. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Availability Zones are only supported with a [Standard SKU](https://docs.microsoft.com/azure/virtual-network/virtual-network-ip-addresses-overview-arm#standard) and [in select regions](https://docs.microsoft.com/azure/availability-zones/az-overview) at this time. Standard SKU Public IP Addresses that do not specify a zone are **not** zone-redundant by default.
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct PublicIpResult {
        /// Defines the allocation method for this IP address. Possible values are `Static` or `Dynamic`.
        ///
        /// > **Note** `Dynamic` Public IP Addresses aren't allocated until they're assigned to a resource (such as a Virtual Machine or a Load Balancer) by design within Azure. See `ip_address` argument.
        pub allocation_method: pulumi_gestalt_rust::Output<String>,
        /// The DDoS protection mode of the public IP. Possible values are `Disabled`, `Enabled`, and `VirtualNetworkInherited`. Defaults to `VirtualNetworkInherited`.
        pub ddos_protection_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of DDoS protection plan associated with the public IP.
        ///
        /// > **Note:** `ddos_protection_plan_id` can only be set when `ddos_protection_mode` is `Enabled`.
        pub ddos_protection_plan_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Label for the Domain Name. Will be used to make up the FQDN. If a domain name label is specified, an A DNS record is created for the public IP in the Microsoft Azure DNS system.
        pub domain_name_label: pulumi_gestalt_rust::Output<Option<String>>,
        /// Scope for the domain name label. If a domain name label scope is specified, an A DNS record is created for the public IP in the Microsoft Azure DNS system with a hashed value includes in FQDN. Possible values are `NoReuse`, `ResourceGroupReuse`, `SubscriptionReuse` and `TenantReuse`. Changing this forces a new Public IP to be created.
        pub domain_name_label_scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Edge Zone within the Azure Region where this Public IP should exist. Changing this forces a new Public IP to be created.
        pub edge_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// Fully qualified domain name of the A DNS record associated with the public IP. `domain_name_label` must be specified to get the `fqdn`. This is the concatenation of the `domain_name_label` and the regionalized DNS zone
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the timeout for the TCP idle connection. The value can be set between 4 and 30 minutes.
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The IP address value that was allocated.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// A mapping of IP tags to assign to the public IP. Changing this forces a new resource to be created.
        ///
        /// > **Note** IP Tag `RoutingPreference` requires multiple `zones` and `Standard` SKU to be set.
        pub ip_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The IP Version to use, IPv6 or IPv4. Changing this forces a new resource to be created. Defaults to `IPv4`.
        ///
        /// > **Note** Only `static` IP address allocation is supported for IPv6.
        pub ip_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the Public IP should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Public IP. Changing this forces a new Public IP to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// If specified then public IP address allocated will be provided from the public IP prefix resource. Changing this forces a new resource to be created.
        pub public_ip_prefix_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group where this Public IP should exist. Changing this forces a new Public IP to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A fully qualified domain name that resolves to this public IP address. If the reverseFqdn is specified, then a PTR DNS record is created pointing from the IP address in the in-addr.arpa domain to the reverse FQDN.
        pub reverse_fqdn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The SKU of the Public IP. Accepted values are `Basic` and `Standard`. Defaults to `Standard`. Changing this forces a new resource to be created.
        ///
        /// > **Note** Public IP Standard SKUs require `allocation_method` to be set to `Static`.
        pub sku: pulumi_gestalt_rust::Output<Option<String>>,
        /// The SKU Tier that should be used for the Public IP. Possible values are `Regional` and `Global`. Defaults to `Regional`. Changing this forces a new resource to be created.
        ///
        /// > **Note** When `sku_tier` is set to `Global`, `sku` must be set to `Standard`.
        pub sku_tier: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A collection containing the availability zone to allocate the Public IP in. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Availability Zones are only supported with a [Standard SKU](https://docs.microsoft.com/azure/virtual-network/virtual-network-ip-addresses-overview-arm#standard) and [in select regions](https://docs.microsoft.com/azure/availability-zones/az-overview) at this time. Standard SKU Public IP Addresses that do not specify a zone are **not** zone-redundant by default.
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PublicIpArgs,
    ) -> PublicIpResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allocation_method_binding = args.allocation_method.get_output(context);
        let ddos_protection_mode_binding = args.ddos_protection_mode.get_output(context);
        let ddos_protection_plan_id_binding = args
            .ddos_protection_plan_id
            .get_output(context);
        let domain_name_label_binding = args.domain_name_label.get_output(context);
        let domain_name_label_scope_binding = args
            .domain_name_label_scope
            .get_output(context);
        let edge_zone_binding = args.edge_zone.get_output(context);
        let idle_timeout_in_minutes_binding = args
            .idle_timeout_in_minutes
            .get_output(context);
        let ip_tags_binding = args.ip_tags.get_output(context);
        let ip_version_binding = args.ip_version.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_ip_prefix_id_binding = args.public_ip_prefix_id.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let reverse_fqdn_binding = args.reverse_fqdn.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let sku_tier_binding = args.sku_tier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/publicIp:PublicIp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allocationMethod".into(),
                    value: &allocation_method_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ddosProtectionMode".into(),
                    value: &ddos_protection_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ddosProtectionPlanId".into(),
                    value: &ddos_protection_plan_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainNameLabel".into(),
                    value: &domain_name_label_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainNameLabelScope".into(),
                    value: &domain_name_label_scope_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idleTimeoutInMinutes".into(),
                    value: &idle_timeout_in_minutes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipTags".into(),
                    value: &ip_tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipVersion".into(),
                    value: &ip_version_binding.drop_type(),
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
                    name: "publicIpPrefixId".into(),
                    value: &public_ip_prefix_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reverseFqdn".into(),
                    value: &reverse_fqdn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuTier".into(),
                    value: &sku_tier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: &zones_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PublicIpResult {
            allocation_method: o.get_field("allocationMethod"),
            ddos_protection_mode: o.get_field("ddosProtectionMode"),
            ddos_protection_plan_id: o.get_field("ddosProtectionPlanId"),
            domain_name_label: o.get_field("domainNameLabel"),
            domain_name_label_scope: o.get_field("domainNameLabelScope"),
            edge_zone: o.get_field("edgeZone"),
            fqdn: o.get_field("fqdn"),
            idle_timeout_in_minutes: o.get_field("idleTimeoutInMinutes"),
            ip_address: o.get_field("ipAddress"),
            ip_tags: o.get_field("ipTags"),
            ip_version: o.get_field("ipVersion"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            public_ip_prefix_id: o.get_field("publicIpPrefixId"),
            resource_group_name: o.get_field("resourceGroupName"),
            reverse_fqdn: o.get_field("reverseFqdn"),
            sku: o.get_field("sku"),
            sku_tier: o.get_field("skuTier"),
            tags: o.get_field("tags"),
            zones: o.get_field("zones"),
        }
    }
}
