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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VolumeAttachmentArgs,
    ) -> VolumeAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let device_name_binding = args.device_name.get_output(context);
        let force_detach_binding = args.force_detach.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let skip_destroy_binding = args.skip_destroy.get_output(context);
        let stop_instance_before_detaching_binding = args
            .stop_instance_before_detaching
            .get_output(context);
        let volume_id_binding = args.volume_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/volumeAttachment:VolumeAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceName".into(),
                    value: device_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDetach".into(),
                    value: force_detach_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipDestroy".into(),
                    value: skip_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stopInstanceBeforeDetaching".into(),
                    value: stop_instance_before_detaching_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumeId".into(),
                    value: volume_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VolumeAttachmentResult {
            device_name: o.get_field("deviceName"),
            force_detach: o.get_field("forceDetach"),
            instance_id: o.get_field("instanceId"),
            skip_destroy: o.get_field("skipDestroy"),
            stop_instance_before_detaching: o.get_field("stopInstanceBeforeDetaching"),
            volume_id: o.get_field("volumeId"),
        }
    }
}
