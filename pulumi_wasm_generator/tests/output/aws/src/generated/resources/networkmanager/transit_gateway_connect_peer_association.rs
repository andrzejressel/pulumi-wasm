/// Associates a transit gateway Connect peer with a device, and optionally, with a link.
/// If you specify a link, it must be associated with the specified device.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod transit_gateway_connect_peer_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransitGatewayConnectPeerAssociationArgs {
        /// The ID of the device.
        #[builder(into)]
        pub device_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the global network.
        #[builder(into)]
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the link.
        #[builder(into, default)]
        pub link_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Connect peer.
        #[builder(into)]
        pub transit_gateway_connect_peer_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TransitGatewayConnectPeerAssociationResult {
        /// The ID of the device.
        pub device_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the global network.
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the link.
        pub link_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Connect peer.
        pub transit_gateway_connect_peer_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TransitGatewayConnectPeerAssociationArgs,
    ) -> TransitGatewayConnectPeerAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let device_id_binding = args.device_id.get_inner();
        let global_network_id_binding = args.global_network_id.get_inner();
        let link_id_binding = args.link_id.get_inner();
        let transit_gateway_connect_peer_arn_binding = args
            .transit_gateway_connect_peer_arn
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/transitGatewayConnectPeerAssociation:TransitGatewayConnectPeerAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deviceId".into(),
                    value: &device_id_binding,
                },
                register_interface::ObjectField {
                    name: "globalNetworkId".into(),
                    value: &global_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "linkId".into(),
                    value: &link_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayConnectPeerArn".into(),
                    value: &transit_gateway_connect_peer_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "deviceId".into(),
                },
                register_interface::ResultField {
                    name: "globalNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "linkId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayConnectPeerArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TransitGatewayConnectPeerAssociationResult {
            device_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceId").unwrap(),
            ),
            global_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalNetworkId").unwrap(),
            ),
            link_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkId").unwrap(),
            ),
            transit_gateway_connect_peer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayConnectPeerArn").unwrap(),
            ),
        }
    }
}
