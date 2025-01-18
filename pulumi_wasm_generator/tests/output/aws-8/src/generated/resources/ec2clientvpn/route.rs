/// Provides additional routes for AWS Client VPN endpoints. For more information on usage, please see the
/// [AWS Client VPN Administrator's Guide](https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/what-is.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteArgs {
        /// The ID of the Client VPN endpoint.
        #[builder(into)]
        pub client_vpn_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// A brief description of the route.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 address range, in CIDR notation, of the route destination.
        #[builder(into)]
        pub destination_cidr_block: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet to route the traffic through. It must already be attached to the Client VPN.
        #[builder(into)]
        pub target_vpc_subnet_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RouteResult {
        /// The ID of the Client VPN endpoint.
        pub client_vpn_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// A brief description of the route.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 address range, in CIDR notation, of the route destination.
        pub destination_cidr_block: pulumi_wasm_rust::Output<String>,
        /// Indicates how the Client VPN route was added. Will be `add-route` for routes created by this resource.
        pub origin: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet to route the traffic through. It must already be attached to the Client VPN.
        pub target_vpc_subnet_id: pulumi_wasm_rust::Output<String>,
        /// The type of the route.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RouteArgs) -> RouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_vpn_endpoint_id_binding = args.client_vpn_endpoint_id.get_inner();
        let description_binding = args.description.get_inner();
        let destination_cidr_block_binding = args.destination_cidr_block.get_inner();
        let target_vpc_subnet_id_binding = args.target_vpc_subnet_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2clientvpn/route:Route".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientVpnEndpointId".into(),
                    value: &client_vpn_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "destinationCidrBlock".into(),
                    value: &destination_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "targetVpcSubnetId".into(),
                    value: &target_vpc_subnet_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clientVpnEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "destinationCidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "origin".into(),
                },
                register_interface::ResultField {
                    name: "targetVpcSubnetId".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RouteResult {
            client_vpn_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientVpnEndpointId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            destination_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationCidrBlock").unwrap(),
            ),
            origin: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("origin").unwrap(),
            ),
            target_vpc_subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetVpcSubnetId").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
