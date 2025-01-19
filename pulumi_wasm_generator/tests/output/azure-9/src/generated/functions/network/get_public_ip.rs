pub mod get_public_ip {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPublicIpArgs {
        /// Specifies the name of the public IP address.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetPublicIpResult {
        /// The allocation method for this IP address. Possible values are `Static` or `Dynamic`.
        pub allocation_method: pulumi_wasm_rust::Output<String>,
        /// The DDoS protection mode of the public IP.
        pub ddos_protection_mode: pulumi_wasm_rust::Output<String>,
        /// The ID of DDoS protection plan associated with the public IP.
        pub ddos_protection_plan_id: pulumi_wasm_rust::Output<String>,
        /// The label for the Domain Name.
        pub domain_name_label: pulumi_wasm_rust::Output<String>,
        /// Fully qualified domain name of the A DNS record associated with the public IP. This is the concatenation of the domainNameLabel and the regionalized DNS zone.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Specifies the timeout for the TCP idle connection.
        pub idle_timeout_in_minutes: pulumi_wasm_rust::Output<i32>,
        /// The IP address value that was allocated.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assigned to the resource.
        pub ip_tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The IP version being used, for example `IPv4` or `IPv6`.
        pub ip_version: pulumi_wasm_rust::Output<String>,
        /// The region that this public ip exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The fully qualified domain name that resolves to this public IP address.
        pub reverse_fqdn: pulumi_wasm_rust::Output<String>,
        /// The SKU of the Public IP.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of Availability Zones in which this Public IP is located.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPublicIpArgs) -> GetPublicIpResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getPublicIP:getPublicIP".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
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
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "reverseFqdn".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPublicIpResult {
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
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            reverse_fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reverseFqdn").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
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
