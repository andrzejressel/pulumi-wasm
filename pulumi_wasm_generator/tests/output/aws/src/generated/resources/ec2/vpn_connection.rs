/// Manages a Site-to-Site VPN connection. A Site-to-Site VPN connection is an Internet Protocol security (IPsec) VPN connection between a VPC and an on-premises network.
/// Any new Site-to-Site VPN connection that you create is an [AWS VPN connection](https://docs.aws.amazon.com/vpn/latest/s2svpn/vpn-categories.html).
///
/// > **Note:** The CIDR blocks in the arguments `tunnel1_inside_cidr` and `tunnel2_inside_cidr` must have a prefix of /30 and be a part of a specific range.
/// [Read more about this in the AWS documentation](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_VpnTunnelOptionsSpecification.html).
///
/// ## Example Usage
///
/// ### EC2 Transit Gateway
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2transitgateway:TransitGateway
///   exampleCustomerGateway:
///     type: aws:ec2:CustomerGateway
///     name: example
///     properties:
///       bgpAsn: 65000
///       ipAddress: 172.0.0.1
///       type: ipsec.1
///   exampleVpnConnection:
///     type: aws:ec2:VpnConnection
///     name: example
///     properties:
///       customerGatewayId: ${exampleCustomerGateway.id}
///       transitGatewayId: ${example.id}
///       type: ${exampleCustomerGateway.type}
/// ```
///
/// ### Virtual Private Gateway
///
/// ```yaml
/// resources:
///   vpc:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.0.0.0/16
///   vpnGateway:
///     type: aws:ec2:VpnGateway
///     name: vpn_gateway
///     properties:
///       vpcId: ${vpc.id}
///   customerGateway:
///     type: aws:ec2:CustomerGateway
///     name: customer_gateway
///     properties:
///       bgpAsn: 65000
///       ipAddress: 172.0.0.1
///       type: ipsec.1
///   main:
///     type: aws:ec2:VpnConnection
///     properties:
///       vpnGatewayId: ${vpnGateway.id}
///       customerGatewayId: ${customerGateway.id}
///       type: ipsec.1
///       staticRoutesOnly: true
/// ```
///
/// ### AWS Site to Site Private VPN
///
/// ```yaml
/// resources:
///   exampleGateway:
///     type: aws:directconnect:Gateway
///     name: example
///     properties:
///       name: example_ipsec_vpn_example
///       amazonSideAsn: '64512'
///   exampleTransitGateway:
///     type: aws:ec2transitgateway:TransitGateway
///     name: example
///     properties:
///       amazonSideAsn: '64513'
///       description: example_ipsec_vpn_example
///       transitGatewayCidrBlocks:
///         - 10.0.0.0/24
///   exampleCustomerGateway:
///     type: aws:ec2:CustomerGateway
///     name: example
///     properties:
///       bgpAsn: 64514
///       ipAddress: 10.0.0.1
///       type: ipsec.1
///       tags:
///         Name: example_ipsec_vpn_example
///   exampleGatewayAssociation:
///     type: aws:directconnect:GatewayAssociation
///     name: example
///     properties:
///       dxGatewayId: ${exampleGateway.id}
///       associatedGatewayId: ${exampleTransitGateway.id}
///       allowedPrefixes:
///         - 10.0.0.0/8
///   exampleVpnConnection:
///     type: aws:ec2:VpnConnection
///     name: example
///     properties:
///       customerGatewayId: ${exampleCustomerGateway.id}
///       outsideIpAddressType: PrivateIpv4
///       transitGatewayId: ${exampleTransitGateway.id}
///       transportTransitGatewayAttachmentId: ${example.id}
///       type: ipsec.1
///       tags:
///         Name: example_ipsec_vpn_example
/// variables:
///   example:
///     fn::invoke:
///       Function: aws:ec2transitgateway:getDirectConnectGatewayAttachment
///       Arguments:
///         transitGatewayId: ${exampleTransitGateway.id}
///         dxGatewayId: ${exampleGateway.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPN Connections using the VPN connection `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpnConnection:VpnConnection testvpnconnection vpn-40f41529
/// ```
pub mod vpn_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnConnectionArgs {
        /// The ID of the customer gateway.
        #[builder(into)]
        pub customer_gateway_id: pulumi_wasm_rust::Output<String>,
        /// Indicate whether to enable acceleration for the VPN connection. Supports only EC2 Transit Gateway.
        #[builder(into, default)]
        pub enable_acceleration: pulumi_wasm_rust::Output<Option<bool>>,
        /// The IPv4 CIDR on the customer gateway (on-premises) side of the VPN connection.
        #[builder(into, default)]
        pub local_ipv4_network_cidr: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv6 CIDR on the customer gateway (on-premises) side of the VPN connection.
        #[builder(into, default)]
        pub local_ipv6_network_cidr: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates if a Public S2S VPN or Private S2S VPN over AWS Direct Connect. Valid values are `PublicIpv4 | PrivateIpv4`
        #[builder(into, default)]
        pub outside_ip_address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 CIDR on the AWS side of the VPN connection.
        #[builder(into, default)]
        pub remote_ipv4_network_cidr: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv6 CIDR on the AWS side of the VPN connection.
        #[builder(into, default)]
        pub remote_ipv6_network_cidr: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the VPN connection uses static routes exclusively. Static routes must be used for devices that don't support BGP.
        #[builder(into, default)]
        pub static_routes_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// Tags to apply to the connection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the EC2 Transit Gateway.
        #[builder(into, default)]
        pub transit_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// . The attachment ID of the Transit Gateway attachment to Direct Connect Gateway. The ID is obtained through a data source only.
        #[builder(into, default)]
        pub transport_transit_gateway_attachment_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The action to take after DPD timeout occurs for the first VPN tunnel. Specify restart to restart the IKE initiation. Specify clear to end the IKE session. Valid values are `clear | none | restart`.
        #[builder(into, default)]
        pub tunnel1_dpd_timeout_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of seconds after which a DPD timeout occurs for the first VPN tunnel. Valid value is equal or higher than `30`.
        #[builder(into, default)]
        pub tunnel1_dpd_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Turn on or off tunnel endpoint lifecycle control feature for the first VPN tunnel. Valid values are `true | false`.
        #[builder(into, default)]
        pub tunnel1_enable_tunnel_lifecycle_control: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The IKE versions that are permitted for the first VPN tunnel. Valid values are `ikev1 | ikev2`.
        #[builder(into, default)]
        pub tunnel1_ike_versions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The CIDR block of the inside IP addresses for the first VPN tunnel. Valid value is a size /30 CIDR block from the 169.254.0.0/16 range.
        #[builder(into, default)]
        pub tunnel1_inside_cidr: pulumi_wasm_rust::Output<Option<String>>,
        /// The range of inside IPv6 addresses for the first VPN tunnel. Supports only EC2 Transit Gateway. Valid value is a size /126 CIDR block from the local fd00::/8 range.
        #[builder(into, default)]
        pub tunnel1_inside_ipv6_cidr: pulumi_wasm_rust::Output<Option<String>>,
        /// Options for logging VPN tunnel activity. See Log Options below for more details.
        #[builder(into, default)]
        pub tunnel1_log_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::VpnConnectionTunnel1LogOptions>,
        >,
        /// List of one or more Diffie-Hellman group numbers that are permitted for the first VPN tunnel for phase 1 IKE negotiations. Valid values are ` 2 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24`.
        #[builder(into, default)]
        pub tunnel1_phase1_dh_group_numbers: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        /// List of one or more encryption algorithms that are permitted for the first VPN tunnel for phase 1 IKE negotiations. Valid values are `AES128 | AES256 | AES128-GCM-16 | AES256-GCM-16`.
        #[builder(into, default)]
        pub tunnel1_phase1_encryption_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// One or more integrity algorithms that are permitted for the first VPN tunnel for phase 1 IKE negotiations. Valid values are `SHA1 | SHA2-256 | SHA2-384 | SHA2-512`.
        #[builder(into, default)]
        pub tunnel1_phase1_integrity_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The lifetime for phase 1 of the IKE negotiation for the first VPN tunnel, in seconds. Valid value is between `900` and `28800`.
        #[builder(into, default)]
        pub tunnel1_phase1_lifetime_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of one or more Diffie-Hellman group numbers that are permitted for the first VPN tunnel for phase 2 IKE negotiations. Valid values are `2 | 5 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24`.
        #[builder(into, default)]
        pub tunnel1_phase2_dh_group_numbers: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        /// List of one or more encryption algorithms that are permitted for the first VPN tunnel for phase 2 IKE negotiations. Valid values are `AES128 | AES256 | AES128-GCM-16 | AES256-GCM-16`.
        #[builder(into, default)]
        pub tunnel1_phase2_encryption_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// List of one or more integrity algorithms that are permitted for the first VPN tunnel for phase 2 IKE negotiations. Valid values are `SHA1 | SHA2-256 | SHA2-384 | SHA2-512`.
        #[builder(into, default)]
        pub tunnel1_phase2_integrity_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The lifetime for phase 2 of the IKE negotiation for the first VPN tunnel, in seconds. Valid value is between `900` and `3600`.
        #[builder(into, default)]
        pub tunnel1_phase2_lifetime_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The preshared key of the first VPN tunnel. The preshared key must be between 8 and 64 characters in length and cannot start with zero(0). Allowed characters are alphanumeric characters, periods(.) and underscores(_).
        #[builder(into, default)]
        pub tunnel1_preshared_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The percentage of the rekey window for the first VPN tunnel (determined by `tunnel1_rekey_margin_time_seconds`) during which the rekey time is randomly selected. Valid value is between `0` and `100`.
        #[builder(into, default)]
        pub tunnel1_rekey_fuzz_percentage: pulumi_wasm_rust::Output<Option<i32>>,
        /// The margin time, in seconds, before the phase 2 lifetime expires, during which the AWS side of the first VPN connection performs an IKE rekey. The exact time of the rekey is randomly selected based on the value for `tunnel1_rekey_fuzz_percentage`. Valid value is between `60` and half of `tunnel1_phase2_lifetime_seconds`.
        #[builder(into, default)]
        pub tunnel1_rekey_margin_time_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The number of packets in an IKE replay window for the first VPN tunnel. Valid value is between `64` and `2048`.
        #[builder(into, default)]
        pub tunnel1_replay_window_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The action to take when the establishing the tunnel for the first VPN connection. By default, your customer gateway device must initiate the IKE negotiation and bring up the tunnel. Specify start for AWS to initiate the IKE negotiation. Valid values are `add | start`.
        #[builder(into, default)]
        pub tunnel1_startup_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The action to take after DPD timeout occurs for the second VPN tunnel. Specify restart to restart the IKE initiation. Specify clear to end the IKE session. Valid values are `clear | none | restart`.
        #[builder(into, default)]
        pub tunnel2_dpd_timeout_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of seconds after which a DPD timeout occurs for the second VPN tunnel. Valid value is equal or higher than `30`.
        #[builder(into, default)]
        pub tunnel2_dpd_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Turn on or off tunnel endpoint lifecycle control feature for the second VPN tunnel. Valid values are `true | false`.
        #[builder(into, default)]
        pub tunnel2_enable_tunnel_lifecycle_control: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The IKE versions that are permitted for the second VPN tunnel. Valid values are `ikev1 | ikev2`.
        #[builder(into, default)]
        pub tunnel2_ike_versions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The CIDR block of the inside IP addresses for the second VPN tunnel. Valid value is a size /30 CIDR block from the 169.254.0.0/16 range.
        #[builder(into, default)]
        pub tunnel2_inside_cidr: pulumi_wasm_rust::Output<Option<String>>,
        /// The range of inside IPv6 addresses for the second VPN tunnel. Supports only EC2 Transit Gateway. Valid value is a size /126 CIDR block from the local fd00::/8 range.
        #[builder(into, default)]
        pub tunnel2_inside_ipv6_cidr: pulumi_wasm_rust::Output<Option<String>>,
        /// Options for logging VPN tunnel activity. See Log Options below for more details.
        #[builder(into, default)]
        pub tunnel2_log_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::VpnConnectionTunnel2LogOptions>,
        >,
        /// List of one or more Diffie-Hellman group numbers that are permitted for the second VPN tunnel for phase 1 IKE negotiations. Valid values are ` 2 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24`.
        #[builder(into, default)]
        pub tunnel2_phase1_dh_group_numbers: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        /// List of one or more encryption algorithms that are permitted for the second VPN tunnel for phase 1 IKE negotiations. Valid values are `AES128 | AES256 | AES128-GCM-16 | AES256-GCM-16`.
        #[builder(into, default)]
        pub tunnel2_phase1_encryption_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// One or more integrity algorithms that are permitted for the second VPN tunnel for phase 1 IKE negotiations. Valid values are `SHA1 | SHA2-256 | SHA2-384 | SHA2-512`.
        #[builder(into, default)]
        pub tunnel2_phase1_integrity_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The lifetime for phase 1 of the IKE negotiation for the second VPN tunnel, in seconds. Valid value is between `900` and `28800`.
        #[builder(into, default)]
        pub tunnel2_phase1_lifetime_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of one or more Diffie-Hellman group numbers that are permitted for the second VPN tunnel for phase 2 IKE negotiations. Valid values are `2 | 5 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24`.
        #[builder(into, default)]
        pub tunnel2_phase2_dh_group_numbers: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        /// List of one or more encryption algorithms that are permitted for the second VPN tunnel for phase 2 IKE negotiations. Valid values are `AES128 | AES256 | AES128-GCM-16 | AES256-GCM-16`.
        #[builder(into, default)]
        pub tunnel2_phase2_encryption_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// List of one or more integrity algorithms that are permitted for the second VPN tunnel for phase 2 IKE negotiations. Valid values are `SHA1 | SHA2-256 | SHA2-384 | SHA2-512`.
        #[builder(into, default)]
        pub tunnel2_phase2_integrity_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The lifetime for phase 2 of the IKE negotiation for the second VPN tunnel, in seconds. Valid value is between `900` and `3600`.
        #[builder(into, default)]
        pub tunnel2_phase2_lifetime_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The preshared key of the second VPN tunnel. The preshared key must be between 8 and 64 characters in length and cannot start with zero(0). Allowed characters are alphanumeric characters, periods(.) and underscores(_).
        #[builder(into, default)]
        pub tunnel2_preshared_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The percentage of the rekey window for the second VPN tunnel (determined by `tunnel2_rekey_margin_time_seconds`) during which the rekey time is randomly selected. Valid value is between `0` and `100`.
        #[builder(into, default)]
        pub tunnel2_rekey_fuzz_percentage: pulumi_wasm_rust::Output<Option<i32>>,
        /// The margin time, in seconds, before the phase 2 lifetime expires, during which the AWS side of the second VPN connection performs an IKE rekey. The exact time of the rekey is randomly selected based on the value for `tunnel2_rekey_fuzz_percentage`. Valid value is between `60` and half of `tunnel2_phase2_lifetime_seconds`.
        #[builder(into, default)]
        pub tunnel2_rekey_margin_time_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The number of packets in an IKE replay window for the second VPN tunnel. Valid value is between `64` and `2048`.
        #[builder(into, default)]
        pub tunnel2_replay_window_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The action to take when the establishing the tunnel for the second VPN connection. By default, your customer gateway device must initiate the IKE negotiation and bring up the tunnel. Specify start for AWS to initiate the IKE negotiation. Valid values are `add | start`.
        #[builder(into, default)]
        pub tunnel2_startup_action: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicate whether the VPN tunnels process IPv4 or IPv6 traffic. Valid values are `ipv4 | ipv6`. `ipv6` Supports only EC2 Transit Gateway.
        #[builder(into, default)]
        pub tunnel_inside_ip_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of VPN connection. The only type AWS supports at this time is "ipsec.1".
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Private Gateway.
        #[builder(into, default)]
        pub vpn_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VpnConnectionResult {
        /// Amazon Resource Name (ARN) of the VPN Connection.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the core network.
        pub core_network_arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the core network attachment.
        pub core_network_attachment_arn: pulumi_wasm_rust::Output<String>,
        /// The configuration information for the VPN connection's customer gateway (in the native XML format).
        pub customer_gateway_configuration: pulumi_wasm_rust::Output<String>,
        /// The ID of the customer gateway.
        pub customer_gateway_id: pulumi_wasm_rust::Output<String>,
        /// Indicate whether to enable acceleration for the VPN connection. Supports only EC2 Transit Gateway.
        pub enable_acceleration: pulumi_wasm_rust::Output<bool>,
        /// The IPv4 CIDR on the customer gateway (on-premises) side of the VPN connection.
        pub local_ipv4_network_cidr: pulumi_wasm_rust::Output<String>,
        /// The IPv6 CIDR on the customer gateway (on-premises) side of the VPN connection.
        pub local_ipv6_network_cidr: pulumi_wasm_rust::Output<String>,
        /// Indicates if a Public S2S VPN or Private S2S VPN over AWS Direct Connect. Valid values are `PublicIpv4 | PrivateIpv4`
        pub outside_ip_address_type: pulumi_wasm_rust::Output<String>,
        /// The IPv4 CIDR on the AWS side of the VPN connection.
        pub remote_ipv4_network_cidr: pulumi_wasm_rust::Output<String>,
        /// The IPv6 CIDR on the AWS side of the VPN connection.
        pub remote_ipv6_network_cidr: pulumi_wasm_rust::Output<String>,
        /// The static routes associated with the VPN connection. Detailed below.
        pub routes: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::VpnConnectionRoute>,
        >,
        /// Whether the VPN connection uses static routes exclusively. Static routes must be used for devices that don't support BGP.
        pub static_routes_only: pulumi_wasm_rust::Output<bool>,
        /// Tags to apply to the connection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// When associated with an EC2 Transit Gateway (`transit_gateway_id` argument), the attachment ID. See also the `aws.ec2.Tag` resource for tagging the EC2 Transit Gateway VPN Attachment.
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the EC2 Transit Gateway.
        pub transit_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// . The attachment ID of the Transit Gateway attachment to Direct Connect Gateway. The ID is obtained through a data source only.
        pub transport_transit_gateway_attachment_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The public IP address of the first VPN tunnel.
        pub tunnel1_address: pulumi_wasm_rust::Output<String>,
        /// The bgp asn number of the first VPN tunnel.
        pub tunnel1_bgp_asn: pulumi_wasm_rust::Output<String>,
        /// The bgp holdtime of the first VPN tunnel.
        pub tunnel1_bgp_holdtime: pulumi_wasm_rust::Output<i32>,
        /// The RFC 6890 link-local address of the first VPN tunnel (Customer Gateway Side).
        pub tunnel1_cgw_inside_address: pulumi_wasm_rust::Output<String>,
        /// The action to take after DPD timeout occurs for the first VPN tunnel. Specify restart to restart the IKE initiation. Specify clear to end the IKE session. Valid values are `clear | none | restart`.
        pub tunnel1_dpd_timeout_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of seconds after which a DPD timeout occurs for the first VPN tunnel. Valid value is equal or higher than `30`.
        pub tunnel1_dpd_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Turn on or off tunnel endpoint lifecycle control feature for the first VPN tunnel. Valid values are `true | false`.
        pub tunnel1_enable_tunnel_lifecycle_control: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The IKE versions that are permitted for the first VPN tunnel. Valid values are `ikev1 | ikev2`.
        pub tunnel1_ike_versions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The CIDR block of the inside IP addresses for the first VPN tunnel. Valid value is a size /30 CIDR block from the 169.254.0.0/16 range.
        pub tunnel1_inside_cidr: pulumi_wasm_rust::Output<String>,
        /// The range of inside IPv6 addresses for the first VPN tunnel. Supports only EC2 Transit Gateway. Valid value is a size /126 CIDR block from the local fd00::/8 range.
        pub tunnel1_inside_ipv6_cidr: pulumi_wasm_rust::Output<String>,
        /// Options for logging VPN tunnel activity. See Log Options below for more details.
        pub tunnel1_log_options: pulumi_wasm_rust::Output<
            super::super::types::ec2::VpnConnectionTunnel1LogOptions,
        >,
        /// List of one or more Diffie-Hellman group numbers that are permitted for the first VPN tunnel for phase 1 IKE negotiations. Valid values are ` 2 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24`.
        pub tunnel1_phase1_dh_group_numbers: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        /// List of one or more encryption algorithms that are permitted for the first VPN tunnel for phase 1 IKE negotiations. Valid values are `AES128 | AES256 | AES128-GCM-16 | AES256-GCM-16`.
        pub tunnel1_phase1_encryption_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// One or more integrity algorithms that are permitted for the first VPN tunnel for phase 1 IKE negotiations. Valid values are `SHA1 | SHA2-256 | SHA2-384 | SHA2-512`.
        pub tunnel1_phase1_integrity_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The lifetime for phase 1 of the IKE negotiation for the first VPN tunnel, in seconds. Valid value is between `900` and `28800`.
        pub tunnel1_phase1_lifetime_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of one or more Diffie-Hellman group numbers that are permitted for the first VPN tunnel for phase 2 IKE negotiations. Valid values are `2 | 5 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24`.
        pub tunnel1_phase2_dh_group_numbers: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        /// List of one or more encryption algorithms that are permitted for the first VPN tunnel for phase 2 IKE negotiations. Valid values are `AES128 | AES256 | AES128-GCM-16 | AES256-GCM-16`.
        pub tunnel1_phase2_encryption_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// List of one or more integrity algorithms that are permitted for the first VPN tunnel for phase 2 IKE negotiations. Valid values are `SHA1 | SHA2-256 | SHA2-384 | SHA2-512`.
        pub tunnel1_phase2_integrity_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The lifetime for phase 2 of the IKE negotiation for the first VPN tunnel, in seconds. Valid value is between `900` and `3600`.
        pub tunnel1_phase2_lifetime_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The preshared key of the first VPN tunnel. The preshared key must be between 8 and 64 characters in length and cannot start with zero(0). Allowed characters are alphanumeric characters, periods(.) and underscores(_).
        pub tunnel1_preshared_key: pulumi_wasm_rust::Output<String>,
        /// The percentage of the rekey window for the first VPN tunnel (determined by `tunnel1_rekey_margin_time_seconds`) during which the rekey time is randomly selected. Valid value is between `0` and `100`.
        pub tunnel1_rekey_fuzz_percentage: pulumi_wasm_rust::Output<Option<i32>>,
        /// The margin time, in seconds, before the phase 2 lifetime expires, during which the AWS side of the first VPN connection performs an IKE rekey. The exact time of the rekey is randomly selected based on the value for `tunnel1_rekey_fuzz_percentage`. Valid value is between `60` and half of `tunnel1_phase2_lifetime_seconds`.
        pub tunnel1_rekey_margin_time_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The number of packets in an IKE replay window for the first VPN tunnel. Valid value is between `64` and `2048`.
        pub tunnel1_replay_window_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The action to take when the establishing the tunnel for the first VPN connection. By default, your customer gateway device must initiate the IKE negotiation and bring up the tunnel. Specify start for AWS to initiate the IKE negotiation. Valid values are `add | start`.
        pub tunnel1_startup_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The RFC 6890 link-local address of the first VPN tunnel (VPN Gateway Side).
        pub tunnel1_vgw_inside_address: pulumi_wasm_rust::Output<String>,
        /// The public IP address of the second VPN tunnel.
        pub tunnel2_address: pulumi_wasm_rust::Output<String>,
        /// The bgp asn number of the second VPN tunnel.
        pub tunnel2_bgp_asn: pulumi_wasm_rust::Output<String>,
        /// The bgp holdtime of the second VPN tunnel.
        pub tunnel2_bgp_holdtime: pulumi_wasm_rust::Output<i32>,
        /// The RFC 6890 link-local address of the second VPN tunnel (Customer Gateway Side).
        pub tunnel2_cgw_inside_address: pulumi_wasm_rust::Output<String>,
        /// The action to take after DPD timeout occurs for the second VPN tunnel. Specify restart to restart the IKE initiation. Specify clear to end the IKE session. Valid values are `clear | none | restart`.
        pub tunnel2_dpd_timeout_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of seconds after which a DPD timeout occurs for the second VPN tunnel. Valid value is equal or higher than `30`.
        pub tunnel2_dpd_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Turn on or off tunnel endpoint lifecycle control feature for the second VPN tunnel. Valid values are `true | false`.
        pub tunnel2_enable_tunnel_lifecycle_control: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The IKE versions that are permitted for the second VPN tunnel. Valid values are `ikev1 | ikev2`.
        pub tunnel2_ike_versions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The CIDR block of the inside IP addresses for the second VPN tunnel. Valid value is a size /30 CIDR block from the 169.254.0.0/16 range.
        pub tunnel2_inside_cidr: pulumi_wasm_rust::Output<String>,
        /// The range of inside IPv6 addresses for the second VPN tunnel. Supports only EC2 Transit Gateway. Valid value is a size /126 CIDR block from the local fd00::/8 range.
        pub tunnel2_inside_ipv6_cidr: pulumi_wasm_rust::Output<String>,
        /// Options for logging VPN tunnel activity. See Log Options below for more details.
        pub tunnel2_log_options: pulumi_wasm_rust::Output<
            super::super::types::ec2::VpnConnectionTunnel2LogOptions,
        >,
        /// List of one or more Diffie-Hellman group numbers that are permitted for the second VPN tunnel for phase 1 IKE negotiations. Valid values are ` 2 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24`.
        pub tunnel2_phase1_dh_group_numbers: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        /// List of one or more encryption algorithms that are permitted for the second VPN tunnel for phase 1 IKE negotiations. Valid values are `AES128 | AES256 | AES128-GCM-16 | AES256-GCM-16`.
        pub tunnel2_phase1_encryption_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// One or more integrity algorithms that are permitted for the second VPN tunnel for phase 1 IKE negotiations. Valid values are `SHA1 | SHA2-256 | SHA2-384 | SHA2-512`.
        pub tunnel2_phase1_integrity_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The lifetime for phase 1 of the IKE negotiation for the second VPN tunnel, in seconds. Valid value is between `900` and `28800`.
        pub tunnel2_phase1_lifetime_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of one or more Diffie-Hellman group numbers that are permitted for the second VPN tunnel for phase 2 IKE negotiations. Valid values are `2 | 5 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24`.
        pub tunnel2_phase2_dh_group_numbers: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        /// List of one or more encryption algorithms that are permitted for the second VPN tunnel for phase 2 IKE negotiations. Valid values are `AES128 | AES256 | AES128-GCM-16 | AES256-GCM-16`.
        pub tunnel2_phase2_encryption_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// List of one or more integrity algorithms that are permitted for the second VPN tunnel for phase 2 IKE negotiations. Valid values are `SHA1 | SHA2-256 | SHA2-384 | SHA2-512`.
        pub tunnel2_phase2_integrity_algorithms: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The lifetime for phase 2 of the IKE negotiation for the second VPN tunnel, in seconds. Valid value is between `900` and `3600`.
        pub tunnel2_phase2_lifetime_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The preshared key of the second VPN tunnel. The preshared key must be between 8 and 64 characters in length and cannot start with zero(0). Allowed characters are alphanumeric characters, periods(.) and underscores(_).
        pub tunnel2_preshared_key: pulumi_wasm_rust::Output<String>,
        /// The percentage of the rekey window for the second VPN tunnel (determined by `tunnel2_rekey_margin_time_seconds`) during which the rekey time is randomly selected. Valid value is between `0` and `100`.
        pub tunnel2_rekey_fuzz_percentage: pulumi_wasm_rust::Output<Option<i32>>,
        /// The margin time, in seconds, before the phase 2 lifetime expires, during which the AWS side of the second VPN connection performs an IKE rekey. The exact time of the rekey is randomly selected based on the value for `tunnel2_rekey_fuzz_percentage`. Valid value is between `60` and half of `tunnel2_phase2_lifetime_seconds`.
        pub tunnel2_rekey_margin_time_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The number of packets in an IKE replay window for the second VPN tunnel. Valid value is between `64` and `2048`.
        pub tunnel2_replay_window_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The action to take when the establishing the tunnel for the second VPN connection. By default, your customer gateway device must initiate the IKE negotiation and bring up the tunnel. Specify start for AWS to initiate the IKE negotiation. Valid values are `add | start`.
        pub tunnel2_startup_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The RFC 6890 link-local address of the second VPN tunnel (VPN Gateway Side).
        pub tunnel2_vgw_inside_address: pulumi_wasm_rust::Output<String>,
        /// Indicate whether the VPN tunnels process IPv4 or IPv6 traffic. Valid values are `ipv4 | ipv6`. `ipv6` Supports only EC2 Transit Gateway.
        pub tunnel_inside_ip_version: pulumi_wasm_rust::Output<String>,
        /// The type of VPN connection. The only type AWS supports at this time is "ipsec.1".
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Telemetry for the VPN tunnels. Detailed below.
        pub vgw_telemetries: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::VpnConnectionVgwTelemetry>,
        >,
        /// The ID of the Virtual Private Gateway.
        pub vpn_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpnConnectionArgs) -> VpnConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let customer_gateway_id_binding = args.customer_gateway_id.get_inner();
        let enable_acceleration_binding = args.enable_acceleration.get_inner();
        let local_ipv4_network_cidr_binding = args.local_ipv4_network_cidr.get_inner();
        let local_ipv6_network_cidr_binding = args.local_ipv6_network_cidr.get_inner();
        let outside_ip_address_type_binding = args.outside_ip_address_type.get_inner();
        let remote_ipv4_network_cidr_binding = args.remote_ipv4_network_cidr.get_inner();
        let remote_ipv6_network_cidr_binding = args.remote_ipv6_network_cidr.get_inner();
        let static_routes_only_binding = args.static_routes_only.get_inner();
        let tags_binding = args.tags.get_inner();
        let transit_gateway_id_binding = args.transit_gateway_id.get_inner();
        let transport_transit_gateway_attachment_id_binding = args
            .transport_transit_gateway_attachment_id
            .get_inner();
        let tunnel1_dpd_timeout_action_binding = args
            .tunnel1_dpd_timeout_action
            .get_inner();
        let tunnel1_dpd_timeout_seconds_binding = args
            .tunnel1_dpd_timeout_seconds
            .get_inner();
        let tunnel1_enable_tunnel_lifecycle_control_binding = args
            .tunnel1_enable_tunnel_lifecycle_control
            .get_inner();
        let tunnel1_ike_versions_binding = args.tunnel1_ike_versions.get_inner();
        let tunnel1_inside_cidr_binding = args.tunnel1_inside_cidr.get_inner();
        let tunnel1_inside_ipv6_cidr_binding = args.tunnel1_inside_ipv6_cidr.get_inner();
        let tunnel1_log_options_binding = args.tunnel1_log_options.get_inner();
        let tunnel1_phase1_dh_group_numbers_binding = args
            .tunnel1_phase1_dh_group_numbers
            .get_inner();
        let tunnel1_phase1_encryption_algorithms_binding = args
            .tunnel1_phase1_encryption_algorithms
            .get_inner();
        let tunnel1_phase1_integrity_algorithms_binding = args
            .tunnel1_phase1_integrity_algorithms
            .get_inner();
        let tunnel1_phase1_lifetime_seconds_binding = args
            .tunnel1_phase1_lifetime_seconds
            .get_inner();
        let tunnel1_phase2_dh_group_numbers_binding = args
            .tunnel1_phase2_dh_group_numbers
            .get_inner();
        let tunnel1_phase2_encryption_algorithms_binding = args
            .tunnel1_phase2_encryption_algorithms
            .get_inner();
        let tunnel1_phase2_integrity_algorithms_binding = args
            .tunnel1_phase2_integrity_algorithms
            .get_inner();
        let tunnel1_phase2_lifetime_seconds_binding = args
            .tunnel1_phase2_lifetime_seconds
            .get_inner();
        let tunnel1_preshared_key_binding = args.tunnel1_preshared_key.get_inner();
        let tunnel1_rekey_fuzz_percentage_binding = args
            .tunnel1_rekey_fuzz_percentage
            .get_inner();
        let tunnel1_rekey_margin_time_seconds_binding = args
            .tunnel1_rekey_margin_time_seconds
            .get_inner();
        let tunnel1_replay_window_size_binding = args
            .tunnel1_replay_window_size
            .get_inner();
        let tunnel1_startup_action_binding = args.tunnel1_startup_action.get_inner();
        let tunnel2_dpd_timeout_action_binding = args
            .tunnel2_dpd_timeout_action
            .get_inner();
        let tunnel2_dpd_timeout_seconds_binding = args
            .tunnel2_dpd_timeout_seconds
            .get_inner();
        let tunnel2_enable_tunnel_lifecycle_control_binding = args
            .tunnel2_enable_tunnel_lifecycle_control
            .get_inner();
        let tunnel2_ike_versions_binding = args.tunnel2_ike_versions.get_inner();
        let tunnel2_inside_cidr_binding = args.tunnel2_inside_cidr.get_inner();
        let tunnel2_inside_ipv6_cidr_binding = args.tunnel2_inside_ipv6_cidr.get_inner();
        let tunnel2_log_options_binding = args.tunnel2_log_options.get_inner();
        let tunnel2_phase1_dh_group_numbers_binding = args
            .tunnel2_phase1_dh_group_numbers
            .get_inner();
        let tunnel2_phase1_encryption_algorithms_binding = args
            .tunnel2_phase1_encryption_algorithms
            .get_inner();
        let tunnel2_phase1_integrity_algorithms_binding = args
            .tunnel2_phase1_integrity_algorithms
            .get_inner();
        let tunnel2_phase1_lifetime_seconds_binding = args
            .tunnel2_phase1_lifetime_seconds
            .get_inner();
        let tunnel2_phase2_dh_group_numbers_binding = args
            .tunnel2_phase2_dh_group_numbers
            .get_inner();
        let tunnel2_phase2_encryption_algorithms_binding = args
            .tunnel2_phase2_encryption_algorithms
            .get_inner();
        let tunnel2_phase2_integrity_algorithms_binding = args
            .tunnel2_phase2_integrity_algorithms
            .get_inner();
        let tunnel2_phase2_lifetime_seconds_binding = args
            .tunnel2_phase2_lifetime_seconds
            .get_inner();
        let tunnel2_preshared_key_binding = args.tunnel2_preshared_key.get_inner();
        let tunnel2_rekey_fuzz_percentage_binding = args
            .tunnel2_rekey_fuzz_percentage
            .get_inner();
        let tunnel2_rekey_margin_time_seconds_binding = args
            .tunnel2_rekey_margin_time_seconds
            .get_inner();
        let tunnel2_replay_window_size_binding = args
            .tunnel2_replay_window_size
            .get_inner();
        let tunnel2_startup_action_binding = args.tunnel2_startup_action.get_inner();
        let tunnel_inside_ip_version_binding = args.tunnel_inside_ip_version.get_inner();
        let type__binding = args.type_.get_inner();
        let vpn_gateway_id_binding = args.vpn_gateway_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpnConnection:VpnConnection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customerGatewayId".into(),
                    value: &customer_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "enableAcceleration".into(),
                    value: &enable_acceleration_binding,
                },
                register_interface::ObjectField {
                    name: "localIpv4NetworkCidr".into(),
                    value: &local_ipv4_network_cidr_binding,
                },
                register_interface::ObjectField {
                    name: "localIpv6NetworkCidr".into(),
                    value: &local_ipv6_network_cidr_binding,
                },
                register_interface::ObjectField {
                    name: "outsideIpAddressType".into(),
                    value: &outside_ip_address_type_binding,
                },
                register_interface::ObjectField {
                    name: "remoteIpv4NetworkCidr".into(),
                    value: &remote_ipv4_network_cidr_binding,
                },
                register_interface::ObjectField {
                    name: "remoteIpv6NetworkCidr".into(),
                    value: &remote_ipv6_network_cidr_binding,
                },
                register_interface::ObjectField {
                    name: "staticRoutesOnly".into(),
                    value: &static_routes_only_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "transportTransitGatewayAttachmentId".into(),
                    value: &transport_transit_gateway_attachment_id_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1DpdTimeoutAction".into(),
                    value: &tunnel1_dpd_timeout_action_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1DpdTimeoutSeconds".into(),
                    value: &tunnel1_dpd_timeout_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1EnableTunnelLifecycleControl".into(),
                    value: &tunnel1_enable_tunnel_lifecycle_control_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1IkeVersions".into(),
                    value: &tunnel1_ike_versions_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1InsideCidr".into(),
                    value: &tunnel1_inside_cidr_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1InsideIpv6Cidr".into(),
                    value: &tunnel1_inside_ipv6_cidr_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1LogOptions".into(),
                    value: &tunnel1_log_options_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1Phase1DhGroupNumbers".into(),
                    value: &tunnel1_phase1_dh_group_numbers_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1Phase1EncryptionAlgorithms".into(),
                    value: &tunnel1_phase1_encryption_algorithms_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1Phase1IntegrityAlgorithms".into(),
                    value: &tunnel1_phase1_integrity_algorithms_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1Phase1LifetimeSeconds".into(),
                    value: &tunnel1_phase1_lifetime_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1Phase2DhGroupNumbers".into(),
                    value: &tunnel1_phase2_dh_group_numbers_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1Phase2EncryptionAlgorithms".into(),
                    value: &tunnel1_phase2_encryption_algorithms_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1Phase2IntegrityAlgorithms".into(),
                    value: &tunnel1_phase2_integrity_algorithms_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1Phase2LifetimeSeconds".into(),
                    value: &tunnel1_phase2_lifetime_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1PresharedKey".into(),
                    value: &tunnel1_preshared_key_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1RekeyFuzzPercentage".into(),
                    value: &tunnel1_rekey_fuzz_percentage_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1RekeyMarginTimeSeconds".into(),
                    value: &tunnel1_rekey_margin_time_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1ReplayWindowSize".into(),
                    value: &tunnel1_replay_window_size_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel1StartupAction".into(),
                    value: &tunnel1_startup_action_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2DpdTimeoutAction".into(),
                    value: &tunnel2_dpd_timeout_action_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2DpdTimeoutSeconds".into(),
                    value: &tunnel2_dpd_timeout_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2EnableTunnelLifecycleControl".into(),
                    value: &tunnel2_enable_tunnel_lifecycle_control_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2IkeVersions".into(),
                    value: &tunnel2_ike_versions_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2InsideCidr".into(),
                    value: &tunnel2_inside_cidr_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2InsideIpv6Cidr".into(),
                    value: &tunnel2_inside_ipv6_cidr_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2LogOptions".into(),
                    value: &tunnel2_log_options_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2Phase1DhGroupNumbers".into(),
                    value: &tunnel2_phase1_dh_group_numbers_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2Phase1EncryptionAlgorithms".into(),
                    value: &tunnel2_phase1_encryption_algorithms_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2Phase1IntegrityAlgorithms".into(),
                    value: &tunnel2_phase1_integrity_algorithms_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2Phase1LifetimeSeconds".into(),
                    value: &tunnel2_phase1_lifetime_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2Phase2DhGroupNumbers".into(),
                    value: &tunnel2_phase2_dh_group_numbers_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2Phase2EncryptionAlgorithms".into(),
                    value: &tunnel2_phase2_encryption_algorithms_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2Phase2IntegrityAlgorithms".into(),
                    value: &tunnel2_phase2_integrity_algorithms_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2Phase2LifetimeSeconds".into(),
                    value: &tunnel2_phase2_lifetime_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2PresharedKey".into(),
                    value: &tunnel2_preshared_key_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2RekeyFuzzPercentage".into(),
                    value: &tunnel2_rekey_fuzz_percentage_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2RekeyMarginTimeSeconds".into(),
                    value: &tunnel2_rekey_margin_time_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2ReplayWindowSize".into(),
                    value: &tunnel2_replay_window_size_binding,
                },
                register_interface::ObjectField {
                    name: "tunnel2StartupAction".into(),
                    value: &tunnel2_startup_action_binding,
                },
                register_interface::ObjectField {
                    name: "tunnelInsideIpVersion".into(),
                    value: &tunnel_inside_ip_version_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "vpnGatewayId".into(),
                    value: &vpn_gateway_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "coreNetworkArn".into(),
                },
                register_interface::ResultField {
                    name: "coreNetworkAttachmentArn".into(),
                },
                register_interface::ResultField {
                    name: "customerGatewayConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "customerGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "enableAcceleration".into(),
                },
                register_interface::ResultField {
                    name: "localIpv4NetworkCidr".into(),
                },
                register_interface::ResultField {
                    name: "localIpv6NetworkCidr".into(),
                },
                register_interface::ResultField {
                    name: "outsideIpAddressType".into(),
                },
                register_interface::ResultField {
                    name: "remoteIpv4NetworkCidr".into(),
                },
                register_interface::ResultField {
                    name: "remoteIpv6NetworkCidr".into(),
                },
                register_interface::ResultField {
                    name: "routes".into(),
                },
                register_interface::ResultField {
                    name: "staticRoutesOnly".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayAttachmentId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "transportTransitGatewayAttachmentId".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1Address".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1BgpAsn".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1BgpHoldtime".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1CgwInsideAddress".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1DpdTimeoutAction".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1DpdTimeoutSeconds".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1EnableTunnelLifecycleControl".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1IkeVersions".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1InsideCidr".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1InsideIpv6Cidr".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1LogOptions".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1Phase1DhGroupNumbers".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1Phase1EncryptionAlgorithms".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1Phase1IntegrityAlgorithms".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1Phase1LifetimeSeconds".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1Phase2DhGroupNumbers".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1Phase2EncryptionAlgorithms".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1Phase2IntegrityAlgorithms".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1Phase2LifetimeSeconds".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1PresharedKey".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1RekeyFuzzPercentage".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1RekeyMarginTimeSeconds".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1ReplayWindowSize".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1StartupAction".into(),
                },
                register_interface::ResultField {
                    name: "tunnel1VgwInsideAddress".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2Address".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2BgpAsn".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2BgpHoldtime".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2CgwInsideAddress".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2DpdTimeoutAction".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2DpdTimeoutSeconds".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2EnableTunnelLifecycleControl".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2IkeVersions".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2InsideCidr".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2InsideIpv6Cidr".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2LogOptions".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2Phase1DhGroupNumbers".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2Phase1EncryptionAlgorithms".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2Phase1IntegrityAlgorithms".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2Phase1LifetimeSeconds".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2Phase2DhGroupNumbers".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2Phase2EncryptionAlgorithms".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2Phase2IntegrityAlgorithms".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2Phase2LifetimeSeconds".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2PresharedKey".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2RekeyFuzzPercentage".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2RekeyMarginTimeSeconds".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2ReplayWindowSize".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2StartupAction".into(),
                },
                register_interface::ResultField {
                    name: "tunnel2VgwInsideAddress".into(),
                },
                register_interface::ResultField {
                    name: "tunnelInsideIpVersion".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "vgwTelemetries".into(),
                },
                register_interface::ResultField {
                    name: "vpnGatewayId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpnConnectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            core_network_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreNetworkArn").unwrap(),
            ),
            core_network_attachment_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreNetworkAttachmentArn").unwrap(),
            ),
            customer_gateway_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerGatewayConfiguration").unwrap(),
            ),
            customer_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerGatewayId").unwrap(),
            ),
            enable_acceleration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableAcceleration").unwrap(),
            ),
            local_ipv4_network_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localIpv4NetworkCidr").unwrap(),
            ),
            local_ipv6_network_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localIpv6NetworkCidr").unwrap(),
            ),
            outside_ip_address_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outsideIpAddressType").unwrap(),
            ),
            remote_ipv4_network_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteIpv4NetworkCidr").unwrap(),
            ),
            remote_ipv6_network_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteIpv6NetworkCidr").unwrap(),
            ),
            routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routes").unwrap(),
            ),
            static_routes_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("staticRoutesOnly").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            transit_gateway_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayAttachmentId").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
            transport_transit_gateway_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transportTransitGatewayAttachmentId").unwrap(),
            ),
            tunnel1_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1Address").unwrap(),
            ),
            tunnel1_bgp_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1BgpAsn").unwrap(),
            ),
            tunnel1_bgp_holdtime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1BgpHoldtime").unwrap(),
            ),
            tunnel1_cgw_inside_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1CgwInsideAddress").unwrap(),
            ),
            tunnel1_dpd_timeout_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1DpdTimeoutAction").unwrap(),
            ),
            tunnel1_dpd_timeout_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1DpdTimeoutSeconds").unwrap(),
            ),
            tunnel1_enable_tunnel_lifecycle_control: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1EnableTunnelLifecycleControl").unwrap(),
            ),
            tunnel1_ike_versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1IkeVersions").unwrap(),
            ),
            tunnel1_inside_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1InsideCidr").unwrap(),
            ),
            tunnel1_inside_ipv6_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1InsideIpv6Cidr").unwrap(),
            ),
            tunnel1_log_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1LogOptions").unwrap(),
            ),
            tunnel1_phase1_dh_group_numbers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1Phase1DhGroupNumbers").unwrap(),
            ),
            tunnel1_phase1_encryption_algorithms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1Phase1EncryptionAlgorithms").unwrap(),
            ),
            tunnel1_phase1_integrity_algorithms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1Phase1IntegrityAlgorithms").unwrap(),
            ),
            tunnel1_phase1_lifetime_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1Phase1LifetimeSeconds").unwrap(),
            ),
            tunnel1_phase2_dh_group_numbers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1Phase2DhGroupNumbers").unwrap(),
            ),
            tunnel1_phase2_encryption_algorithms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1Phase2EncryptionAlgorithms").unwrap(),
            ),
            tunnel1_phase2_integrity_algorithms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1Phase2IntegrityAlgorithms").unwrap(),
            ),
            tunnel1_phase2_lifetime_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1Phase2LifetimeSeconds").unwrap(),
            ),
            tunnel1_preshared_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1PresharedKey").unwrap(),
            ),
            tunnel1_rekey_fuzz_percentage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1RekeyFuzzPercentage").unwrap(),
            ),
            tunnel1_rekey_margin_time_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1RekeyMarginTimeSeconds").unwrap(),
            ),
            tunnel1_replay_window_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1ReplayWindowSize").unwrap(),
            ),
            tunnel1_startup_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1StartupAction").unwrap(),
            ),
            tunnel1_vgw_inside_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel1VgwInsideAddress").unwrap(),
            ),
            tunnel2_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2Address").unwrap(),
            ),
            tunnel2_bgp_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2BgpAsn").unwrap(),
            ),
            tunnel2_bgp_holdtime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2BgpHoldtime").unwrap(),
            ),
            tunnel2_cgw_inside_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2CgwInsideAddress").unwrap(),
            ),
            tunnel2_dpd_timeout_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2DpdTimeoutAction").unwrap(),
            ),
            tunnel2_dpd_timeout_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2DpdTimeoutSeconds").unwrap(),
            ),
            tunnel2_enable_tunnel_lifecycle_control: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2EnableTunnelLifecycleControl").unwrap(),
            ),
            tunnel2_ike_versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2IkeVersions").unwrap(),
            ),
            tunnel2_inside_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2InsideCidr").unwrap(),
            ),
            tunnel2_inside_ipv6_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2InsideIpv6Cidr").unwrap(),
            ),
            tunnel2_log_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2LogOptions").unwrap(),
            ),
            tunnel2_phase1_dh_group_numbers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2Phase1DhGroupNumbers").unwrap(),
            ),
            tunnel2_phase1_encryption_algorithms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2Phase1EncryptionAlgorithms").unwrap(),
            ),
            tunnel2_phase1_integrity_algorithms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2Phase1IntegrityAlgorithms").unwrap(),
            ),
            tunnel2_phase1_lifetime_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2Phase1LifetimeSeconds").unwrap(),
            ),
            tunnel2_phase2_dh_group_numbers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2Phase2DhGroupNumbers").unwrap(),
            ),
            tunnel2_phase2_encryption_algorithms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2Phase2EncryptionAlgorithms").unwrap(),
            ),
            tunnel2_phase2_integrity_algorithms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2Phase2IntegrityAlgorithms").unwrap(),
            ),
            tunnel2_phase2_lifetime_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2Phase2LifetimeSeconds").unwrap(),
            ),
            tunnel2_preshared_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2PresharedKey").unwrap(),
            ),
            tunnel2_rekey_fuzz_percentage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2RekeyFuzzPercentage").unwrap(),
            ),
            tunnel2_rekey_margin_time_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2RekeyMarginTimeSeconds").unwrap(),
            ),
            tunnel2_replay_window_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2ReplayWindowSize").unwrap(),
            ),
            tunnel2_startup_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2StartupAction").unwrap(),
            ),
            tunnel2_vgw_inside_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnel2VgwInsideAddress").unwrap(),
            ),
            tunnel_inside_ip_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnelInsideIpVersion").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            vgw_telemetries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vgwTelemetries").unwrap(),
            ),
            vpn_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnGatewayId").unwrap(),
            ),
        }
    }
}