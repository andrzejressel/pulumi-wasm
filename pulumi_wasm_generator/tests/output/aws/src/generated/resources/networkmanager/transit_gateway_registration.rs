/// Registers a transit gateway to a global network. The transit gateway can be in any AWS Region,
/// but it must be owned by the same AWS account that owns the global network.
/// You cannot register a transit gateway in more than one global network.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = global_network::create(
///         "example",
///         GlobalNetworkArgs::builder().description("example").build_struct(),
///     );
///     let exampleTransitGateway = transit_gateway::create(
///         "exampleTransitGateway",
///         TransitGatewayArgs::builder().build_struct(),
///     );
///     let exampleTransitGatewayRegistration = transit_gateway_registration::create(
///         "exampleTransitGatewayRegistration",
///         TransitGatewayRegistrationArgs::builder()
///             .global_network_id("${example.id}")
///             .transit_gateway_arn("${exampleTransitGateway.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_transit_gateway_registration` using the global network ID and transit gateway ARN. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/transitGatewayRegistration:TransitGatewayRegistration example global-network-0d47f6t230mz46dy4,arn:aws:ec2:us-west-2:123456789012:transit-gateway/tgw-123abc05e04123abc
/// ```
pub mod transit_gateway_registration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransitGatewayRegistrationArgs {
        /// The ID of the Global Network to register to.
        #[builder(into)]
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the Transit Gateway to register.
        #[builder(into)]
        pub transit_gateway_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TransitGatewayRegistrationResult {
        /// The ID of the Global Network to register to.
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the Transit Gateway to register.
        pub transit_gateway_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TransitGatewayRegistrationArgs,
    ) -> TransitGatewayRegistrationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let global_network_id_binding = args.global_network_id.get_inner();
        let transit_gateway_arn_binding = args.transit_gateway_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/transitGatewayRegistration:TransitGatewayRegistration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "globalNetworkId".into(),
                    value: &global_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayArn".into(),
                    value: &transit_gateway_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "globalNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TransitGatewayRegistrationResult {
            global_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalNetworkId").unwrap(),
            ),
            transit_gateway_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayArn").unwrap(),
            ),
        }
    }
}