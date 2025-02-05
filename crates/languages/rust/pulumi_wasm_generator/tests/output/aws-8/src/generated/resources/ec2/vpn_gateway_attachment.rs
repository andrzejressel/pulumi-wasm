/// Provides a Virtual Private Gateway attachment resource, allowing for an existing
/// hardware VPN gateway to be attached and/or detached from a VPC.
///
/// > **Note:** The `aws.ec2.VpnGateway`
/// resource can also automatically attach the Virtual Private Gateway it creates
/// to an existing VPC by setting the `vpc_id` attribute accordingly.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   network:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.0.0.0/16
///   vpn:
///     type: aws:ec2:VpnGateway
///     properties:
///       tags:
///         Name: example-vpn-gateway
///   vpnAttachment:
///     type: aws:ec2:VpnGatewayAttachment
///     name: vpn_attachment
///     properties:
///       vpcId: ${network.id}
///       vpnGatewayId: ${vpn.id}
/// ```
///
/// See [Virtual Private Cloud](http://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/VPC_Introduction.html)
/// and [Virtual Private Gateway](http://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/VPC_VPN.html) user
/// guides for more information.
///
/// ## Import
///
/// You cannot import this resource.
///
pub mod vpn_gateway_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnGatewayAttachmentArgs {
        /// The ID of the VPC.
        #[builder(into)]
        pub vpc_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Virtual Private Gateway.
        #[builder(into)]
        pub vpn_gateway_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpnGatewayAttachmentResult {
        /// The ID of the VPC.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Private Gateway.
        pub vpn_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpnGatewayAttachmentArgs,
    ) -> VpnGatewayAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let vpn_gateway_id_binding = args.vpn_gateway_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpnGatewayAttachment:VpnGatewayAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpnGatewayId".into(),
                    value: &vpn_gateway_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpnGatewayAttachmentResult {
            vpc_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpcId")),
            vpn_gateway_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpnGatewayId"),
            ),
        }
    }
}
