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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NetworkInterfaceAttachmentArgs,
    ) -> NetworkInterfaceAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let device_index_binding = args.device_index.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let network_interface_id_binding = args
            .network_interface_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/networkInterfaceAttachment:NetworkInterfaceAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkInterfaceAttachmentResult {
            attachment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachmentId"),
            ),
            device_index: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deviceIndex"),
            ),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            network_interface_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceId"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
