/// Provides a hosted connection on the specified interconnect or a link aggregation group (LAG) of interconnects. Intended for use by AWS Direct Connect Partners only.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation)]
pub mod hosted_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostedConnectionArgs {
        /// The bandwidth of the connection. Valid values for dedicated connections: 1Gbps, 10Gbps. Valid values for hosted connections: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps and 10Gbps. Case sensitive.
        #[builder(into)]
        pub bandwidth: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the interconnect or LAG.
        #[builder(into)]
        pub connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the connection.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the AWS account of the customer for the connection.
        #[builder(into)]
        pub owner_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The dedicated VLAN provisioned to the hosted connection.
        #[builder(into)]
        pub vlan: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct HostedConnectionResult {
        /// The Direct Connect endpoint on which the physical connection terminates.
        pub aws_device: pulumi_gestalt_rust::Output<String>,
        /// The bandwidth of the connection. Valid values for dedicated connections: 1Gbps, 10Gbps. Valid values for hosted connections: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps and 10Gbps. Case sensitive.
        pub bandwidth: pulumi_gestalt_rust::Output<String>,
        /// The ID of the interconnect or LAG.
        pub connection_id: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the connection supports a secondary BGP peer in the same address family (IPv4/IPv6).
        pub has_logical_redundancy: pulumi_gestalt_rust::Output<String>,
        /// Boolean value representing if jumbo frames have been enabled for this connection.
        pub jumbo_frame_capable: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the LAG.
        pub lag_id: pulumi_gestalt_rust::Output<String>,
        /// The time of the most recent call to [DescribeLoa](https://docs.aws.amazon.com/directconnect/latest/APIReference/API_DescribeLoa.html) for this connection.
        pub loa_issue_time: pulumi_gestalt_rust::Output<String>,
        /// The location of the connection.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the connection.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the AWS account of the customer for the connection.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the AWS Direct Connect service provider associated with the connection.
        pub partner_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the service provider associated with the connection.
        pub provider_name: pulumi_gestalt_rust::Output<String>,
        /// The AWS Region where the connection is located.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The state of the connection. Possible values include: ordering, requested, pending, available, down, deleting, deleted, rejected, unknown. See [AllocateHostedConnection](https://docs.aws.amazon.com/directconnect/latest/APIReference/API_AllocateHostedConnection.html) for a description of each connection state.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The dedicated VLAN provisioned to the hosted connection.
        pub vlan: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: HostedConnectionArgs,
    ) -> HostedConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bandwidth_binding = args.bandwidth.get_output(context).get_inner();
        let connection_id_binding = args.connection_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let owner_account_id_binding = args
            .owner_account_id
            .get_output(context)
            .get_inner();
        let vlan_binding = args.vlan.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        HostedConnectionResult {
            aws_device: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsDevice"),
            ),
            bandwidth: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bandwidth"),
            ),
            connection_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionId"),
            ),
            has_logical_redundancy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hasLogicalRedundancy"),
            ),
            jumbo_frame_capable: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("jumboFrameCapable"),
            ),
            lag_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lagId"),
            ),
            loa_issue_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loaIssueTime"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerAccountId"),
            ),
            partner_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partnerName"),
            ),
            provider_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("providerName"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            vlan: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vlan")),
        }
    }
}
