/// Provides a Global Accelerator listener.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = accelerator::create(
///         "example",
///         AcceleratorArgs::builder()
///             .attributes(
///                 AcceleratorAttributes::builder()
///                     .flowLogsEnabled(true)
///                     .flowLogsS3Bucket("example-bucket")
///                     .flowLogsS3Prefix("flow-logs/")
///                     .build_struct(),
///             )
///             .enabled(true)
///             .ip_address_type("IPV4")
///             .name("Example")
///             .build_struct(),
///     );
///     let exampleListener = listener::create(
///         "exampleListener",
///         ListenerArgs::builder()
///             .accelerator_arn("${example.id}")
///             .client_affinity("SOURCE_IP")
///             .port_ranges(
///                 vec![
///                     ListenerPortRange::builder().fromPort(80).toPort(80).build_struct(),
///                 ],
///             )
///             .protocol("TCP")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Global Accelerator listeners using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:globalaccelerator/listener:Listener example arn:aws:globalaccelerator::111111111111:accelerator/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/listener/xxxxxxxx
/// ```
pub mod listener {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListenerArgs {
        /// The Amazon Resource Name (ARN) of your accelerator.
        #[builder(into)]
        pub accelerator_arn: pulumi_wasm_rust::Output<String>,
        /// Direct all requests from a user to the same endpoint. Valid values are `NONE`, `SOURCE_IP`. Default: `NONE`. If `NONE`, Global Accelerator uses the "five-tuple" properties of source IP address, source port, destination IP address, destination port, and protocol to select the hash value. If `SOURCE_IP`, Global Accelerator uses the "two-tuple" properties of source (client) IP address and destination IP address to select the hash value.
        #[builder(into, default)]
        pub client_affinity: pulumi_wasm_rust::Output<Option<String>>,
        /// The list of port ranges for the connections from clients to the accelerator. Fields documented below.
        #[builder(into)]
        pub port_ranges: pulumi_wasm_rust::Output<
            Vec<super::super::types::globalaccelerator::ListenerPortRange>,
        >,
        /// The protocol for the connections from clients to the accelerator. Valid values are `TCP`, `UDP`.
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ListenerResult {
        /// The Amazon Resource Name (ARN) of your accelerator.
        pub accelerator_arn: pulumi_wasm_rust::Output<String>,
        /// Direct all requests from a user to the same endpoint. Valid values are `NONE`, `SOURCE_IP`. Default: `NONE`. If `NONE`, Global Accelerator uses the "five-tuple" properties of source IP address, source port, destination IP address, destination port, and protocol to select the hash value. If `SOURCE_IP`, Global Accelerator uses the "two-tuple" properties of source (client) IP address and destination IP address to select the hash value.
        pub client_affinity: pulumi_wasm_rust::Output<Option<String>>,
        /// The list of port ranges for the connections from clients to the accelerator. Fields documented below.
        pub port_ranges: pulumi_wasm_rust::Output<
            Vec<super::super::types::globalaccelerator::ListenerPortRange>,
        >,
        /// The protocol for the connections from clients to the accelerator. Valid values are `TCP`, `UDP`.
        pub protocol: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ListenerArgs) -> ListenerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accelerator_arn_binding = args.accelerator_arn.get_inner();
        let client_affinity_binding = args.client_affinity.get_inner();
        let port_ranges_binding = args.port_ranges.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:globalaccelerator/listener:Listener".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceleratorArn".into(),
                    value: &accelerator_arn_binding,
                },
                register_interface::ObjectField {
                    name: "clientAffinity".into(),
                    value: &client_affinity_binding,
                },
                register_interface::ObjectField {
                    name: "portRanges".into(),
                    value: &port_ranges_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceleratorArn".into(),
                },
                register_interface::ResultField {
                    name: "clientAffinity".into(),
                },
                register_interface::ResultField {
                    name: "portRanges".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ListenerResult {
            accelerator_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceleratorArn").unwrap(),
            ),
            client_affinity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientAffinity").unwrap(),
            ),
            port_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("portRanges").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
        }
    }
}
