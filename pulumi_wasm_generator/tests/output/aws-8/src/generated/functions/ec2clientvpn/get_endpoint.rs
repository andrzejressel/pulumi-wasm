pub mod get_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEndpointArgs {
        /// ID of the Client VPN endpoint.
        #[builder(into, default)]
        pub client_vpn_endpoint_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2clientvpn::GetEndpointFilter>>,
        >,
        /// Map of tags, each pair of which must exactly match a pair on the desired endpoint.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetEndpointResult {
        /// The ARN of the Client VPN endpoint.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Information about the authentication method used by the Client VPN endpoint.
        pub authentication_options: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ec2clientvpn::GetEndpointAuthenticationOption,
            >,
        >,
        /// IPv4 address range, in CIDR notation, from which client IP addresses are assigned.
        pub client_cidr_block: pulumi_wasm_rust::Output<String>,
        /// The options for managing connection authorization for new client connections.
        pub client_connect_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2clientvpn::GetEndpointClientConnectOption>,
        >,
        /// Options for enabling a customizable text banner that will be displayed on AWS provided clients when a VPN session is established.
        pub client_login_banner_options: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ec2clientvpn::GetEndpointClientLoginBannerOption,
            >,
        >,
        pub client_vpn_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// Information about the client connection logging options for the Client VPN endpoint.
        pub connection_log_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2clientvpn::GetEndpointConnectionLogOption>,
        >,
        /// Brief description of the endpoint.
        pub description: pulumi_wasm_rust::Output<String>,
        /// DNS name to be used by clients when connecting to the Client VPN endpoint.
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// Information about the DNS servers to be used for DNS resolution.
        pub dns_servers: pulumi_wasm_rust::Output<Vec<String>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2clientvpn::GetEndpointFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// IDs of the security groups for the target network associated with the Client VPN endpoint.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether the self-service portal for the Client VPN endpoint is enabled.
        pub self_service_portal: pulumi_wasm_rust::Output<String>,
        /// The URL of the self-service portal.
        pub self_service_portal_url: pulumi_wasm_rust::Output<String>,
        /// The ARN of the server certificate.
        pub server_certificate_arn: pulumi_wasm_rust::Output<String>,
        /// The maximum VPN session duration time in hours.
        pub session_timeout_hours: pulumi_wasm_rust::Output<i32>,
        /// Whether split-tunnel is enabled in the AWS Client VPN endpoint.
        pub split_tunnel: pulumi_wasm_rust::Output<bool>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Transport protocol used by the Client VPN endpoint.
        pub transport_protocol: pulumi_wasm_rust::Output<String>,
        /// ID of the VPC associated with the Client VPN endpoint.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// Port number for the Client VPN endpoint.
        pub vpn_port: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetEndpointArgs,
    ) -> GetEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
                    name: "clientVpnEndpointId".into(),
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
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEndpointResult {
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
            client_vpn_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientVpnEndpointId").unwrap(),
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
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
