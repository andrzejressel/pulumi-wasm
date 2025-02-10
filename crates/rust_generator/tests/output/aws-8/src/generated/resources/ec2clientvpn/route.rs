/// Provides additional routes for AWS Client VPN endpoints. For more information on usage, please see the
/// [AWS Client VPN Administrator's Guide](https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/what-is.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = route::create(
///         "example",
///         RouteArgs::builder()
///             .client_vpn_endpoint_id("${exampleEndpoint.id}")
///             .destination_cidr_block("0.0.0.0/0")
///             .target_vpc_subnet_id("${exampleNetworkAssociation.subnetId}")
///             .build_struct(),
///     );
///     let exampleEndpoint = endpoint::create(
///         "exampleEndpoint",
///         EndpointArgs::builder()
///             .authentication_options(
///                 vec![
///                     EndpointAuthenticationOption::builder()
///                     .rootCertificateChainArn("${exampleAwsAcmCertificate.arn}"). type
///                     ("certificate-authentication").build_struct(),
///                 ],
///             )
///             .client_cidr_block("10.0.0.0/16")
///             .connection_log_options(
///                 EndpointConnectionLogOptions::builder().enabled(false).build_struct(),
///             )
///             .description("Example Client VPN endpoint")
///             .server_certificate_arn("${exampleAwsAcmCertificate.arn}")
///             .build_struct(),
///     );
///     let exampleNetworkAssociation = network_association::create(
///         "exampleNetworkAssociation",
///         NetworkAssociationArgs::builder()
///             .client_vpn_endpoint_id("${exampleEndpoint.id}")
///             .subnet_id("${exampleAwsSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Client VPN routes using the endpoint ID, target subnet ID, and destination CIDR block. All values are separated by a `,`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2clientvpn/route:Route example cvpn-endpoint-1234567890abcdef,subnet-9876543210fedcba,10.1.0.0/24
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteArgs {
        /// The ID of the Client VPN endpoint.
        #[builder(into)]
        pub client_vpn_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A brief description of the route.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IPv4 address range, in CIDR notation, of the route destination.
        #[builder(into)]
        pub destination_cidr_block: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subnet to route the traffic through. It must already be attached to the Client VPN.
        #[builder(into)]
        pub target_vpc_subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouteResult {
        /// The ID of the Client VPN endpoint.
        pub client_vpn_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// A brief description of the route.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IPv4 address range, in CIDR notation, of the route destination.
        pub destination_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// Indicates how the Client VPN route was added. Will be `add-route` for routes created by this resource.
        pub origin: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet to route the traffic through. It must already be attached to the Client VPN.
        pub target_vpc_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// The type of the route.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteArgs,
    ) -> RouteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let client_vpn_endpoint_id_binding = args
            .client_vpn_endpoint_id
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let destination_cidr_block_binding = args
            .destination_cidr_block
            .get_output(context);
        let target_vpc_subnet_id_binding = args.target_vpc_subnet_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2clientvpn/route:Route".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientVpnEndpointId".into(),
                    value: client_vpn_endpoint_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationCidrBlock".into(),
                    value: destination_cidr_block_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetVpcSubnetId".into(),
                    value: target_vpc_subnet_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouteResult {
            client_vpn_endpoint_id: o.get_field("clientVpnEndpointId"),
            description: o.get_field("description"),
            destination_cidr_block: o.get_field("destinationCidrBlock"),
            origin: o.get_field("origin"),
            target_vpc_subnet_id: o.get_field("targetVpcSubnetId"),
            type_: o.get_field("type"),
        }
    }
}
