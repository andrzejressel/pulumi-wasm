/// Provides network associations for AWS Client VPN endpoints. For more information on usage, please see the
/// [AWS Client VPN Administrator's Guide](https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/what-is.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = network_association::create(
///         "example",
///         NetworkAssociationArgs::builder()
///             .client_vpn_endpoint_id("${exampleAwsEc2ClientVpnEndpoint.id}")
///             .subnet_id("${exampleAwsSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Client VPN network associations using the endpoint ID and the association ID. Values are separated by a `,`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2clientvpn/networkAssociation:NetworkAssociation example cvpn-endpoint-0ac3a1abbccddd666,cvpn-assoc-0b8db902465d069ad
/// ```
pub mod network_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkAssociationArgs {
        /// The ID of the Client VPN endpoint.
        #[builder(into)]
        pub client_vpn_endpoint_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the subnet to associate with the Client VPN endpoint.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkAssociationResult {
        /// The unique ID of the target network association.
        pub association_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Client VPN endpoint.
        pub client_vpn_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the subnet to associate with the Client VPN endpoint.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPC in which the target subnet is located.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NetworkAssociationArgs,
    ) -> NetworkAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_vpn_endpoint_id_binding = args
            .client_vpn_endpoint_id
            .get_output(context)
            .get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2clientvpn/networkAssociation:NetworkAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientVpnEndpointId".into(),
                    value: &client_vpn_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkAssociationResult {
            association_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("associationId"),
            ),
            client_vpn_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientVpnEndpointId"),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
