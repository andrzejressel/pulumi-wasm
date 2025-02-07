/// Manages a custom virtual machine image that can be used to create virtual machines.
///
/// ## Example Usage
///
/// > **Note:** For a more complete example, see the `examples/image` directory within the GitHub Repository.
///
/// ```yaml
/// resources:
///   exampleImage:
///     type: azure:compute:Image
///     name: example
///     properties:
///       name: exampleimage
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sourceVirtualMachineId: ${example.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:compute:getVirtualMachine
///       arguments:
///         name: examplevm
///         resourceGroupName: example-resources
/// ```
///
/// ## Import
///
/// Images can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/image:Image example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/images/image1
/// ```
///
pub mod image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageArgs {
        /// One or more `data_disk` blocks as defined below.
        #[builder(into, default)]
        pub data_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::ImageDataDisk>>,
        >,
        /// The HyperVGenerationType of the VirtualMachine created from the image as `V1`, `V2`. Defaults to `V1`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `zone_resilient` can only be set to `true` if the image is stored in a region that supports availability zones.
        #[builder(into, default)]
        pub hyper_v_generation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specified the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `os_disk` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub os_disk: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ImageOsDisk>,
        >,
        /// The name of the resource group in which to create the image. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Virtual Machine ID from which to create the image.
        #[builder(into, default)]
        pub source_virtual_machine_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Is zone resiliency enabled? Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub zone_resilient: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ImageResult {
        /// One or more `data_disk` blocks as defined below.
        pub data_disks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::ImageDataDisk>>,
        >,
        /// The HyperVGenerationType of the VirtualMachine created from the image as `V1`, `V2`. Defaults to `V1`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `zone_resilient` can only be set to `true` if the image is stored in a region that supports availability zones.
        pub hyper_v_generation: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specified the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the image. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `os_disk` blocks as defined below. Changing this forces a new resource to be created.
        pub os_disk: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::ImageOsDisk>,
        >,
        /// The name of the resource group in which to create the image. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Virtual Machine ID from which to create the image.
        pub source_virtual_machine_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Is zone resiliency enabled? Defaults to `false`. Changing this forces a new resource to be created.
        pub zone_resilient: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ImageArgs,
    ) -> ImageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let data_disks_binding = args.data_disks.get_output(context).get_inner();
        let hyper_v_generation_binding = args
            .hyper_v_generation
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let os_disk_binding = args.os_disk.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let source_virtual_machine_id_binding = args
            .source_virtual_machine_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let zone_resilient_binding = args.zone_resilient.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/image:Image".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataDisks".into(),
                    value: &data_disks_binding,
                },
                register_interface::ObjectField {
                    name: "hyperVGeneration".into(),
                    value: &hyper_v_generation_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "osDisk".into(),
                    value: &os_disk_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceVirtualMachineId".into(),
                    value: &source_virtual_machine_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zoneResilient".into(),
                    value: &zone_resilient_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ImageResult {
            data_disks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataDisks"),
            ),
            hyper_v_generation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hyperVGeneration"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            os_disk: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("osDisk"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            source_virtual_machine_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceVirtualMachineId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            zone_resilient: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneResilient"),
            ),
        }
    }
}
