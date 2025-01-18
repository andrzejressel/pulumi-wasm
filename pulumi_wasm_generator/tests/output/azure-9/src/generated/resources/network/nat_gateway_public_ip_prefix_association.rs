/// Manages the association between a NAT Gateway and a Public IP Prefix.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleNatGateway = nat_gateway::create(
///         "exampleNatGateway",
///         NatGatewayArgs::builder()
///             .location("${example.location}")
///             .name("example-NatGateway")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard")
///             .build_struct(),
///     );
///     let exampleNatGatewayPublicIpPrefixAssociation = nat_gateway_public_ip_prefix_association::create(
///         "exampleNatGatewayPublicIpPrefixAssociation",
///         NatGatewayPublicIpPrefixAssociationArgs::builder()
///             .nat_gateway_id("${exampleNatGateway.id}")
///             .public_ip_prefix_id("${examplePublicIpPrefix.id}")
///             .build_struct(),
///     );
///     let examplePublicIpPrefix = public_ip_prefix::create(
///         "examplePublicIpPrefix",
///         PublicIpPrefixArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .prefix_length(30)
///             .resource_group_name("${example.name}")
///             .zones(vec!["1",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Associations between NAT Gateway and Public IP Prefixes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/natGatewayPublicIpPrefixAssociation:NatGatewayPublicIpPrefixAssociation example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/natGateways/gateway1|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/publicIPPrefixes/myPublicIpPrefix1"
/// ```
///
pub mod nat_gateway_public_ip_prefix_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NatGatewayPublicIpPrefixAssociationArgs {
        /// The ID of the NAT Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub nat_gateway_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Public IP Prefix which this NAT Gateway which should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub public_ip_prefix_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NatGatewayPublicIpPrefixAssociationResult {
        /// The ID of the NAT Gateway. Changing this forces a new resource to be created.
        pub nat_gateway_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Public IP Prefix which this NAT Gateway which should be connected to. Changing this forces a new resource to be created.
        pub public_ip_prefix_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NatGatewayPublicIpPrefixAssociationArgs,
    ) -> NatGatewayPublicIpPrefixAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let nat_gateway_id_binding = args.nat_gateway_id.get_inner();
        let public_ip_prefix_id_binding = args.public_ip_prefix_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/natGatewayPublicIpPrefixAssociation:NatGatewayPublicIpPrefixAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "natGatewayId".into(),
                    value: &nat_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "publicIpPrefixId".into(),
                    value: &public_ip_prefix_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "natGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "publicIpPrefixId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NatGatewayPublicIpPrefixAssociationResult {
            nat_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("natGatewayId").unwrap(),
            ),
            public_ip_prefix_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpPrefixId").unwrap(),
            ),
        }
    }
}
