#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_local_disk {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalDiskArgs {
        /// Device node of the local disk to retrieve. For example, `/dev/sdb`.
        #[builder(into, default)]
        pub disk_node: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Device path of the local disk to retrieve. For example, `/dev/xvdb` or `/dev/nvme1n1`.
        #[builder(into, default)]
        pub disk_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the gateway.
        #[builder(into)]
        pub gateway_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLocalDiskResult {
        /// Disk identifierE.g., `pci-0000:03:00.0-scsi-0:0:0:0`
        pub disk_id: pulumi_gestalt_rust::Output<String>,
        pub disk_node: pulumi_gestalt_rust::Output<String>,
        pub disk_path: pulumi_gestalt_rust::Output<String>,
        pub gateway_arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLocalDiskArgs,
    ) -> GetLocalDiskResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let disk_node_binding = args.disk_node.get_output(context);
        let disk_path_binding = args.disk_path.get_output(context);
        let gateway_arn_binding = args.gateway_arn.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:storagegateway/getLocalDisk:getLocalDisk".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskNode".into(),
                    value: &disk_node_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskPath".into(),
                    value: &disk_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayArn".into(),
                    value: &gateway_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLocalDiskResult {
            disk_id: o.get_field("diskId"),
            disk_node: o.get_field("diskNode"),
            disk_path: o.get_field("diskPath"),
            gateway_arn: o.get_field("gatewayArn"),
            id: o.get_field("id"),
        }
    }
}
