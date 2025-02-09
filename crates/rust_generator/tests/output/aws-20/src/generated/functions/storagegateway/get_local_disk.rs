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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLocalDiskArgs,
    ) -> GetLocalDiskResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let disk_node_binding_1 = args.disk_node.get_output(context);
        let disk_node_binding = disk_node_binding_1.get_inner();
        let disk_path_binding_1 = args.disk_path.get_output(context);
        let disk_path_binding = disk_path_binding_1.get_inner();
        let gateway_arn_binding_1 = args.gateway_arn.get_output(context);
        let gateway_arn_binding = gateway_arn_binding_1.get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLocalDiskResult {
            disk_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskId"),
            ),
            disk_node: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskNode"),
            ),
            disk_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskPath"),
            ),
            gateway_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayArn"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
