/// Associates a transit gateway Connect peer with a device, and optionally, with a link.
/// If you specify a link, it must be associated with the specified device.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = transit_gateway_connect_peer_association::create(
///         "example",
///         TransitGatewayConnectPeerAssociationArgs::builder()
///             .device_id("${exampleAwsNetworkmanagerDevice.id}")
///             .global_network_id("${exampleAwsNetworkmanagerGlobalNetwork.id}")
///             .transit_gateway_connect_peer_arn(
///                 "${exampleAwsEc2TransitGatewayConnectPeer.arn}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_transit_gateway_connect_peer_association` using the global network ID and customer gateway ARN. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/transitGatewayConnectPeerAssociation:TransitGatewayConnectPeerAssociation example global-network-0d47f6t230mz46dy4,arn:aws:ec2:us-west-2:123456789012:transit-gateway-connect-peer/tgw-connect-peer-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod transit_gateway_connect_peer_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransitGatewayConnectPeerAssociationArgs {
        /// The ID of the device.
        #[builder(into)]
        pub device_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the global network.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the link.
        #[builder(into, default)]
        pub link_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Connect peer.
        #[builder(into)]
        pub transit_gateway_connect_peer_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TransitGatewayConnectPeerAssociationResult {
        /// The ID of the device.
        pub device_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the global network.
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the link.
        pub link_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Connect peer.
        pub transit_gateway_connect_peer_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TransitGatewayConnectPeerAssociationArgs,
    ) -> TransitGatewayConnectPeerAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let device_id_binding = args.device_id.get_output(context);
        let global_network_id_binding = args.global_network_id.get_output(context);
        let link_id_binding = args.link_id.get_output(context);
        let transit_gateway_connect_peer_arn_binding = args
            .transit_gateway_connect_peer_arn
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmanager/transitGatewayConnectPeerAssociation:TransitGatewayConnectPeerAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceId".into(),
                    value: device_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalNetworkId".into(),
                    value: global_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkId".into(),
                    value: link_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayConnectPeerArn".into(),
                    value: transit_gateway_connect_peer_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TransitGatewayConnectPeerAssociationResult {
            device_id: o.get_field("deviceId"),
            global_network_id: o.get_field("globalNetworkId"),
            link_id: o.get_field("linkId"),
            transit_gateway_connect_peer_arn: o.get_field("transitGatewayConnectPeerArn"),
        }
    }
}
