pub mod get_local_disk {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalDiskArgs {
        /// Device node of the local disk to retrieve. For example, `/dev/sdb`.
        #[builder(into, default)]
        pub disk_node: pulumi_wasm_rust::Output<Option<String>>,
        /// Device path of the local disk to retrieve. For example, `/dev/xvdb` or `/dev/nvme1n1`.
        #[builder(into, default)]
        pub disk_path: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the gateway.
        #[builder(into)]
        pub gateway_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetLocalDiskResult {
        /// Disk identifierE.g., `pci-0000:03:00.0-scsi-0:0:0:0`
        pub disk_id: pulumi_wasm_rust::Output<String>,
        pub disk_node: pulumi_wasm_rust::Output<String>,
        pub disk_path: pulumi_wasm_rust::Output<String>,
        pub gateway_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLocalDiskArgs) -> GetLocalDiskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disk_node_binding = args.disk_node.get_inner();
        let disk_path_binding = args.disk_path.get_inner();
        let gateway_arn_binding = args.gateway_arn.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:storagegateway/getLocalDisk:getLocalDisk".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "diskNode".into(),
                    value: &disk_node_binding,
                },
                register_interface::ObjectField {
                    name: "diskPath".into(),
                    value: &disk_path_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayArn".into(),
                    value: &gateway_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "diskId".into(),
                },
                register_interface::ResultField {
                    name: "diskNode".into(),
                },
                register_interface::ResultField {
                    name: "diskPath".into(),
                },
                register_interface::ResultField {
                    name: "gatewayArn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLocalDiskResult {
            disk_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskId").unwrap(),
            ),
            disk_node: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskNode").unwrap(),
            ),
            disk_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskPath").unwrap(),
            ),
            gateway_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayArn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
