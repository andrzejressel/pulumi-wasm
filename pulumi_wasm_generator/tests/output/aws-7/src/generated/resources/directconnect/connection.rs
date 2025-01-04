/// Provides a Connection of Direct Connect.
///
/// ## Example Usage
///
/// ### Create a connection
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// The bandwidth of the connection. Valid values for dedicated connections: 1Gbps, 10Gbps. Valid values for hosted connections: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps, 10Gbps and 100Gbps. Case sensitive.
        #[builder(into)]
        pub bandwidth: pulumi_wasm_rust::Output<String>,
        /// The connection MAC Security (MACsec) encryption mode. MAC Security (MACsec) is only available on dedicated connections. Valid values are `no_encrypt`, `should_encrypt`, and `must_encrypt`.
        #[builder(into, default)]
        pub encryption_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The AWS Direct Connect location where the connection is located. See [DescribeLocations](https://docs.aws.amazon.com/directconnect/latest/APIReference/API_DescribeLocations.html) for the list of AWS Direct Connect locations. Use `locationCode`.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the connection.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the service provider associated with the connection.
        #[builder(into, default)]
        pub provider_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean value indicating whether you want the connection to support MAC Security (MACsec). MAC Security (MACsec) is only available on dedicated connections. See [MACsec prerequisites](https://docs.aws.amazon.com/directconnect/latest/UserGuide/direct-connect-mac-sec-getting-started.html#mac-sec-prerequisites) for more information about MAC Security (MACsec) prerequisites. Default value: `false`.
        ///
        /// > **NOTE:** Changing the value of `request_macsec` will cause the resource to be destroyed and re-created.
        #[builder(into, default)]
        pub request_macsec: pulumi_wasm_rust::Output<Option<bool>>,
        /// Set to true if you do not wish the connection to be deleted at destroy time, and instead just removed from the state.
        #[builder(into, default)]
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// The ARN of the connection.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Direct Connect endpoint on which the physical connection terminates.
        pub aws_device: pulumi_wasm_rust::Output<String>,
        /// The bandwidth of the connection. Valid values for dedicated connections: 1Gbps, 10Gbps. Valid values for hosted connections: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, 500Mbps, 1Gbps, 2Gbps, 5Gbps, 10Gbps and 100Gbps. Case sensitive.
        pub bandwidth: pulumi_wasm_rust::Output<String>,
        /// The connection MAC Security (MACsec) encryption mode. MAC Security (MACsec) is only available on dedicated connections. Valid values are `no_encrypt`, `should_encrypt`, and `must_encrypt`.
        pub encryption_mode: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the connection supports a secondary BGP peer in the same address family (IPv4/IPv6).
        pub has_logical_redundancy: pulumi_wasm_rust::Output<String>,
        /// Boolean value representing if jumbo frames have been enabled for this connection.
        pub jumbo_frame_capable: pulumi_wasm_rust::Output<bool>,
        /// The AWS Direct Connect location where the connection is located. See [DescribeLocations](https://docs.aws.amazon.com/directconnect/latest/APIReference/API_DescribeLocations.html) for the list of AWS Direct Connect locations. Use `locationCode`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Boolean value indicating whether the connection supports MAC Security (MACsec).
        pub macsec_capable: pulumi_wasm_rust::Output<bool>,
        /// The name of the connection.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the AWS account that owns the connection.
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
        /// The name of the AWS Direct Connect service provider associated with the connection.
        pub partner_name: pulumi_wasm_rust::Output<String>,
        /// The MAC Security (MACsec) port link status of the connection.
        pub port_encryption_status: pulumi_wasm_rust::Output<String>,
        /// The name of the service provider associated with the connection.
        pub provider_name: pulumi_wasm_rust::Output<String>,
        /// Boolean value indicating whether you want the connection to support MAC Security (MACsec). MAC Security (MACsec) is only available on dedicated connections. See [MACsec prerequisites](https://docs.aws.amazon.com/directconnect/latest/UserGuide/direct-connect-mac-sec-getting-started.html#mac-sec-prerequisites) for more information about MAC Security (MACsec) prerequisites. Default value: `false`.
        ///
        /// > **NOTE:** Changing the value of `request_macsec` will cause the resource to be destroyed and re-created.
        pub request_macsec: pulumi_wasm_rust::Output<Option<bool>>,
        /// Set to true if you do not wish the connection to be deleted at destroy time, and instead just removed from the state.
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VLAN ID.
        pub vlan_id: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectionArgs) -> ConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bandwidth_binding = args.bandwidth.get_inner();
        let encryption_mode_binding = args.encryption_mode.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let provider_name_binding = args.provider_name.get_inner();
        let request_macsec_binding = args.request_macsec.get_inner();
        let skip_destroy_binding = args.skip_destroy.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/connection:Connection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bandwidth".into(),
                    value: &bandwidth_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionMode".into(),
                    value: &encryption_mode_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "providerName".into(),
                    value: &provider_name_binding,
                },
                register_interface::ObjectField {
                    name: "requestMacsec".into(),
                    value: &request_macsec_binding,
                },
                register_interface::ObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding,
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
                    name: "awsDevice".into(),
                },
                register_interface::ResultField {
                    name: "bandwidth".into(),
                },
                register_interface::ResultField {
                    name: "encryptionMode".into(),
                },
                register_interface::ResultField {
                    name: "hasLogicalRedundancy".into(),
                },
                register_interface::ResultField {
                    name: "jumboFrameCapable".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "macsecCapable".into(),
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
                    name: "portEncryptionStatus".into(),
                },
                register_interface::ResultField {
                    name: "providerName".into(),
                },
                register_interface::ResultField {
                    name: "requestMacsec".into(),
                },
                register_interface::ResultField {
                    name: "skipDestroy".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vlanId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_device: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsDevice").unwrap(),
            ),
            bandwidth: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bandwidth").unwrap(),
            ),
            encryption_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionMode").unwrap(),
            ),
            has_logical_redundancy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasLogicalRedundancy").unwrap(),
            ),
            jumbo_frame_capable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jumboFrameCapable").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            macsec_capable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("macsecCapable").unwrap(),
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
            port_encryption_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("portEncryptionStatus").unwrap(),
            ),
            provider_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerName").unwrap(),
            ),
            request_macsec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestMacsec").unwrap(),
            ),
            skip_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipDestroy").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vlan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vlanId").unwrap(),
            ),
        }
    }
}
