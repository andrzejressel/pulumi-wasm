/// Creates a connection between two devices.
/// The devices can be a physical or virtual appliance that connects to a third-party appliance in a VPC, or a physical appliance that connects to another physical appliance in an on-premises network.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = connection::create(
///         "example",
///         ConnectionArgs::builder()
///             .connected_device_id("${example2.id}")
///             .device_id("${example1.id}")
///             .global_network_id("${exampleAwsNetworkmanagerGlobalNetwork.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_connection` using the connection ARN. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/connection:Connection example arn:aws:networkmanager::123456789012:device/global-network-0d47f6t230mz46dy4/connection-07f6fd08867abc123
/// ```
pub mod connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// The ID of the second device in the connection.
        #[builder(into)]
        pub connected_device_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the link for the second device.
        #[builder(into, default)]
        pub connected_link_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of the connection.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the first device in the connection.
        #[builder(into)]
        pub device_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the global network.
        #[builder(into)]
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the link for the first device.
        #[builder(into, default)]
        pub link_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value tags for the connection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// The Amazon Resource Name (ARN) of the connection.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the second device in the connection.
        pub connected_device_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the link for the second device.
        pub connected_link_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of the connection.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the first device in the connection.
        pub device_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the global network.
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the link for the first device.
        pub link_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value tags for the connection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectionArgs) -> ConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connected_device_id_binding = args.connected_device_id.get_inner();
        let connected_link_id_binding = args.connected_link_id.get_inner();
        let description_binding = args.description.get_inner();
        let device_id_binding = args.device_id.get_inner();
        let global_network_id_binding = args.global_network_id.get_inner();
        let link_id_binding = args.link_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/connection:Connection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectedDeviceId".into(),
                    value: &connected_device_id_binding,
                },
                register_interface::ObjectField {
                    name: "connectedLinkId".into(),
                    value: &connected_link_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "deviceId".into(),
                    value: &device_id_binding,
                },
                register_interface::ObjectField {
                    name: "globalNetworkId".into(),
                    value: &global_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "linkId".into(),
                    value: &link_id_binding,
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
                    name: "connectedDeviceId".into(),
                },
                register_interface::ResultField {
                    name: "connectedLinkId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "deviceId".into(),
                },
                register_interface::ResultField {
                    name: "globalNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "linkId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
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
            connected_device_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectedDeviceId").unwrap(),
            ),
            connected_link_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectedLinkId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            device_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceId").unwrap(),
            ),
            global_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalNetworkId").unwrap(),
            ),
            link_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
