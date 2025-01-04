pub mod get_nat_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNatGatewayArgs {
        /// Specifies the Name of the NAT Gateway.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of existing Public IP Address resource IDs which the NAT Gateway is using.
        #[builder(into, default)]
        pub public_ip_address_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of existing Public IP Prefix resource IDs which the NAT Gateway is using.
        #[builder(into, default)]
        pub public_ip_prefix_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the name of the Resource Group where the NAT Gateway exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetNatGatewayResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The idle timeout in minutes which is used for the NAT Gateway.
        pub idle_timeout_in_minutes: pulumi_wasm_rust::Output<i32>,
        /// The location where the NAT Gateway exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of existing Public IP Address resource IDs which the NAT Gateway is using.
        pub public_ip_address_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of existing Public IP Prefix resource IDs which the NAT Gateway is using.
        pub public_ip_prefix_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Resource GUID of the NAT Gateway.
        pub resource_guid: pulumi_wasm_rust::Output<String>,
        /// The SKU used by the NAT Gateway.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of Availability Zones which the NAT Gateway exists in.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetNatGatewayArgs) -> GetNatGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let public_ip_address_ids_binding = args.public_ip_address_ids.get_inner();
        let public_ip_prefix_ids_binding = args.public_ip_prefix_ids.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getNatGateway:getNatGateway".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicIpAddressIds".into(),
                    value: &public_ip_address_ids_binding,
                },
                register_interface::ObjectField {
                    name: "publicIpPrefixIds".into(),
                    value: &public_ip_prefix_ids_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "idleTimeoutInMinutes".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicIpAddressIds".into(),
                },
                register_interface::ResultField {
                    name: "publicIpPrefixIds".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGuid".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
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
        GetNatGatewayResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            idle_timeout_in_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleTimeoutInMinutes").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_ip_address_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpAddressIds").unwrap(),
            ),
            public_ip_prefix_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpPrefixIds").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            resource_guid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGuid").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
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
