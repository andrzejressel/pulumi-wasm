pub mod get_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEndpointArgs {
        /// ID of the Client VPN endpoint.
        #[builder(into, default)]
        pub client_vpn_endpoint_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2clientvpn::GetEndpointFilter>>,
        >,
        /// Map of tags, each pair of which must exactly match a pair on the desired endpoint.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetEndpointResult {
        /// The ARN of the Client VPN endpoint.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Information about the authentication method used by the Client VPN endpoint.
        pub authentication_options: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ec2clientvpn::GetEndpointAuthenticationOption,
            >,
        >,
        /// IPv4 address range, in CIDR notation, from which client IP addresses are assigned.
        pub client_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// The options for managing connection authorization for new client connections.
        pub client_connect_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2clientvpn::GetEndpointClientConnectOption>,
        >,
        /// Options for enabling a customizable text banner that will be displayed on AWS provided clients when a VPN session is established.
        pub client_login_banner_options: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ec2clientvpn::GetEndpointClientLoginBannerOption,
            >,
        >,
        pub client_vpn_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// Information about the client connection logging options for the Client VPN endpoint.
        pub connection_log_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2clientvpn::GetEndpointConnectionLogOption>,
        >,
        /// Brief description of the endpoint.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// DNS name to be used by clients when connecting to the Client VPN endpoint.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// Information about the DNS servers to be used for DNS resolution.
        pub dns_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2clientvpn::GetEndpointFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// IDs of the security groups for the target network associated with the Client VPN endpoint.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether the self-service portal for the Client VPN endpoint is enabled.
        pub self_service_portal: pulumi_gestalt_rust::Output<String>,
        /// The URL of the self-service portal.
        pub self_service_portal_url: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the server certificate.
        pub server_certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// The maximum VPN session duration time in hours.
        pub session_timeout_hours: pulumi_gestalt_rust::Output<i32>,
        /// Whether split-tunnel is enabled in the AWS Client VPN endpoint.
        pub split_tunnel: pulumi_gestalt_rust::Output<bool>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Transport protocol used by the Client VPN endpoint.
        pub transport_protocol: pulumi_gestalt_rust::Output<String>,
        /// ID of the VPC associated with the Client VPN endpoint.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// Port number for the Client VPN endpoint.
        pub vpn_port: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetEndpointArgs,
    ) -> GetEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let client_vpn_endpoint_id_binding = args
            .client_vpn_endpoint_id
            .get_output(context)
            .get_inner();
        let filters_binding = args.filters.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2clientvpn/getEndpoint:getEndpoint".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientVpnEndpointId".into(),
                    value: &client_vpn_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetEndpointResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            authentication_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authenticationOptions"),
            ),
            client_cidr_block: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientCidrBlock"),
            ),
            client_connect_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientConnectOptions"),
            ),
            client_login_banner_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientLoginBannerOptions"),
            ),
            client_vpn_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientVpnEndpointId"),
            ),
            connection_log_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionLogOptions"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            dns_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsServers"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            self_service_portal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfServicePortal"),
            ),
            self_service_portal_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfServicePortalUrl"),
            ),
            server_certificate_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverCertificateArn"),
            ),
            session_timeout_hours: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sessionTimeoutHours"),
            ),
            split_tunnel: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("splitTunnel"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            transport_protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transportProtocol"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcId"),
            ),
            vpn_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpnPort"),
            ),
        }
    }
}
