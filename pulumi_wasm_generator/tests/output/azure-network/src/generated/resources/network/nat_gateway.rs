/// Manages an Azure NAT Gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("nat-gateway-example-rg")
///             .build_struct(),
///     );
///     let exampleNatGateway = nat_gateway::create(
///         "exampleNatGateway",
///         NatGatewayArgs::builder()
///             .idle_timeout_in_minutes(10)
///             .location("${example.location}")
///             .name("nat-gateway")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard")
///             .zones(vec!["1",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// For more complete examples, please see the azure.network.NatGatewayPublicIpAssociation and azure.network.NatGatewayPublicIpPrefixAssociation resources.
///
/// ## Import
///
/// NAT Gateway can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/natGateway:NatGateway test /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/natGateways/gateway1
/// ```
///
pub mod nat_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NatGatewayArgs {
        /// The idle timeout which should be used in minutes. Defaults to `4`.
        #[builder(into, default)]
        pub idle_timeout_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the supported Azure location where the NAT Gateway should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the NAT Gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group in which the NAT Gateway should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU which should be used. At this time the only supported value is `Standard`. Defaults to `Standard`.
        #[builder(into, default)]
        pub sku_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Availability Zones in which this NAT Gateway should be located. Changing this forces a new NAT Gateway to be created.
        ///
        /// > **NOTE:** Only one Availability Zone can be defined. For more information, please check out the [Azure documentation](https://learn.microsoft.com/en-us/azure/nat-gateway/nat-overview#availability-zones)
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct NatGatewayResult {
        /// The idle timeout which should be used in minutes. Defaults to `4`.
        pub idle_timeout_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the supported Azure location where the NAT Gateway should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the NAT Gateway. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group in which the NAT Gateway should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The resource GUID property of the NAT Gateway.
        pub resource_guid: pulumi_wasm_rust::Output<String>,
        /// The SKU which should be used. At this time the only supported value is `Standard`. Defaults to `Standard`.
        pub sku_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Availability Zones in which this NAT Gateway should be located. Changing this forces a new NAT Gateway to be created.
        ///
        /// > **NOTE:** Only one Availability Zone can be defined. For more information, please check out the [Azure documentation](https://learn.microsoft.com/en-us/azure/nat-gateway/nat-overview#availability-zones)
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NatGatewayArgs) -> NatGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let idle_timeout_in_minutes_binding = args.idle_timeout_in_minutes.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/natGateway:NatGateway".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "idleTimeoutInMinutes".into(),
                    value: &idle_timeout_in_minutes_binding,
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
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
                    name: "idleTimeoutInMinutes".into(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NatGatewayResult {
            idle_timeout_in_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleTimeoutInMinutes").unwrap(),
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