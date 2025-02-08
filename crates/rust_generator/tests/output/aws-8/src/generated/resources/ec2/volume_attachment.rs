/// Provides an AWS EBS Volume Attachment as a top level resource, to attach and
/// detach volumes from AWS Instances.
///
/// > **NOTE on EBS block devices:** If you use `ebs_block_device` on an `aws.ec2.Instance`, this provider will assume management over the full set of non-root EBS block devices for the instance, and treats additional block devices as drift. For this reason, `ebs_block_device` cannot be mixed with external `aws.ebs.Volume` + `aws.ec2.VolumeAttachment` resources for a given instance.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   ebsAtt:
///     type: aws:ec2:VolumeAttachment
///     name: ebs_att
///     properties:
///       deviceName: /dev/sdh
///       volumeId: ${example.id}
///       instanceId: ${web.id}
///   web:
///     type: aws:ec2:Instance
///     properties:
///       ami: ami-21f78e11
///       availabilityZone: us-west-2a
///       instanceType: t2.micro
///       tags:
///         Name: HelloWorld
///   example:
///     type: aws:ebs:Volume
///     properties:
///       availabilityZone: us-west-2a
///       size: 1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EBS Volume Attachments using `DEVICE_NAME:VOLUME_ID:INSTANCE_ID`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/volumeAttachment:VolumeAttachment example /dev/sdh:vol-049df61146c4d7901:i-12345678
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod volume_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeAttachmentArgs {
        /// The device name to expose to the instance (for
        /// example, `/dev/sdh` or `xvdh`).  See [Device Naming on Linux Instances](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/device_naming.html#available-ec2-device-names) and [Device Naming on Windows Instances](https://docs.aws.amazon.com/AWSEC2/latest/WindowsGuide/device_naming.html#available-ec2-device-names) for more information.
        #[builder(into)]
        pub device_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set to `true` if you want to force the
        /// volume to detach. Useful if previous attempts failed, but use this option only
        /// as a last resort, as this can result in **data loss**. See
        /// [Detaching an Amazon EBS Volume from an Instance](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-detaching-volume.html) for more information.
        #[builder(into, default)]
        pub force_detach: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// ID of the Instance to attach to
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set this to true if you do not wish
        /// to detach the volume from the instance to which it is attached at destroy
        /// time, and instead just remove the attachment from this provider state. This is
        /// useful when destroying an instance which has volumes created by some other
        /// means attached.
        #[builder(into, default)]
        pub skip_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Set this to true to ensure that the target instance is stopped
        /// before trying to detach the volume. Stops the instance, if it is not already stopped.
        #[builder(into, default)]
        pub stop_instance_before_detaching: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// ID of the Volume to be attached
        #[builder(into)]
        pub volume_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VolumeAttachmentResult {
        /// The device name to expose to the instance (for
        /// example, `/dev/sdh` or `xvdh`).  See [Device Naming on Linux Instances](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/device_naming.html#available-ec2-device-names) and [Device Naming on Windows Instances](https://docs.aws.amazon.com/AWSEC2/latest/WindowsGuide/device_naming.html#available-ec2-device-names) for more information.
        pub device_name: pulumi_gestalt_rust::Output<String>,
        /// Set to `true` if you want to force the
        /// volume to detach. Useful if previous attempts failed, but use this option only
        /// as a last resort, as this can result in **data loss**. See
        /// [Detaching an Amazon EBS Volume from an Instance](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-detaching-volume.html) for more information.
        pub force_detach: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ID of the Instance to attach to
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Set this to true if you do not wish
        /// to detach the volume from the instance to which it is attached at destroy
        /// time, and instead just remove the attachment from this provider state. This is
        /// useful when destroying an instance which has volumes created by some other
        /// means attached.
        pub skip_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Set this to true to ensure that the target instance is stopped
        /// before trying to detach the volume. Stops the instance, if it is not already stopped.
        pub stop_instance_before_detaching: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ID of the Volume to be attached
        pub volume_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VolumeAttachmentArgs,
    ) -> VolumeAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let device_name_binding = args.device_name.get_output(context).get_inner();
        let force_detach_binding = args.force_detach.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let skip_destroy_binding = args.skip_destroy.get_output(context).get_inner();
        let stop_instance_before_detaching_binding = args
            .stop_instance_before_detaching
            .get_output(context)
            .get_inner();
        let volume_id_binding = args.volume_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/volumeAttachment:VolumeAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deviceName".into(),
                    value: &device_name_binding,
                },
                register_interface::ObjectField {
                    name: "forceDetach".into(),
                    value: &force_detach_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "stopInstanceBeforeDetaching".into(),
                    value: &stop_instance_before_detaching_binding,
                },
                register_interface::ObjectField {
                    name: "volumeId".into(),
                    value: &volume_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VolumeAttachmentResult {
            device_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deviceName"),
            ),
            force_detach: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDetach"),
            ),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            skip_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skipDestroy"),
            ),
            stop_instance_before_detaching: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stopInstanceBeforeDetaching"),
            ),
            volume_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeId"),
            ),
        }
    }
}
