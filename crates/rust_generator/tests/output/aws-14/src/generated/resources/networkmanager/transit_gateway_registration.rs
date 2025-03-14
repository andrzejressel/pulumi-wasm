/// Registers a transit gateway to a global network. The transit gateway can be in any AWS Region,
/// but it must be owned by the same AWS account that owns the global network.
/// You cannot register a transit gateway in more than one global network.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod transit_gateway_registration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransitGatewayRegistrationArgs {
        /// The ID of the Global Network to register to.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the Transit Gateway to register.
        #[builder(into)]
        pub transit_gateway_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TransitGatewayRegistrationResult {
        /// The ID of the Global Network to register to.
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the Transit Gateway to register.
        pub transit_gateway_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TransitGatewayRegistrationArgs,
    ) -> TransitGatewayRegistrationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let global_network_id_binding = args.global_network_id.get_output(context);
        let transit_gateway_arn_binding = args.transit_gateway_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmanager/transitGatewayRegistration:TransitGatewayRegistration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalNetworkId".into(),
                    value: &global_network_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayArn".into(),
                    value: &transit_gateway_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TransitGatewayRegistrationResult {
            global_network_id: o.get_field("globalNetworkId"),
            transit_gateway_arn: o.get_field("transitGatewayArn"),
        }
    }
}
