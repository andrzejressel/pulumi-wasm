/// Provides an AWS Client VPN endpoint for OpenVPN clients. For more information on usage, please see the
/// [AWS Client VPN Administrator's Guide](https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/what-is.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = endpoint::create(
///         "example",
///         EndpointArgs::builder()
///             .authentication_options(
///                 vec![
///                     EndpointAuthenticationOption::builder()
///                     .rootCertificateChainArn("${rootCert.arn}"). type
///                     ("certificate-authentication").build_struct(),
///                 ],
///             )
///             .client_cidr_block("10.0.0.0/16")
///             .connection_log_options(
///                 EndpointConnectionLogOptions::builder()
///                     .cloudwatchLogGroup("${lg.name}")
///                     .cloudwatchLogStream("${ls.name}")
///                     .enabled(true)
///                     .build_struct(),
///             )
///             .description("clientvpn-example")
///             .server_certificate_arn("${cert.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Client VPN endpoints using the `id` value found via `aws ec2 describe-client-vpn-endpoints`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2clientvpn/endpoint:Endpoint example cvpn-endpoint-0ac3a1abbccddd666
/// ```
pub mod endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// Information about the authentication method to be used to authenticate clients.
        #[builder(into)]
        pub authentication_options: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2clientvpn::EndpointAuthenticationOption>,
        >,
        /// The IPv4 address range, in CIDR notation, from which to assign client IP addresses. The address range cannot overlap with the local CIDR of the VPC in which the associated subnet is located, or the routes that you add manually. The address range cannot be changed after the Client VPN endpoint has been created. The CIDR block should be /22 or greater.
        #[builder(into)]
        pub client_cidr_block: pulumi_wasm_rust::Output<String>,
        /// The options for managing connection authorization for new client connections.
        #[builder(into, default)]
        pub client_connect_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2clientvpn::EndpointClientConnectOptions>,
        >,
        /// Options for enabling a customizable text banner that will be displayed on AWS provided clients when a VPN session is established.
        #[builder(into, default)]
        pub client_login_banner_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2clientvpn::EndpointClientLoginBannerOptions>,
        >,
        /// Information about the client connection logging options.
        #[builder(into)]
        pub connection_log_options: pulumi_wasm_rust::Output<
            super::super::types::ec2clientvpn::EndpointConnectionLogOptions,
        >,
        /// A brief description of the Client VPN endpoint.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Information about the DNS servers to be used for DNS resolution. A Client VPN endpoint can have up to two DNS servers. If no DNS server is specified, the DNS address of the connecting device is used.
        #[builder(into, default)]
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The IDs of one or more security groups to apply to the target network. You must also specify the ID of the VPC that contains the security groups.
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specify whether to enable the self-service portal for the Client VPN endpoint. Values can be `enabled` or `disabled`. Default value is `disabled`.
        #[builder(into, default)]
        pub self_service_portal: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the ACM server certificate.
        #[builder(into)]
        pub server_certificate_arn: pulumi_wasm_rust::Output<String>,
        /// The maximum session duration is a trigger by which end-users are required to re-authenticate prior to establishing a VPN session. Default value is `24` - Valid values: `8 | 10 | 12 | 24`
        #[builder(into, default)]
        pub session_timeout_hours: pulumi_wasm_rust::Output<Option<i32>>,
        /// Indicates whether split-tunnel is enabled on VPN endpoint. Default value is `false`.
        #[builder(into, default)]
        pub split_tunnel: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The transport protocol to be used by the VPN session. Default value is `udp`.
        #[builder(into, default)]
        pub transport_protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the VPC to associate with the Client VPN endpoint. If no security group IDs are specified in the request, the default security group for the VPC is applied.
        #[builder(into, default)]
        pub vpc_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The port number for the Client VPN endpoint. Valid values are `443` and `1194`. Default value is `443`.
        #[builder(into, default)]
        pub vpn_port: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// The ARN of the Client VPN endpoint.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Information about the authentication method to be used to authenticate clients.
        pub authentication_options: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2clientvpn::EndpointAuthenticationOption>,
        >,
        /// The IPv4 address range, in CIDR notation, from which to assign client IP addresses. The address range cannot overlap with the local CIDR of the VPC in which the associated subnet is located, or the routes that you add manually. The address range cannot be changed after the Client VPN endpoint has been created. The CIDR block should be /22 or greater.
        pub client_cidr_block: pulumi_wasm_rust::Output<String>,
        /// The options for managing connection authorization for new client connections.
        pub client_connect_options: pulumi_wasm_rust::Output<
            super::super::types::ec2clientvpn::EndpointClientConnectOptions,
        >,
        /// Options for enabling a customizable text banner that will be displayed on AWS provided clients when a VPN session is established.
        pub client_login_banner_options: pulumi_wasm_rust::Output<
            super::super::types::ec2clientvpn::EndpointClientLoginBannerOptions,
        >,
        /// Information about the client connection logging options.
        pub connection_log_options: pulumi_wasm_rust::Output<
            super::super::types::ec2clientvpn::EndpointConnectionLogOptions,
        >,
        /// A brief description of the Client VPN endpoint.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The DNS name to be used by clients when establishing their VPN session.
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// Information about the DNS servers to be used for DNS resolution. A Client VPN endpoint can have up to two DNS servers. If no DNS server is specified, the DNS address of the connecting device is used.
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The IDs of one or more security groups to apply to the target network. You must also specify the ID of the VPC that contains the security groups.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specify whether to enable the self-service portal for the Client VPN endpoint. Values can be `enabled` or `disabled`. Default value is `disabled`.
        pub self_service_portal: pulumi_wasm_rust::Output<Option<String>>,
        /// The URL of the self-service portal.
        pub self_service_portal_url: pulumi_wasm_rust::Output<String>,
        /// The ARN of the ACM server certificate.
        pub server_certificate_arn: pulumi_wasm_rust::Output<String>,
        /// The maximum session duration is a trigger by which end-users are required to re-authenticate prior to establishing a VPN session. Default value is `24` - Valid values: `8 | 10 | 12 | 24`
        pub session_timeout_hours: pulumi_wasm_rust::Output<Option<i32>>,
        /// Indicates whether split-tunnel is enabled on VPN endpoint. Default value is `false`.
        pub split_tunnel: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The transport protocol to be used by the VPN session. Default value is `udp`.
        pub transport_protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the VPC to associate with the Client VPN endpoint. If no security group IDs are specified in the request, the default security group for the VPC is applied.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// The port number for the Client VPN endpoint. Valid values are `443` and `1194`. Default value is `443`.
        pub vpn_port: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EndpointArgs) -> EndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_options_binding = args.authentication_options.get_inner();
        let client_cidr_block_binding = args.client_cidr_block.get_inner();
        let client_connect_options_binding = args.client_connect_options.get_inner();
        let client_login_banner_options_binding = args
            .client_login_banner_options
            .get_inner();
        let connection_log_options_binding = args.connection_log_options.get_inner();
        let description_binding = args.description.get_inner();
        let dns_servers_binding = args.dns_servers.get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let self_service_portal_binding = args.self_service_portal.get_inner();
        let server_certificate_arn_binding = args.server_certificate_arn.get_inner();
        let session_timeout_hours_binding = args.session_timeout_hours.get_inner();
        let split_tunnel_binding = args.split_tunnel.get_inner();
        let tags_binding = args.tags.get_inner();
        let transport_protocol_binding = args.transport_protocol.get_inner();
        let vpc_id_binding = args.vpc_id.get_inner();
        let vpn_port_binding = args.vpn_port.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2clientvpn/endpoint:Endpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationOptions".into(),
                    value: &authentication_options_binding,
                },
                register_interface::ObjectField {
                    name: "clientCidrBlock".into(),
                    value: &client_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "clientConnectOptions".into(),
                    value: &client_connect_options_binding,
                },
                register_interface::ObjectField {
                    name: "clientLoginBannerOptions".into(),
                    value: &client_login_banner_options_binding,
                },
                register_interface::ObjectField {
                    name: "connectionLogOptions".into(),
                    value: &connection_log_options_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dnsServers".into(),
                    value: &dns_servers_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "selfServicePortal".into(),
                    value: &self_service_portal_binding,
                },
                register_interface::ObjectField {
                    name: "serverCertificateArn".into(),
                    value: &server_certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "sessionTimeoutHours".into(),
                    value: &session_timeout_hours_binding,
                },
                register_interface::ObjectField {
                    name: "splitTunnel".into(),
                    value: &split_tunnel_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transportProtocol".into(),
                    value: &transport_protocol_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpnPort".into(),
                    value: &vpn_port_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authenticationOptions".into(),
                },
                register_interface::ResultField {
                    name: "clientCidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "clientConnectOptions".into(),
                },
                register_interface::ResultField {
                    name: "clientLoginBannerOptions".into(),
                },
                register_interface::ResultField {
                    name: "connectionLogOptions".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "dnsServers".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "selfServicePortal".into(),
                },
                register_interface::ResultField {
                    name: "selfServicePortalUrl".into(),
                },
                register_interface::ResultField {
                    name: "serverCertificateArn".into(),
                },
                register_interface::ResultField {
                    name: "sessionTimeoutHours".into(),
                },
                register_interface::ResultField {
                    name: "splitTunnel".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "transportProtocol".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
                register_interface::ResultField {
                    name: "vpnPort".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EndpointResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authentication_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationOptions").unwrap(),
            ),
            client_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientCidrBlock").unwrap(),
            ),
            client_connect_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientConnectOptions").unwrap(),
            ),
            client_login_banner_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientLoginBannerOptions").unwrap(),
            ),
            connection_log_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionLogOptions").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsServers").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            self_service_portal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfServicePortal").unwrap(),
            ),
            self_service_portal_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfServicePortalUrl").unwrap(),
            ),
            server_certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverCertificateArn").unwrap(),
            ),
            session_timeout_hours: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionTimeoutHours").unwrap(),
            ),
            split_tunnel: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("splitTunnel").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            transport_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transportProtocol").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
            vpn_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnPort").unwrap(),
            ),
        }
    }
}
