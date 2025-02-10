/// Provides a Connection of Direct Connect.
///
/// ## Example Usage
///
/// ### Create a connection
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let hoge = connection::create(
///         "hoge",
///         ConnectionArgs::builder()
///             .bandwidth("1Gbps")
///             .location("EqDC2")
///             .name("tf-dx-connection")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Request a MACsec-capable connection
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = connection::create(
///         "example",
///         ConnectionArgs::builder()
///             .bandwidth("10Gbps")
///             .location("EqDA2")
///             .name("tf-dx-connection")
///             .request_macsec(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Configure encryption mode for MACsec-capable connections
///
/// > **NOTE:** You can only specify the `encryption_mode` argument once the connection is in an `Available` state.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = connection::create(
///         "example",
///         ConnectionArgs::builder()
///             .bandwidth("10Gbps")
///             .encryption_mode("must_encrypt")
///             .location("EqDC2")
///             .name("tf-dx-connection")
///             .request_macsec(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Direct Connect connections using the connection `id`. For example:
///
/// ```sh
/// $ pulumi import aws:directconnect/connection:Connection test_connection dxcon-ffre0ec3
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// The bandwidth of the connection. Valid values for dedicated connections: 1Gbps, 10Gbps. Valid values for hosted connections: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps, 10Gbps and 100Gbps. Case sensitive.
        #[builder(into)]
        pub bandwidth: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The connection MAC Security (MACsec) encryption mode. MAC Security (MACsec) is only available on dedicated connections. Valid values are `no_encrypt`, `should_encrypt`, and `must_encrypt`.
        #[builder(into, default)]
        pub encryption_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The AWS Direct Connect location where the connection is located. See [DescribeLocations](https://docs.aws.amazon.com/directconnect/latest/APIReference/API_DescribeLocations.html) for the list of AWS Direct Connect locations. Use `locationCode`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the connection.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the service provider associated with the connection.
        #[builder(into, default)]
        pub provider_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean value indicating whether you want the connection to support MAC Security (MACsec). MAC Security (MACsec) is only available on dedicated connections. See [MACsec prerequisites](https://docs.aws.amazon.com/directconnect/latest/UserGuide/direct-connect-mac-sec-getting-started.html#mac-sec-prerequisites) for more information about MAC Security (MACsec) prerequisites. Default value: `false`.
        ///
        /// > **NOTE:** Changing the value of `request_macsec` will cause the resource to be destroyed and re-created.
        #[builder(into, default)]
        pub request_macsec: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Set to true if you do not wish the connection to be deleted at destroy time, and instead just removed from the state.
        #[builder(into, default)]
        pub skip_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// The ARN of the connection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Direct Connect endpoint on which the physical connection terminates.
        pub aws_device: pulumi_gestalt_rust::Output<String>,
        /// The bandwidth of the connection. Valid values for dedicated connections: 1Gbps, 10Gbps. Valid values for hosted connections: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps, 10Gbps and 100Gbps. Case sensitive.
        pub bandwidth: pulumi_gestalt_rust::Output<String>,
        /// The connection MAC Security (MACsec) encryption mode. MAC Security (MACsec) is only available on dedicated connections. Valid values are `no_encrypt`, `should_encrypt`, and `must_encrypt`.
        pub encryption_mode: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the connection supports a secondary BGP peer in the same address family (IPv4/IPv6).
        pub has_logical_redundancy: pulumi_gestalt_rust::Output<String>,
        /// Boolean value representing if jumbo frames have been enabled for this connection.
        pub jumbo_frame_capable: pulumi_gestalt_rust::Output<bool>,
        /// The AWS Direct Connect location where the connection is located. See [DescribeLocations](https://docs.aws.amazon.com/directconnect/latest/APIReference/API_DescribeLocations.html) for the list of AWS Direct Connect locations. Use `locationCode`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Boolean value indicating whether the connection supports MAC Security (MACsec).
        pub macsec_capable: pulumi_gestalt_rust::Output<bool>,
        /// The name of the connection.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the AWS account that owns the connection.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the AWS Direct Connect service provider associated with the connection.
        pub partner_name: pulumi_gestalt_rust::Output<String>,
        /// The MAC Security (MACsec) port link status of the connection.
        pub port_encryption_status: pulumi_gestalt_rust::Output<String>,
        /// The name of the service provider associated with the connection.
        pub provider_name: pulumi_gestalt_rust::Output<String>,
        /// Boolean value indicating whether you want the connection to support MAC Security (MACsec). MAC Security (MACsec) is only available on dedicated connections. See [MACsec prerequisites](https://docs.aws.amazon.com/directconnect/latest/UserGuide/direct-connect-mac-sec-getting-started.html#mac-sec-prerequisites) for more information about MAC Security (MACsec) prerequisites. Default value: `false`.
        ///
        /// > **NOTE:** Changing the value of `request_macsec` will cause the resource to be destroyed and re-created.
        pub request_macsec: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Set to true if you do not wish the connection to be deleted at destroy time, and instead just removed from the state.
        pub skip_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VLAN ID.
        pub vlan_id: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectionArgs,
    ) -> ConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bandwidth_binding = args.bandwidth.get_output(context);
        let encryption_mode_binding = args.encryption_mode.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let provider_name_binding = args.provider_name.get_output(context);
        let request_macsec_binding = args.request_macsec.get_output(context);
        let skip_destroy_binding = args.skip_destroy.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directconnect/connection:Connection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bandwidth".into(),
                    value: bandwidth_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionMode".into(),
                    value: encryption_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "providerName".into(),
                    value: provider_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestMacsec".into(),
                    value: request_macsec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipDestroy".into(),
                    value: skip_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectionResult {
            arn: o.get_field("arn"),
            aws_device: o.get_field("awsDevice"),
            bandwidth: o.get_field("bandwidth"),
            encryption_mode: o.get_field("encryptionMode"),
            has_logical_redundancy: o.get_field("hasLogicalRedundancy"),
            jumbo_frame_capable: o.get_field("jumboFrameCapable"),
            location: o.get_field("location"),
            macsec_capable: o.get_field("macsecCapable"),
            name: o.get_field("name"),
            owner_account_id: o.get_field("ownerAccountId"),
            partner_name: o.get_field("partnerName"),
            port_encryption_status: o.get_field("portEncryptionStatus"),
            provider_name: o.get_field("providerName"),
            request_macsec: o.get_field("requestMacsec"),
            skip_destroy: o.get_field("skipDestroy"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vlan_id: o.get_field("vlanId"),
        }
    }
}
