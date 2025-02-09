/// Provides an AWS Client VPN endpoint for OpenVPN clients. For more information on usage, please see the
/// [AWS Client VPN Administrator's Guide](https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/what-is.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// Information about the authentication method to be used to authenticate clients.
        #[builder(into)]
        pub authentication_options: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::ec2clientvpn::EndpointAuthenticationOption>,
        >,
        /// The IPv4 address range, in CIDR notation, from which to assign client IP addresses. The address range cannot overlap with the local CIDR of the VPC in which the associated subnet is located, or the routes that you add manually. The address range cannot be changed after the Client VPN endpoint has been created. The CIDR block should be /22 or greater.
        #[builder(into)]
        pub client_cidr_block: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The options for managing connection authorization for new client connections.
        #[builder(into, default)]
        pub client_connect_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2clientvpn::EndpointClientConnectOptions>,
        >,
        /// Options for enabling a customizable text banner that will be displayed on AWS provided clients when a VPN session is established.
        #[builder(into, default)]
        pub client_login_banner_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2clientvpn::EndpointClientLoginBannerOptions>,
        >,
        /// Information about the client connection logging options.
        #[builder(into)]
        pub connection_log_options: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::ec2clientvpn::EndpointConnectionLogOptions,
        >,
        /// A brief description of the Client VPN endpoint.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Information about the DNS servers to be used for DNS resolution. A Client VPN endpoint can have up to two DNS servers. If no DNS server is specified, the DNS address of the connecting device is used.
        #[builder(into, default)]
        pub dns_servers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The IDs of one or more security groups to apply to the target network. You must also specify the ID of the VPC that contains the security groups.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specify whether to enable the self-service portal for the Client VPN endpoint. Values can be `enabled` or `disabled`. Default value is `disabled`.
        #[builder(into, default)]
        pub self_service_portal: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the ACM server certificate.
        #[builder(into)]
        pub server_certificate_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The maximum session duration is a trigger by which end-users are required to re-authenticate prior to establishing a VPN session. Default value is `24` - Valid values: `8 | 10 | 12 | 24`
        #[builder(into, default)]
        pub session_timeout_hours: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Indicates whether split-tunnel is enabled on VPN endpoint. Default value is `false`.
        #[builder(into, default)]
        pub split_tunnel: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The transport protocol to be used by the VPN session. Default value is `udp`.
        #[builder(into, default)]
        pub transport_protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the VPC to associate with the Client VPN endpoint. If no security group IDs are specified in the request, the default security group for the VPC is applied.
        #[builder(into, default)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The port number for the Client VPN endpoint. Valid values are `443` and `1194`. Default value is `443`.
        #[builder(into, default)]
        pub vpn_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// The ARN of the Client VPN endpoint.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Information about the authentication method to be used to authenticate clients.
        pub authentication_options: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2clientvpn::EndpointAuthenticationOption>,
        >,
        /// The IPv4 address range, in CIDR notation, from which to assign client IP addresses. The address range cannot overlap with the local CIDR of the VPC in which the associated subnet is located, or the routes that you add manually. The address range cannot be changed after the Client VPN endpoint has been created. The CIDR block should be /22 or greater.
        pub client_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// The options for managing connection authorization for new client connections.
        pub client_connect_options: pulumi_gestalt_rust::Output<
            super::super::types::ec2clientvpn::EndpointClientConnectOptions,
        >,
        /// Options for enabling a customizable text banner that will be displayed on AWS provided clients when a VPN session is established.
        pub client_login_banner_options: pulumi_gestalt_rust::Output<
            super::super::types::ec2clientvpn::EndpointClientLoginBannerOptions,
        >,
        /// Information about the client connection logging options.
        pub connection_log_options: pulumi_gestalt_rust::Output<
            super::super::types::ec2clientvpn::EndpointConnectionLogOptions,
        >,
        /// A brief description of the Client VPN endpoint.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The DNS name to be used by clients when establishing their VPN session.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// Information about the DNS servers to be used for DNS resolution. A Client VPN endpoint can have up to two DNS servers. If no DNS server is specified, the DNS address of the connecting device is used.
        pub dns_servers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The IDs of one or more security groups to apply to the target network. You must also specify the ID of the VPC that contains the security groups.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specify whether to enable the self-service portal for the Client VPN endpoint. Values can be `enabled` or `disabled`. Default value is `disabled`.
        pub self_service_portal: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URL of the self-service portal.
        pub self_service_portal_url: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the ACM server certificate.
        pub server_certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// The maximum session duration is a trigger by which end-users are required to re-authenticate prior to establishing a VPN session. Default value is `24` - Valid values: `8 | 10 | 12 | 24`
        pub session_timeout_hours: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Indicates whether split-tunnel is enabled on VPN endpoint. Default value is `false`.
        pub split_tunnel: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The transport protocol to be used by the VPN session. Default value is `udp`.
        pub transport_protocol: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the VPC to associate with the Client VPN endpoint. If no security group IDs are specified in the request, the default security group for the VPC is applied.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// The port number for the Client VPN endpoint. Valid values are `443` and `1194`. Default value is `443`.
        pub vpn_port: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointArgs,
    ) -> EndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_options_binding = args
            .authentication_options
            .get_output(context);
        let client_cidr_block_binding = args.client_cidr_block.get_output(context);
        let client_connect_options_binding = args
            .client_connect_options
            .get_output(context);
        let client_login_banner_options_binding = args
            .client_login_banner_options
            .get_output(context);
        let connection_log_options_binding = args
            .connection_log_options
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let dns_servers_binding = args.dns_servers.get_output(context);
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let self_service_portal_binding = args.self_service_portal.get_output(context);
        let server_certificate_arn_binding = args
            .server_certificate_arn
            .get_output(context);
        let session_timeout_hours_binding = args
            .session_timeout_hours
            .get_output(context);
        let split_tunnel_binding = args.split_tunnel.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let transport_protocol_binding = args.transport_protocol.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let vpn_port_binding = args.vpn_port.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2clientvpn/endpoint:Endpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationOptions".into(),
                    value: authentication_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCidrBlock".into(),
                    value: client_cidr_block_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientConnectOptions".into(),
                    value: client_connect_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientLoginBannerOptions".into(),
                    value: client_login_banner_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionLogOptions".into(),
                    value: connection_log_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsServers".into(),
                    value: dns_servers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: security_group_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfServicePortal".into(),
                    value: self_service_portal_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverCertificateArn".into(),
                    value: server_certificate_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sessionTimeoutHours".into(),
                    value: session_timeout_hours_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "splitTunnel".into(),
                    value: split_tunnel_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transportProtocol".into(),
                    value: transport_protocol_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: vpc_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnPort".into(),
                    value: vpn_port_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointResult {
            arn: o.get_field("arn"),
            authentication_options: o.get_field("authenticationOptions"),
            client_cidr_block: o.get_field("clientCidrBlock"),
            client_connect_options: o.get_field("clientConnectOptions"),
            client_login_banner_options: o.get_field("clientLoginBannerOptions"),
            connection_log_options: o.get_field("connectionLogOptions"),
            description: o.get_field("description"),
            dns_name: o.get_field("dnsName"),
            dns_servers: o.get_field("dnsServers"),
            security_group_ids: o.get_field("securityGroupIds"),
            self_service_portal: o.get_field("selfServicePortal"),
            self_service_portal_url: o.get_field("selfServicePortalUrl"),
            server_certificate_arn: o.get_field("serverCertificateArn"),
            session_timeout_hours: o.get_field("sessionTimeoutHours"),
            split_tunnel: o.get_field("splitTunnel"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            transport_protocol: o.get_field("transportProtocol"),
            vpc_id: o.get_field("vpcId"),
            vpn_port: o.get_field("vpnPort"),
        }
    }
}
