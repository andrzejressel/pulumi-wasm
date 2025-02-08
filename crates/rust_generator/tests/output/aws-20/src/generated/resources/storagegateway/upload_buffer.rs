/// Manages an AWS Storage Gateway upload buffer.
///
/// > **NOTE:** The Storage Gateway API provides no method to remove an upload buffer disk. Destroying this resource does not perform any Storage Gateway actions.
///
/// ## Example Usage
///
/// ### Cached and VTL Gateway Type
///
/// ```yaml
/// resources:
///   testUploadBuffer:
///     type: aws:storagegateway:UploadBuffer
///     name: test
///     properties:
///       diskPath: ${test.diskPath}
///       gatewayArn: ${testAwsStoragegatewayGateway.arn}
/// variables:
///   test:
///     fn::invoke:
///       function: aws:storagegateway:getLocalDisk
///       arguments:
///         diskNode: ${testAwsVolumeAttachment.deviceName}
///         gatewayArn: ${testAwsStoragegatewayGateway.arn}
/// ```
///
/// ### Stored Gateway Type
///
/// ```yaml
/// resources:
///   example:
///     type: aws:storagegateway:UploadBuffer
///     properties:
///       diskId: ${exampleAwsStoragegatewayLocalDisk.id}
///       gatewayArn: ${exampleAwsStoragegatewayGateway.arn}
/// variables:
///   test:
///     fn::invoke:
///       function: aws:storagegateway:getLocalDisk
///       arguments:
///         diskNode: ${testAwsVolumeAttachment.deviceName}
///         gatewayArn: ${testAwsStoragegatewayGateway.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_storagegateway_upload_buffer` using the gateway Amazon Resource Name (ARN) and local disk identifier separated with a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:storagegateway/uploadBuffer:UploadBuffer example arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678:pci-0000:03:00.0-scsi-0:0:0:0
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod upload_buffer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UploadBufferArgs {
        /// Local disk identifier. For example, `pci-0000:03:00.0-scsi-0:0:0:0`.
        #[builder(into, default)]
        pub disk_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Local disk path. For example, `/dev/nvme1n1`.
        #[builder(into, default)]
        pub disk_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the gateway.
        #[builder(into)]
        pub gateway_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UploadBufferResult {
        /// Local disk identifier. For example, `pci-0000:03:00.0-scsi-0:0:0:0`.
        pub disk_id: pulumi_gestalt_rust::Output<String>,
        /// Local disk path. For example, `/dev/nvme1n1`.
        pub disk_path: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the gateway.
        pub gateway_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: UploadBufferArgs,
    ) -> UploadBufferResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let disk_id_binding = args.disk_id.get_output(context).get_inner();
        let disk_path_binding = args.disk_path.get_output(context).get_inner();
        let gateway_arn_binding = args.gateway_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:storagegateway/uploadBuffer:UploadBuffer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "diskId".into(),
                    value: &disk_id_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        UploadBufferResult {
            disk_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskId"),
            ),
            disk_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskPath"),
            ),
            gateway_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayArn"),
            ),
        }
    }
}
