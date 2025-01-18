/// Provides a hosted connection on the specified interconnect or a link aggregation group (LAG) of interconnects. Intended for use by AWS Direct Connect Partners only.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let hosted = hosted_connection::create(
///         "hosted",
///         HostedConnectionArgs::builder()
///             .bandwidth("100Mbps")
///             .connection_id("dxcon-ffabc123")
///             .name("tf-dx-hosted-connection")
///             .owner_account_id("123456789012")
///             .vlan(1)
///             .build_struct(),
///     );
/// }
/// ```
pub mod hosted_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostedConnectionArgs {
        /// The bandwidth of the connection. Valid values for dedicated connections: 1Gbps, 10Gbps. Valid values for hosted connections: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps and 10Gbps. Case sensitive.
        #[builder(into)]
        pub bandwidth: pulumi_wasm_rust::Output<String>,
        /// The ID of the interconnect or LAG.
        #[builder(into)]
        pub connection_id: pulumi_wasm_rust::Output<String>,
        /// The name of the connection.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the AWS account of the customer for the connection.
        #[builder(into)]
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
        /// The dedicated VLAN provisioned to the hosted connection.
        #[builder(into)]
        pub vlan: pulumi_wasm_rust::Output<i32>,
    }
    #[allow(dead_code)]
    pub struct HostedConnectionResult {
        /// The Direct Connect endpoint on which the physical connection terminates.
        pub aws_device: pulumi_wasm_rust::Output<String>,
        /// The bandwidth of the connection. Valid values for dedicated connections: 1Gbps, 10Gbps. Valid values for hosted connections: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps and 10Gbps. Case sensitive.
        pub bandwidth: pulumi_wasm_rust::Output<String>,
        /// The ID of the interconnect or LAG.
        pub connection_id: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the connection supports a secondary BGP peer in the same address family (IPv4/IPv6).
        pub has_logical_redundancy: pulumi_wasm_rust::Output<String>,
        /// Boolean value representing if jumbo frames have been enabled for this connection.
        pub jumbo_frame_capable: pulumi_wasm_rust::Output<bool>,
        /// The ID of the LAG.
        pub lag_id: pulumi_wasm_rust::Output<String>,
        /// The time of the most recent call to [DescribeLoa](https://docs.aws.amazon.com/directconnect/latest/APIReference/API_DescribeLoa.html) for this connection.
        pub loa_issue_time: pulumi_wasm_rust::Output<String>,
        /// The location of the connection.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the connection.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the AWS account of the customer for the connection.
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
        /// The name of the AWS Direct Connect service provider associated with the connection.
        pub partner_name: pulumi_wasm_rust::Output<String>,
        /// The name of the service provider associated with the connection.
        pub provider_name: pulumi_wasm_rust::Output<String>,
        /// The AWS Region where the connection is located.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The state of the connection. Possible values include: ordering, requested, pending, available, down, deleting, deleted, rejected, unknown. See [AllocateHostedConnection](https://docs.aws.amazon.com/directconnect/latest/APIReference/API_AllocateHostedConnection.html) for a description of each connection state.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The dedicated VLAN provisioned to the hosted connection.
        pub vlan: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HostedConnectionArgs) -> HostedConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bandwidth_binding = args.bandwidth.get_inner();
        let connection_id_binding = args.connection_id.get_inner();
        let name_binding = args.name.get_inner();
        let owner_account_id_binding = args.owner_account_id.get_inner();
        let vlan_binding = args.vlan.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/hostedConnection:HostedConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bandwidth".into(),
                    value: &bandwidth_binding,
                },
                register_interface::ObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ownerAccountId".into(),
                    value: &owner_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "vlan".into(),
                    value: &vlan_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "awsDevice".into(),
                },
                register_interface::ResultField {
                    name: "bandwidth".into(),
                },
                register_interface::ResultField {
                    name: "connectionId".into(),
                },
                register_interface::ResultField {
                    name: "hasLogicalRedundancy".into(),
                },
                register_interface::ResultField {
                    name: "jumboFrameCapable".into(),
                },
                register_interface::ResultField {
                    name: "lagId".into(),
                },
                register_interface::ResultField {
                    name: "loaIssueTime".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ownerAccountId".into(),
                },
                register_interface::ResultField {
                    name: "partnerName".into(),
                },
                register_interface::ResultField {
                    name: "providerName".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "vlan".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HostedConnectionResult {
            aws_device: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsDevice").unwrap(),
            ),
            bandwidth: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bandwidth").unwrap(),
            ),
            connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionId").unwrap(),
            ),
            has_logical_redundancy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasLogicalRedundancy").unwrap(),
            ),
            jumbo_frame_capable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jumboFrameCapable").unwrap(),
            ),
            lag_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lagId").unwrap(),
            ),
            loa_issue_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loaIssueTime").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerAccountId").unwrap(),
            ),
            partner_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partnerName").unwrap(),
            ),
            provider_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerName").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            vlan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vlan").unwrap(),
            ),
        }
    }
}
