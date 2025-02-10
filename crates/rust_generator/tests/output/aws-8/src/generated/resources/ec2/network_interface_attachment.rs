/// Attach an Elastic network interface (ENI) resource with EC2 instance.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_interface_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInterfaceAttachmentArgs {
        /// Network interface index (int).
        #[builder(into)]
        pub device_index: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Instance ID to attach.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ENI ID to attach.
        #[builder(into)]
        pub network_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkInterfaceAttachmentResult {
        /// The ENI Attachment ID.
        pub attachment_id: pulumi_gestalt_rust::Output<String>,
        /// Network interface index (int).
        pub device_index: pulumi_gestalt_rust::Output<i32>,
        /// Instance ID to attach.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// ENI ID to attach.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// The status of the Network Interface Attachment.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkInterfaceAttachmentArgs,
    ) -> NetworkInterfaceAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let device_index_binding = args.device_index.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let network_interface_id_binding = args.network_interface_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/networkInterfaceAttachment:NetworkInterfaceAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceIndex".into(),
                    value: device_index_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaceId".into(),
                    value: network_interface_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkInterfaceAttachmentResult {
            attachment_id: o.get_field("attachmentId"),
            device_index: o.get_field("deviceIndex"),
            instance_id: o.get_field("instanceId"),
            network_interface_id: o.get_field("networkInterfaceId"),
            status: o.get_field("status"),
        }
    }
}
