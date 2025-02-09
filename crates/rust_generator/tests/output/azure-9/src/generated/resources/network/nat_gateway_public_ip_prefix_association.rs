/// Manages the association between a NAT Gateway and a Public IP Prefix.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod nat_gateway_public_ip_prefix_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NatGatewayPublicIpPrefixAssociationArgs {
        /// The ID of the NAT Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub nat_gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Public IP Prefix which this NAT Gateway which should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub public_ip_prefix_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NatGatewayPublicIpPrefixAssociationResult {
        /// The ID of the NAT Gateway. Changing this forces a new resource to be created.
        pub nat_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Public IP Prefix which this NAT Gateway which should be connected to. Changing this forces a new resource to be created.
        pub public_ip_prefix_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NatGatewayPublicIpPrefixAssociationArgs,
    ) -> NatGatewayPublicIpPrefixAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let nat_gateway_id_binding_1 = args.nat_gateway_id.get_output(context);
        let nat_gateway_id_binding = nat_gateway_id_binding_1.get_inner();
        let public_ip_prefix_id_binding_1 = args.public_ip_prefix_id.get_output(context);
        let public_ip_prefix_id_binding = public_ip_prefix_id_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        NatGatewayPublicIpPrefixAssociationResult {
            nat_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("natGatewayId"),
            ),
            public_ip_prefix_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicIpPrefixId"),
            ),
        }
    }
}
