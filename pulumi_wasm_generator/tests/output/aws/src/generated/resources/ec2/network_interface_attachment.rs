/// Attach an Elastic network interface (ENI) resource with EC2 instance.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = network_interface_attachment::create(
///         "test",
///         NetworkInterfaceAttachmentArgs::builder()
///             .device_index(0)
///             .instance_id("${testAwsInstance.id}")
///             .network_interface_id("${testAwsNetworkInterface.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Elastic network interface (ENI) Attachments using its Attachment ID. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/networkInterfaceAttachment:NetworkInterfaceAttachment secondary_nic eni-attach-0a33842b4ec347c4c
/// ```
pub mod network_interface_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInterfaceAttachmentArgs {
        /// Network interface index (int).
        #[builder(into)]
        pub device_index: pulumi_wasm_rust::Output<i32>,
        /// Instance ID to attach.
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// ENI ID to attach.
        #[builder(into)]
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkInterfaceAttachmentResult {
        /// The ENI Attachment ID.
        pub attachment_id: pulumi_wasm_rust::Output<String>,
        /// Network interface index (int).
        pub device_index: pulumi_wasm_rust::Output<i32>,
        /// Instance ID to attach.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// ENI ID to attach.
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// The status of the Network Interface Attachment.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkInterfaceAttachmentArgs,
    ) -> NetworkInterfaceAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let device_index_binding = args.device_index.get_inner();
        let instance_id_binding = args.instance_id.get_inner();
        let network_interface_id_binding = args.network_interface_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/networkInterfaceAttachment:NetworkInterfaceAttachment"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deviceIndex".into(),
                    value: &device_index_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attachmentId".into(),
                },
                register_interface::ResultField {
                    name: "deviceIndex".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkInterfaceAttachmentResult {
            attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachmentId").unwrap(),
            ),
            device_index: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceIndex").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceId").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}