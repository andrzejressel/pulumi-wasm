/// Manages an AWS Storage Gateway working storage.
///
/// > **NOTE:** The Storage Gateway API provides no method to remove a working storage disk. Destroying this resource does not perform any Storage Gateway actions.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = working_storage::create(
///         "example",
///         WorkingStorageArgs::builder()
///             .disk_id("${exampleAwsStoragegatewayLocalDisk.id}")
///             .gateway_arn("${exampleAwsStoragegatewayGateway.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_storagegateway_working_storage` using the gateway Amazon Resource Name (ARN) and local disk identifier separated with a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:storagegateway/workingStorage:WorkingStorage example arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678:pci-0000:03:00.0-scsi-0:0:0:0
/// ```
pub mod working_storage {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkingStorageArgs {
        /// Local disk identifier. For example, `pci-0000:03:00.0-scsi-0:0:0:0`.
        #[builder(into)]
        pub disk_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the gateway.
        #[builder(into)]
        pub gateway_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkingStorageResult {
        /// Local disk identifier. For example, `pci-0000:03:00.0-scsi-0:0:0:0`.
        pub disk_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the gateway.
        pub gateway_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkingStorageArgs) -> WorkingStorageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disk_id_binding = args.disk_id.get_inner();
        let gateway_arn_binding = args.gateway_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:storagegateway/workingStorage:WorkingStorage".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "diskId".into(),
                    value: &disk_id_binding,
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
                    name: "gatewayArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkingStorageResult {
            disk_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskId").unwrap(),
            ),
            gateway_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayArn").unwrap(),
            ),
        }
    }
}