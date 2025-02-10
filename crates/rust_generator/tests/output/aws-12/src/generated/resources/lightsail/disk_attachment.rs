/// Attaches a Lightsail disk to a Lightsail Instance
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lightsail:Disk
///     properties:
///       name: test-disk
///       sizeInGb: 8
///       availabilityZone: ${available.names[0]}
///   testInstance:
///     type: aws:lightsail:Instance
///     name: test
///     properties:
///       name: test-instance
///       availabilityZone: ${available.names[0]}
///       blueprintId: amazon_linux_2
///       bundleId: nano_3_0
///   testDisk_attachment:
///     type: aws:lightsail:Disk_attachment
///     name: test
///     properties:
///       diskName: ${test.name}
///       instanceName: ${testInstance.name}
///       diskPath: /dev/xvdf
/// variables:
///   available:
///     fn::invoke:
///       function: aws:getAvailabilityZones
///       arguments:
///         state: available
///         filters:
///           - name: opt-in-status
///             values:
///               - opt-in-not-required
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_disk` using the id attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/disk_attachment:Disk_attachment test test-disk,test-instance
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod disk_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Disk_attachmentArgs {
        /// The name of the Lightsail Disk.
        #[builder(into)]
        pub disk_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The disk path to expose to the instance.
        #[builder(into)]
        pub disk_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Lightsail Instance to attach to.
        #[builder(into)]
        pub instance_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct Disk_attachmentResult {
        /// The name of the Lightsail Disk.
        pub disk_name: pulumi_gestalt_rust::Output<String>,
        /// The disk path to expose to the instance.
        pub disk_path: pulumi_gestalt_rust::Output<String>,
        /// The name of the Lightsail Instance to attach to.
        pub instance_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: Disk_attachmentArgs,
    ) -> Disk_attachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let disk_name_binding = args.disk_name.get_output(context);
        let disk_path_binding = args.disk_path.get_output(context);
        let instance_name_binding = args.instance_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/disk_attachment:Disk_attachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskName".into(),
                    value: disk_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskPath".into(),
                    value: disk_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceName".into(),
                    value: instance_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        Disk_attachmentResult {
            disk_name: o.get_field("diskName"),
            disk_path: o.get_field("diskPath"),
            instance_name: o.get_field("instanceName"),
        }
    }
}
