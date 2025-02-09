/// Associates a customer gateway with a device and optionally, with a link.
/// If you specify a link, it must be associated with the specified device.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkmanager:GlobalNetwork
///     properties:
///       description: example
///   exampleSite:
///     type: aws:networkmanager:Site
///     name: example
///     properties:
///       globalNetworkId: ${example.id}
///   exampleDevice:
///     type: aws:networkmanager:Device
///     name: example
///     properties:
///       globalNetworkId: ${example.id}
///       siteId: ${exampleSite.id}
///   exampleCustomerGateway:
///     type: aws:ec2:CustomerGateway
///     name: example
///     properties:
///       bgpAsn: 65000
///       ipAddress: 172.83.124.10
///       type: ipsec.1
///   exampleTransitGateway:
///     type: aws:ec2transitgateway:TransitGateway
///     name: example
///   exampleVpnConnection:
///     type: aws:ec2:VpnConnection
///     name: example
///     properties:
///       customerGatewayId: ${exampleCustomerGateway.id}
///       transitGatewayId: ${exampleTransitGateway.id}
///       type: ${exampleCustomerGateway.type}
///       staticRoutesOnly: true
///   exampleTransitGatewayRegistration:
///     type: aws:networkmanager:TransitGatewayRegistration
///     name: example
///     properties:
///       globalNetworkId: ${example.id}
///       transitGatewayArn: ${exampleTransitGateway.arn}
///     options:
///       dependsOn:
///         - ${exampleVpnConnection}
///   exampleCustomerGatewayAssociation:
///     type: aws:networkmanager:CustomerGatewayAssociation
///     name: example
///     properties:
///       globalNetworkId: ${example.id}
///       customerGatewayArn: ${exampleCustomerGateway.arn}
///       deviceId: ${exampleDevice.id}
///     options:
///       dependsOn:
///         - ${exampleTransitGatewayRegistration}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_customer_gateway_association` using the global network ID and customer gateway ARN. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/customerGatewayAssociation:CustomerGatewayAssociation example global-network-0d47f6t230mz46dy4,arn:aws:ec2:us-west-2:123456789012:customer-gateway/cgw-123abc05e04123abc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod customer_gateway_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomerGatewayAssociationArgs {
        /// The Amazon Resource Name (ARN) of the customer gateway.
        #[builder(into)]
        pub customer_gateway_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the device.
        #[builder(into)]
        pub device_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the global network.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the link.
        #[builder(into, default)]
        pub link_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CustomerGatewayAssociationResult {
        /// The Amazon Resource Name (ARN) of the customer gateway.
        pub customer_gateway_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the device.
        pub device_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the global network.
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the link.
        pub link_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomerGatewayAssociationArgs,
    ) -> CustomerGatewayAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let customer_gateway_arn_binding = args.customer_gateway_arn.get_output(context);
        let device_id_binding = args.device_id.get_output(context);
        let global_network_id_binding = args.global_network_id.get_output(context);
        let link_id_binding = args.link_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmanager/customerGatewayAssociation:CustomerGatewayAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerGatewayArn".into(),
                    value: customer_gateway_arn_binding.get_id(),
                },
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
            ],
        };
        let o = context.register_resource(request);
        CustomerGatewayAssociationResult {
            customer_gateway_arn: o.get_field("customerGatewayArn"),
            device_id: o.get_field("deviceId"),
            global_network_id: o.get_field("globalNetworkId"),
            link_id: o.get_field("linkId"),
        }
    }
}
